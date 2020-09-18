#!/usr/bin/env python

# mypy: allow-any-generics

from __future__ import print_function

import itertools
import os
import re
import sys

from collections import defaultdict, namedtuple, OrderedDict
from contextlib import contextmanager
from typing import (
    Any,
    DefaultDict,
    Dict,
    Iterator,
    List,
    Optional,
    Set,
    Text,
    Tuple,
    Union,
)

import six

from google.protobuf.compiler import plugin_pb2 as plugin
from google.protobuf.descriptor_pb2 import (
    DescriptorProto,
    EnumDescriptorProto,
    EnumValueDescriptorProto,
    FieldDescriptorProto,
    FieldOptions,
    FileDescriptorProto,
    OneofDescriptorProto,
    SourceCodeInfo,
)
from google.protobuf.message import Message

from proto.github.com.gogo.protobuf.gogoproto import gogo_pb2
from proto.rust import extensions_pb2

# Proto type -> (RustType, ImplsEq, ImplsCopy))
PRIMITIVE_TYPES = {
    FieldDescriptorProto.TYPE_FLOAT: ("f32", False, True),
    FieldDescriptorProto.TYPE_DOUBLE: ("f64", False, True),
    FieldDescriptorProto.TYPE_INT32: ("i32", True, True),
    FieldDescriptorProto.TYPE_INT64: ("i64", True, True),
    FieldDescriptorProto.TYPE_UINT32: ("u32", True, True),
    FieldDescriptorProto.TYPE_UINT64: ("u64", True, True),
    FieldDescriptorProto.TYPE_SINT32: ("::pb_jelly::Signed32", True, True),
    FieldDescriptorProto.TYPE_SINT64: ("::pb_jelly::Signed64", True, True),
    FieldDescriptorProto.TYPE_FIXED32: ("::pb_jelly::Fixed32", True, True),
    FieldDescriptorProto.TYPE_FIXED64: ("::pb_jelly::Fixed64", True, True),
    FieldDescriptorProto.TYPE_SFIXED32: ("::pb_jelly::Sfixed32", True, True),
    FieldDescriptorProto.TYPE_SFIXED64: ("::pb_jelly::Sfixed64", True, True),
    FieldDescriptorProto.TYPE_BOOL: ("bool", True, True),
    FieldDescriptorProto.TYPE_STRING: ("::std::string::String", True, False),
    FieldDescriptorProto.TYPE_BYTES: ("::std::vec::Vec<u8>", True, False),
}

BLOB_TYPE = "::pb_jelly::Lazy<::blob_pb::WrappedBlob>"
VEC_SLICE_TYPE = "::pb_jelly::Lazy<::blob_pb::VecSlice>"
LAZY_BYTES_TYPE = "::pb_jelly::Lazy<::bytes::Bytes>"
# pull out `x` from every instance of `::x::y::z`, but not `y` or `z`
CRATE_NAME_REGEX = re.compile(r"(?:^|\W)::(\w+)(?:::\w+)*")

# Keywords in rust which cannot be module names.
RESERVED_KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "Self",
    "self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "alignof",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pure",
    "sizeof",
    "typeof",
    "unsized",
    "virtual",
    "yield",
}


def camelcase(underscored):
    # type: (Text) -> Text
    return "".join(s.capitalize() for s in underscored.split("_"))


class StringIO(object):
    def __init__(self):
        # type: () -> None
        self.contents = []  # type: List[Text]

    def write(self, s):
        # type: (Text) -> None
        self.contents.append(s)

    def getvalue(self):
        # type: () -> Text
        return "".join(self.contents)


class RustType(object):
    def __init__(self, ctx, proto_file, msg_type, field):
        # type: (Context, FileDescriptorProto, DescriptorProto, FieldDescriptorProto) -> None
        self.ctx = ctx
        self.proto_file = proto_file
        self.field = field
        self.is_proto3 = proto_file.syntax == "proto3"
        self.oneof = (
            field.HasField("oneof_index") and msg_type.oneof_decl[field.oneof_index]
        )

    def default(self, msg_name):
        # type: (Text) -> Text
        if self.oneof:
            if oneof_nullable(self.oneof):
                return "None"
            else:
                return self.oneof_val(msg_name, "::std::default::Default::default()")

        # Proto 3 doesn't have configurable default values.
        if not self.is_proto3 and self.field.default_value != "":
            if self.field.type == FieldDescriptorProto.TYPE_STRING:
                return 'Some("%s".into())' % self.field.default_value

            if self.field.type == FieldDescriptorProto.TYPE_BYTES:
                return 'Some(b"%s".to_vec())' % self.field.default_value

            if self.field.type in PRIMITIVE_TYPES:
                typ_name = PRIMITIVE_TYPES[self.field.type][0]
                if "::pb" in typ_name:
                    return "Some(%s(%s))" % (typ_name, self.field.default_value)
                if typ_name.startswith("f") and "." not in self.field.default_value:
                    return "Some(%s.)" % self.field.default_value
                return "Some(%s)" % self.field.default_value

            if self.field.type == FieldDescriptorProto.TYPE_ENUM:
                proto_type = self.ctx.find(self.field.type_name)
                crate, mod_parts = self.ctx.crate_from_proto_filename(
                    self.proto_file.name
                )
                value = (
                    proto_type.rust_name(crate, mod_parts)
                    + "::"
                    + self.field.default_value
                )
                return "Some(%s)" % value

            typ = FieldDescriptorProto.Type.Name(self.field.type)
            raise RuntimeError(
                "Default not supported on field {} of type {!r}".format(
                    self.field.name, typ
                )
            )

        return "::std::default::Default::default()"

    def is_length_delimited(self):
        # type: () -> bool
        length_delimited_types = [
            FieldDescriptorProto.TYPE_MESSAGE,
            FieldDescriptorProto.TYPE_STRING,
            FieldDescriptorProto.TYPE_BYTES,
        ]

        return self.field.type in length_delimited_types

    def wire_format(self):
        # type: () -> Text
        if self.is_length_delimited():
            return "LengthDelimited"

        fixed64_types = [
            FieldDescriptorProto.TYPE_DOUBLE,
            FieldDescriptorProto.TYPE_FIXED64,
            FieldDescriptorProto.TYPE_SFIXED64,
        ]

        fixed32_types = [
            FieldDescriptorProto.TYPE_FLOAT,
            FieldDescriptorProto.TYPE_FIXED32,
            FieldDescriptorProto.TYPE_SFIXED32,
        ]

        if self.field.type in fixed64_types:
            return "Fixed64"

        if self.field.type in fixed32_types:
            return "Fixed32"

        return "Varint"

    def is_grpc_slices(self):
        # type: () -> bool
        return (
            self.field.type == FieldDescriptorProto.TYPE_BYTES
            and self.field.options.Extensions[extensions_pb2.grpc_slices]
        )

    def is_blob(self):
        # type: () -> bool
        return (
            self.field.type == FieldDescriptorProto.TYPE_BYTES
            and self.field.options.ctype == FieldOptions.CORD
        )

    def is_lazy_bytes(self):
        # type: () -> bool
        return (
            self.field.type == FieldDescriptorProto.TYPE_BYTES
            and self.field.options.Extensions[extensions_pb2.zero_copy]
        )

    def is_boxed(self):
        # type: () -> bool
        return (
            self.field.type == FieldDescriptorProto.TYPE_MESSAGE
            and self.field.options.Extensions[extensions_pb2.box_it]
        )

    def has_custom_type(self):
        # type: () -> bool
        return self.field.options.HasExtension(extensions_pb2.type)

    def custom_type(self):
        # type: () -> Text
        return self.field.options.Extensions[extensions_pb2.type]

    def is_nullable(self):
        # type: () -> bool
        if self.field.type in PRIMITIVE_TYPES and self.is_proto3:
            # Primitive types in proto3 are never nullable on the wire - as
            # they cannot actually be represented on the wire as null
            # rather they are represented as 0-value ("" or 0)
            return False
        if self.field.options.HasExtension(gogo_pb2.nullable):
            return self.field.options.Extensions[gogo_pb2.nullable]
        return (
            not self.is_proto3 or self.field.type == FieldDescriptorProto.TYPE_MESSAGE
        )

    def is_empty_oneof_field(self):
        # type: () -> bool
        assert self.oneof
        return self.field.type_name == ".google.protobuf.Empty"

    def can_be_packed(self):
        # type: () -> bool
        # Return true if incoming messages could be packed on the wire
        return self.field.label == FieldDescriptorProto.LABEL_REPEATED and self.wire_format() in (
            "Varint",
            "Fixed64",
            "Fixed32",
        )

    def should_serialize_packed(self):
        # type: () -> bool
        return self.can_be_packed() and (self.field.options.packed or self.is_proto3)

    def is_repeated(self):
        # type: () -> bool
        return self.field.label == FieldDescriptorProto.LABEL_REPEATED

    def set_method(self):
        # type: () -> Tuple[Text, Text]
        assert not self.is_repeated()
        if self.field.type == FieldDescriptorProto.TYPE_FLOAT:
            return "f32", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_DOUBLE:
            return "f64", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_INT32:
            return "i32", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_INT64:
            return "i64", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_UINT32:
            return "u32", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_UINT64:
            return "u64", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_SINT32:
            return "i32", "::pb_jelly::Signed32(v)"
        elif self.field.type == FieldDescriptorProto.TYPE_SINT64:
            return "i64", "::pb_jelly::Signed64(v)"
        elif self.field.type == FieldDescriptorProto.TYPE_FIXED64:
            return "u64", "::pb_jelly::Fixed64(v)"
        elif self.field.type == FieldDescriptorProto.TYPE_SFIXED64:
            return "i64", "::pb_jelly::Sfixed64(v)"
        elif self.field.type == FieldDescriptorProto.TYPE_FIXED32:
            return "u32", "::pb_jelly::Fixed32(v)"
        elif self.field.type == FieldDescriptorProto.TYPE_SFIXED32:
            return "i32", "::pb_jelly::Sfixed32(v)"
        elif self.field.type == FieldDescriptorProto.TYPE_BOOL:
            return "bool", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_STRING:
            return "::std::string::String", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_BYTES:
            if self.is_blob():
                return BLOB_TYPE, "v"
            elif self.is_grpc_slices():
                return VEC_SLICE_TYPE, "v"
            elif self.is_lazy_bytes():
                return LAZY_BYTES_TYPE, "v"
            else:
                return "::std::vec::Vec<u8>", "v"
        elif self.field.type == FieldDescriptorProto.TYPE_ENUM:
            return self.rust_type(), "v"
        elif self.field.type == FieldDescriptorProto.TYPE_MESSAGE:
            if self.is_boxed():
                return "Box<%s>" % self.rust_type(), "v"
            else:
                return self.rust_type(), "v"
        raise AssertionError("Unexpected field type")

    def take_method(self):
        # type: () -> Tuple[Optional[Text], Optional[Text]]
        assert not self.is_repeated()
        has_take_method = [
            FieldDescriptorProto.TYPE_STRING,
            FieldDescriptorProto.TYPE_BYTES,
            FieldDescriptorProto.TYPE_MESSAGE,
        ]

        if not self.field.type in has_take_method:
            return None, None

        expr = "self.%s.take().unwrap_or_default()" % self.field.name

        if self.field.type == FieldDescriptorProto.TYPE_STRING:
            return "::std::string::String", expr
        elif self.field.type == FieldDescriptorProto.TYPE_BYTES:
            if self.is_blob():
                return BLOB_TYPE, expr
            elif self.is_grpc_slices():
                return VEC_SLICE_TYPE, expr
            elif self.is_lazy_bytes():
                return LAZY_BYTES_TYPE, expr
            else:
                return "::std::vec::Vec<u8>", expr
        elif self.field.type == FieldDescriptorProto.TYPE_ENUM:
            return self.rust_type(), expr
        elif self.field.type == FieldDescriptorProto.TYPE_MESSAGE:
            if self.is_boxed():
                return "Box<%s>" % self.rust_type(), expr
            else:
                return self.rust_type(), expr
        raise AssertionError("Unexpected field type")

    def get_method(self):
        # type: () -> Tuple[Text, Text]
        assert not self.is_repeated()
        name = self.field.name

        if self.field.type == FieldDescriptorProto.TYPE_FLOAT:
            return "f32", "self.%s.unwrap_or(0.)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_DOUBLE:
            return "f64", "self.%s.unwrap_or(0.)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_INT32:
            return "i32", "self.%s.unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_INT64:
            return "i64", "self.%s.unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_UINT32:
            return "u32", "self.%s.unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_UINT64:
            return "u64", "self.%s.unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_SINT32:
            return "i32", "self.%s.map(|v| v.0).unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_SINT64:
            return "i64", "self.%s.map(|v| v.0).unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_FIXED64:
            return "u64", "self.%s.map(|v| v.0).unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_SFIXED64:
            return "i64", "self.%s.map(|v| v.0).unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_FIXED32:
            return "u32", "self.%s.map(|v| v.0).unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_SFIXED32:
            return "i32", "self.%s.map(|v| v.0).unwrap_or(0)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_BOOL:
            return "bool", "self.%s.unwrap_or(false)" % name
        elif self.field.type == FieldDescriptorProto.TYPE_STRING:
            return (
                "&str",
                'self.%s.as_ref().map(|ref s| s.as_str()).unwrap_or("")' % name,
            )
        elif self.field.type == FieldDescriptorProto.TYPE_BYTES:
            assert not (
                self.is_blob() or self.is_grpc_slices() or self.is_lazy_bytes()
            ), "Can't generate get method for lazy field"
            return "&[u8]", "self.%s.as_ref().map(|v| &**v).unwrap_or(&[])" % name
        elif self.field.type == FieldDescriptorProto.TYPE_ENUM:
            return self.rust_type(), "self.%s.unwrap_or_default()" % name
        elif self.field.type == FieldDescriptorProto.TYPE_MESSAGE:
            deref = (
                "" if not self.is_boxed() else ".map(|v| ::std::ops::Deref::deref(v))"
            )
            return (
                "&" + self.rust_type(),
                "self.%s.as_ref()%s.unwrap_or(&%s_default)"
                % (name, deref, self.rust_type()),
            )
        raise AssertionError("Unexpected field type")

    def rust_type(self):
        # type: () -> Text
        typ = self.field.type

        if self.has_custom_type():
            return self.custom_type()

        if self.is_blob():
            return BLOB_TYPE

        if self.is_grpc_slices():
            return VEC_SLICE_TYPE

        if self.is_lazy_bytes():
            return LAZY_BYTES_TYPE

        if typ in PRIMITIVE_TYPES:
            return PRIMITIVE_TYPES[typ][0]

        if (
            typ == FieldDescriptorProto.TYPE_MESSAGE
            or typ == FieldDescriptorProto.TYPE_ENUM
        ):
            if self.field.type_name[0] != ".":
                raise RuntimeError("We only support fully qualified type names")

            proto_type = self.ctx.find(self.field.type_name)
            crate, mod_parts = self.ctx.crate_from_proto_filename(self.proto_file.name)
            return proto_type.rust_name(crate, mod_parts)

        raise RuntimeError(
            "Unsupported type: {!r}".format(FieldDescriptorProto.Type.Name(typ))
        )

    def __str__(self):
        # type: () -> str
        rust_type = self.rust_type()

        if self.is_repeated():
            return "::std::vec::Vec<%s>" % rust_type  # type: ignore[return-value]
        elif self.is_nullable() and self.is_boxed():
            return "::std::option::Option<::std::boxed::Box<%s>>" % str(rust_type)
        elif self.is_boxed():
            return "::std::boxed::Box<%s>" % rust_type  # type: ignore[return-value]
        elif self.is_nullable():
            return "::std::option::Option<%s>" % rust_type  # type: ignore[return-value]
        else:
            return str(rust_type)

    def oneof_field_match(self, var):
        # type: (Text) -> Text
        if self.is_empty_oneof_field():
            return camelcase(self.field.name)
        else:
            return "%s(%s)" % (camelcase(self.field.name), var)

    def oneof_val(self, msg_name, var):
        # type: (Text, Text) -> Text
        assert self.oneof
        oneofv = "%s::%s" % (
            oneof_msg_name(msg_name, self.oneof),
            self.oneof_field_match(var),
        )

        if oneof_nullable(self.oneof):
            oneofv = "Some(%s)" % oneofv

        return oneofv


def oneof_msg_name(parent_msg_name, oneof):
    # type: (Text, OneofDescriptorProto) -> Text
    return "%s_%s" % (parent_msg_name, camelcase(oneof.name))


def oneof_nullable(oneof):
    # type: (OneofDescriptorProto) -> bool
    return (
        not oneof.options.HasExtension(extensions_pb2.nullable)
        or oneof.options.Extensions[extensions_pb2.nullable]
    )


def enum_err_if_default_or_unknown(enum):
    # type: (EnumDescriptorProto) -> bool
    return (
        enum.options.HasExtension(extensions_pb2.err_if_default_or_unknown)
        and enum.options.Extensions[extensions_pb2.err_if_default_or_unknown]
    )


@contextmanager
def block(ctx, header):
    # type: (CodeWriter, Text) -> Iterator[None]
    ctx.write("%s {" % header)
    ctx.indentation += 1
    yield
    ctx.indentation -= 1
    ctx.write("}")


@contextmanager
def field_iter(ctx, var, msg_name, msg_type, field):
    # type: (CodeWriter, Text, Text, DescriptorProto, FieldDescriptorProto) -> Iterator[None]
    typ = ctx.rust_type(msg_type, field)

    if typ.oneof:
        # For oneofs, we always emit, even if the primitive field is set to a 0 value
        # This is so we can distinguish which field of oneof is set.
        with block(
            ctx,
            "if let %s = self.%s"
            % (typ.oneof_val(msg_name, "ref " + var), typ.oneof.name),
        ):
            if typ.is_empty_oneof_field():
                ctx.write(
                    "let %s: &%s = &::std::default::Default::default();"
                    % (var, typ.rust_type())
                )
            yield
    elif (
        field.type == FieldDescriptorProto.TYPE_MESSAGE
        and not typ.is_nullable()
        and not typ.is_repeated()
    ):
        # Always emit messages explicitly marked as non-nullable
        with block(ctx, ""):
            ctx.write("let %s = &self.%s;" % (var, field.name))
            yield
    elif (
        field.type == FieldDescriptorProto.TYPE_ENUM
        and not typ.is_repeated()
        and enum_err_if_default_or_unknown(ctx.ctx.find(field.type_name).typ)
    ):
        # The default value (as considered by proto) doesn't appear in the generated enum and
        # doesn't map to .default(). All of the values that actually get generated need to get
        # encoded.
        ctx.write("let %s = &self.%s;" % (var, field.name))
        yield
    elif not typ.is_nullable() and not typ.is_repeated():
        # For proto3, we remove the Option for primitive fields.
        # We only run internal code if the primitive field is non-default for proto3
        # Rather than looping, we set the variable once and run the inner code once
        with block(
            ctx,
            "if self.%s != <%s as ::std::default::Default>::default()"
            % (field.name, typ),
        ):
            if typ.is_boxed():
                ctx.write("let %s = &*self.%s;" % (var, field.name))
            else:
                ctx.write("let %s = &self.%s;" % (var, field.name))
            yield
    else:
        # This iterates through Vec and the Option<> type for optional fieldds
        with block(ctx, "for %s in &self.%s" % (var, field.name)):
            if typ.is_boxed():
                ctx.write("let %s = &**%s;" % (var, var))
            yield


class CodeWriter(object):
    def __init__(self, ctx, proto_file, crate, mod_parts):
        # type: (Context, FileDescriptorProto, Text, List[Text]) -> None
        self.ctx = ctx
        self.proto_file = proto_file
        self.crate = crate
        self.mod_parts = mod_parts
        self.indentation = 0
        self.content = StringIO()
        self.is_proto3 = proto_file and proto_file.syntax == "proto3"
        if proto_file and proto_file.options.Extensions[extensions_pb2.serde_derive]:
            self.derive_serde = True
        else:
            self.derive_serde = False

        # See https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto#L727
        # for context on this beast
        if proto_file is not None:
            self.source_code_info_by_scl = {
                tuple(location.path): location
                for location in proto_file.source_code_info.location
            }

    def write(self, s):
        # type: (Text) -> None
        if s == "":
            self.content.write("\n")
            return

        for _ in range(self.indentation):
            self.content.write("  ")
        self.content.write(s)
        self.content.write("\n")

    def write_line_broken_text_with_prefix(self, text_block, prefix):
        # type: (Text, Text) -> None
        if not text_block:
            return
        for l in text_block.rstrip().split("\n"):
            if l:
                self.write(u"{}{}".format(prefix, l))
            else:
                self.write("")

    def write_comments(self, sci_loc):
        # type: (Optional[SourceCodeInfo.Location]) -> None
        if sci_loc is None:
            return
        for leading_detached_comment in sci_loc.leading_detached_comments:
            self.write_line_broken_text_with_prefix(leading_detached_comment, "//")
            self.write("")
        if sci_loc.leading_comments is not None:
            self.write_line_broken_text_with_prefix(sci_loc.leading_comments, "///")
        # Trailing comments also go in the header - to make sure it gets into the docstring
        if sci_loc.trailing_comments is not None:
            self.write_line_broken_text_with_prefix(sci_loc.trailing_comments, "///")

    def rust_type(self, msg_type, field):
        # type: (DescriptorProto, FieldDescriptorProto) -> RustType
        return RustType(self.ctx, self.proto_file, msg_type, field)

    def gen_closed_enum(self, name, enum_variants, scl):
        # type: (Text, List[Tuple[int, EnumValueDescriptorProto]], SourceCodeLocation) -> None

        # Generate a closed enum
        self.write_comments(self.source_code_info_by_scl.get(tuple(scl)))
        if self.derive_serde:
            self.write(
                "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Deserialize, Serialize)]"
            )
        else:
            self.write(
                "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]"
            )
        self.write("#[repr(i32)]")
        with block(self, "pub enum " + name):
            for idx, value in enum_variants:
                vfn = EnumDescriptorProto.VALUE_FIELD_NUMBER  # type: ignore
                self.write_comments(
                    self.source_code_info_by_scl.get(tuple(scl + [vfn, idx]))
                )
                self.write("%s = %s," % (value.name, value.number))

        with block(self, "impl " + name):
            self.write(
                "pub const KNOWN_VARIANTS: [%s; %s] = [%s];"
                % (
                    name,
                    len(enum_variants),
                    ", ".join(
                        "%s::%s" % (name, value.name) for _, value in enum_variants
                    ),
                )
            )

        with block(self, "impl ::std::default::Default for " + name):
            with block(self, "fn default() -> Self"):
                # It's not actually clear what to do in this case. We take the first-defined
                # value that isn't 0-valued.
                for _, value in enum_variants:
                    self.write("%s::%s" % (name, value.name))
                    break

        with block(self, "impl From<%s> for i32" % name):
            with block(self, "fn from(v: %s) -> i32" % name):
                with block(self, "match v"):
                    for _, value in enum_variants:
                        self.write("%s::%s => %s," % (name, value.name, value.number))

        with block(self, "impl ::std::convert::TryFrom<i32> for %s" % name):
            self.write("type Error = i32;")
            with block(self, "fn try_from(v: i32) -> ::std::result::Result<Self, i32>"):
                with block(self, "match v"):
                    for _, value in enum_variants:
                        self.write(
                            "%s => Ok(%s::%s)," % (value.number, name, value.name)
                        )
                    self.write("_ => Err(v),")

        with block(self, "impl ::pb_jelly::ProtoEnum for " + name):
            pass

        with block(self, "impl ::pb_jelly::ClosedProtoEnum for " + name):
            with block(self, "fn name(self) -> &'static str"):
                with block(self, "match self"):
                    for _, value in enum_variants:
                        self.write('%s::%s => "%s",' % (name, value.name, value.name))

    def gen_open_enum(self, name, enum_variants, scl):
        # type: (Text, List[Tuple[int, EnumValueDescriptorProto]], SourceCodeLocation) -> None

        closed_name = name + "_Closed"

        # Generate an open enum
        self.write_comments(self.source_code_info_by_scl.get(tuple(scl)))
        if self.derive_serde:
            self.write(
                "#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]"
            )
        else:
            self.write("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]")
        self.write("#[repr(transparent)]")
        self.write("pub struct %s(i32);" % name)
        with block(self, "impl " + name):
            for idx, value in enum_variants:
                vfn = EnumDescriptorProto.VALUE_FIELD_NUMBER  # type: ignore
                self.write_comments(
                    self.source_code_info_by_scl.get(tuple(scl + [vfn, idx]))
                )
                self.write(
                    "pub const %s: %s = %s(%s);"
                    % (value.name, name, name, value.number)
                )
            self.write(
                "pub const KNOWN_VARIANTS: [%s; %s] = [%s];"
                % (
                    name,
                    len(enum_variants),
                    ", ".join(
                        "%s::%s" % (name, value.name) for _, value in enum_variants
                    ),
                )
            )

            with block(self, "pub const fn value(self) -> i32"):
                self.write("self.0")

            with block(
                self,
                "pub fn into_known(self) -> ::std::option::Option<%s>" % closed_name,
            ):
                with block(self, "match self"):
                    for _, value in enum_variants:
                        self.write(
                            "%s::%s => Some(%s::%s),"
                            % (name, value.name, closed_name, value.name)
                        )
                    self.write("_ => None,")

        with block(self, "impl ::std::default::Default for " + name):
            with block(self, "fn default() -> Self"):
                # Under proto2, the default value is the first defined.
                # Under proto3, the default value is the 0-valued variant, but it's required to
                # be defined first.
                self.write("%s::%s" % (name, enum_variants[0][1].name))

        with block(self, "impl From<%s> for i32" % name):
            with block(self, "fn from(v: %s) -> i32" % name):
                self.write("v.0")

        with block(self, "impl From<i32> for %s" % name):
            with block(self, "fn from(v: i32) -> %s" % name):
                self.write("%s(v)" % name)

        with block(self, "impl From<%s> for %s" % (closed_name, name)):
            with block(self, "fn from(v: %s) -> %s" % (closed_name, name)):
                self.write("%s(v as i32)" % name)

        with block(self, "impl ::pb_jelly::ProtoEnum for " + name):
            pass

        with block(self, "impl ::pb_jelly::OpenProtoEnum for " + name):
            with block(self, "fn name(self) -> ::std::option::Option<&'static str>"):
                with block(self, "match self"):
                    for _, value in enum_variants:
                        self.write(
                            '%s::%s => Some("%s"),' % (name, value.name, value.name)
                        )
                    self.write("_ => None,")

            with block(self, "fn is_known(self) -> bool"):
                with block(self, "match self"):
                    for _, value in enum_variants:
                        self.write("%s::%s => true," % (name, value.name))
                    self.write("_ => false,")

        with block(self, "impl ::std::fmt::Debug for " + name):
            with block(
                self,
                "fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result",
            ):
                self.write("use ::pb_jelly::OpenProtoEnum;")
                with block(self, "match self.name()"):
                    self.write('Some(s) => write!(f, "{}", s),')
                    self.write('None => write!(f, "Unknown({})", self.0),')

        self.gen_closed_enum(closed_name, enum_variants, scl)

    def gen_enum(self, path, enum_type, scl):
        # type: (List[Text], EnumDescriptorProto, SourceCodeLocation) -> None
        assert self.indentation == 0
        name = "_".join(path + [enum_type.name])
        if enum_err_if_default_or_unknown(enum_type):
            self.gen_closed_enum(
                name, [x for x in enumerate(enum_type.value) if x[1].number != 0], scl
            )
        else:
            self.gen_open_enum(name, list(enumerate(enum_type.value)), scl)

    def gen_msg(self, path, msg_type, scl):
        # type: (List[Text], DescriptorProto, SourceCodeLocation) -> None
        assert self.indentation == 0
        name = "_".join(path + [msg_type.name])

        # Adjust some field names
        for field in msg_type.field:
            if field.name in RESERVED_KEYWORDS:
                field.name = field.name + "_"
        for oneof in msg_type.oneof_decl:
            if oneof.name in RESERVED_KEYWORDS:
                oneof.name = oneof.name + "_"

        oneof_fields = defaultdict(
            list
        )  # type: DefaultDict[Text, List[FieldDescriptorProto]]

        derives = ["Clone", "Debug", "PartialEq"]
        if self.derive_serde:
            derives.extend(["Deserialize", "Serialize"])

        fq_msg = (self.proto_file.name, name)
        if self.ctx.impls_by_msg[fq_msg].Eq:
            derives.extend(["Eq", "PartialOrd", "Ord", "Hash"])
        if self.ctx.impls_by_msg[fq_msg].Copy:
            derives.append("Copy")

        self.write_comments(self.source_code_info_by_scl.get(tuple(scl)))

        self.write("#[derive(%s)]" % ", ".join(sorted(derives)))
        with block(self, "pub struct " + name):
            for idx, field in enumerate(msg_type.field):
                ffn = DescriptorProto.FIELD_FIELD_NUMBER  # type: ignore
                self.write_comments(
                    self.source_code_info_by_scl.get(tuple(scl + [ffn, idx]))
                )

                typ = self.rust_type(msg_type, field)
                if typ.oneof:
                    oneof_fields[typ.oneof.name].append(field)
                else:
                    self.write("pub %s: %s," % (field.name, typ))

            for oneof in msg_type.oneof_decl:
                if oneof_nullable(oneof):
                    self.write(
                        "pub %s: ::std::option::Option<%s>,"
                        % (oneof.name, oneof_msg_name(name, oneof))
                    )
                else:
                    self.write(
                        "pub %s: %s," % (oneof.name, oneof_msg_name(name, oneof))
                    )

            if msg_type.options.Extensions[extensions_pb2.preserve_unrecognized]:
                self.write("pub _unrecognized: Vec<u8>,")

        # Generate any oneof enum structs
        for oneof in msg_type.oneof_decl:
            self.write("#[derive(%s)]" % ", ".join(sorted(derives)))
            with block(self, "pub enum " + oneof_msg_name(name, oneof)):
                for oneof_field in oneof_fields[oneof.name]:
                    typ = self.rust_type(msg_type, oneof_field)
                    self.write("%s," % typ.oneof_field_match(typ.rust_type()))

        if not self.is_proto3:
            with block(self, "impl " + name):
                for field in msg_type.field:
                    typ = self.rust_type(msg_type, field)
                    if typ.oneof:
                        continue
                    if typ.is_repeated():
                        with block(
                            self,
                            "pub fn set_%s(&mut self, v: ::std::vec::Vec<%s>)"
                            % (field.name, typ.rust_type()),
                        ):
                            self.write("self.%s = v;" % field.name)

                        with block(
                            self,
                            "pub fn take_%s(&mut self) -> ::std::vec::Vec<%s>"
                            % (field.name, typ.rust_type()),
                        ):
                            self.write(
                                "::std::mem::replace(&mut self.%s, ::std::vec::Vec::new())"
                                % field.name
                            )

                        with block(
                            self,
                            "pub fn get_%s(&self) -> &[%s]"
                            % (field.name, typ.rust_type()),
                        ):
                            self.write("&self.%s" % field.name)

                        with block(
                            self,
                            "pub fn mut_%s(&mut self) -> &mut ::std::vec::Vec<%s>"
                            % (field.name, typ.rust_type()),
                        ):
                            self.write("&mut self." + field.name)

                    elif typ.is_nullable():
                        with block(self, "pub fn has_%s(&self) -> bool" % field.name):
                            self.write("self.%s.is_some()" % field.name)

                        input_type, input_expr = typ.set_method()
                        with block(
                            self,
                            "pub fn set_%s(&mut self, v: %s)"
                            % (field.name, input_type),
                        ):
                            self.write("self.%s = Some(%s);" % (field.name, input_expr))

                        return_type, return_expr = typ.take_method()
                        if return_type is not None and return_expr is not None:
                            with block(
                                self,
                                "pub fn take_%s(&mut self) -> %s"
                                % (field.name, return_type),
                            ):
                                self.write(return_expr)

                        if not (typ.is_blob() or typ.is_grpc_slices() or typ.is_lazy_bytes()):
                            # It's hard to make this make sense, so let's not generate `get` method for lazy things.
                            return_type, return_expr = typ.get_method()
                            with block(
                                self,
                                "pub fn get_%s(&self) -> %s"
                                % (field.name, return_type),
                            ):
                                self.write(return_expr)

        with block(self, "impl ::std::default::Default for " + name):
            with block(self, "fn default() -> Self"):
                with block(self, name):
                    for field in msg_type.field:
                        typ = self.rust_type(msg_type, field)
                        if not typ.oneof:
                            self.write("%s: %s," % (field.name, typ.default(name)))
                    for oneof in msg_type.oneof_decl:
                        oneof_field = oneof_fields[oneof.name][0]
                        typ = self.rust_type(msg_type, oneof_field)
                        self.write("%s: %s," % (oneof.name, typ.default(name)))
                    if msg_type.options.Extensions[
                        extensions_pb2.preserve_unrecognized
                    ]:
                        self.write("_unrecognized: Vec::new(),")

        with block(self, "lazy_static!"):
            self.write(
                "pub static ref %s_default: %s = %s::default();" % (name, name, name)
            )

        with block(self, "impl ::pb_jelly::Message for " + name):
            with block(self, "fn compute_size(&self) -> usize "):
                if (
                    len(msg_type.field) > 0
                    or msg_type.options.Extensions[extensions_pb2.preserve_unrecognized]
                ):
                    self.write("let mut size = 0;")
                    for field in msg_type.field:
                        typ = self.rust_type(msg_type, field)

                        self.write("let mut %s_size = 0;" % field.name)
                        with field_iter(self, "val", name, msg_type, field):
                            self.write("let l = ::pb_jelly::Message::compute_size(val);")
                            if not typ.should_serialize_packed():
                                self.write(
                                    "%s_size += ::pb_jelly::wire_format::serialized_length(%s);"
                                    % (field.name, field.number)
                                )
                            if typ.is_length_delimited():
                                self.write(
                                    "%s_size += ::pb_jelly::varint::serialized_length(l as u64);"
                                    % field.name
                                )
                            self.write("%s_size += l;" % field.name)
                        if typ.should_serialize_packed():
                            with block(self, "if !self.%s.is_empty()" % field.name):
                                self.write(
                                    "size += ::pb_jelly::wire_format::serialized_length(%s);"
                                    % field.number
                                )
                                self.write(
                                    "size += ::pb_jelly::varint::serialized_length(%s_size as u64);"
                                    % field.name
                                )
                        self.write("size += %s_size;" % field.name)
                    if msg_type.options.Extensions[
                        extensions_pb2.preserve_unrecognized
                    ]:
                        self.write("size += self._unrecognized.len();")
                    self.write("size")
                else:
                    self.write("0")

            with block(self, "fn compute_grpc_slices_size(&self) -> usize "):
                if len(msg_type.field) > 0:
                    self.write("let mut size = 0;")
                    for field in msg_type.field:
                        with field_iter(self, "val", name, msg_type, field):
                            self.write(
                                "size += ::pb_jelly::Message::compute_grpc_slices_size(val);"
                            )
                    self.write("size")
                else:
                    self.write("0")

            with block(
                self,
                "fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()>",
            ):
                for field in sorted(msg_type.field, key=lambda f: f.number):
                    typ = self.rust_type(msg_type, field)
                    # In proto2, this ensures we don't emit fields set to None
                    # In proto3, this ensures we don't emit fields set to their default value
                    if typ.should_serialize_packed():
                        with block(self, "if !self.%s.is_empty()" % field.name):
                            self.write("let mut size = 0;")
                            with field_iter(self, "val", name, msg_type, field):
                                self.write("size += ::pb_jelly::Message::compute_size(val);")
                            self.write(
                                "::pb_jelly::wire_format::write(%s, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;"
                                % field.number
                            )
                            self.write("::pb_jelly::varint::write(size as u64, w)?;")

                    with field_iter(self, "val", name, msg_type, field):
                        if not typ.should_serialize_packed():
                            self.write(
                                "::pb_jelly::wire_format::write(%s, ::pb_jelly::wire_format::Type::%s, w)?;"
                                % (field.number, typ.wire_format())
                            )
                        if typ.is_length_delimited():
                            self.write("let l = ::pb_jelly::Message::compute_size(val);")
                            self.write("::pb_jelly::varint::write(l as u64, w)?;")
                        self.write("::pb_jelly::Message::serialize(val, w)?;")
                if msg_type.options.Extensions[extensions_pb2.preserve_unrecognized]:
                    self.write("w.write_all(&self._unrecognized)?;")
                self.write("Ok(())")

            with block(
                self,
                "fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()>",
            ):
                preserve_unrecognized = msg_type.options.Extensions[
                    extensions_pb2.preserve_unrecognized
                ]
                if preserve_unrecognized:
                    self.write("let mut unrecognized = ::pb_jelly::Unrecognized::default();")

                for oneof in msg_type.oneof_decl:
                    if not oneof_nullable(oneof):
                        oneof_typ = oneof_msg_name(name, oneof)
                        self.write(
                            "let mut oneof_%s: ::std::option::Option<%s> = None;"
                            % (oneof.name, oneof_typ)
                        )
                err_if_default_field_names = (
                    OrderedDict()
                )  # type: OrderedDict[Text, None]
                for field in msg_type.field:
                    if (
                        field.type == FieldDescriptorProto.TYPE_ENUM
                        and not typ.is_repeated()
                    ):
                        enum_type = self.ctx.find(field.type_name).typ
                        if enum_err_if_default_or_unknown(enum_type) and not typ.oneof:
                            typ = self.rust_type(msg_type, field)
                            self.write(
                                "let mut %s: ::std::option::Option<%s> = None;"
                                % (field.name, typ.rust_type())
                            )
                            err_if_default_field_names[field.name] = None

                with block(
                    self,
                    "while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)?",
                ):
                    with block(self, "match field_number"):
                        for field in msg_type.field:
                            typ = self.rust_type(msg_type, field)
                            with block(self, "%s =>" % field.number):
                                if typ.can_be_packed():
                                    with block(self, "match typ"):
                                        with block(
                                            self,
                                            "::pb_jelly::wire_format::Type::LengthDelimited =>",
                                        ):
                                            self.write(
                                                "let len = ::pb_jelly::varint::ensure_read(&mut buf)?;"
                                            )
                                            self.write(
                                                "let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;"
                                            )
                                            with block(
                                                self, "while vals.has_remaining()"
                                            ):
                                                self.write(
                                                    "let mut val: %s = ::std::default::Default::default();"
                                                    % typ.rust_type()
                                                )
                                                self.write(
                                                    "::pb_jelly::Message::deserialize(&mut val, &mut vals)?;"
                                                )
                                                self.write(
                                                    "self.%s.push(val);" % field.name
                                                )
                                        with block(self, "_ =>"):
                                            self.write(
                                                '::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::%s, "%s", %s)?;'
                                                % (
                                                    typ.wire_format(),
                                                    name,
                                                    field.number,
                                                )
                                            )
                                            self.write(
                                                "let mut val: %s = ::std::default::Default::default();"
                                                % typ.rust_type()
                                            )
                                            self.write(
                                                "::pb_jelly::Message::deserialize(&mut val, buf)?;"
                                            )
                                            self.write(
                                                "self.%s.push(val);" % field.name
                                            )
                                else:
                                    if typ.is_length_delimited():
                                        self.write(
                                            '::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "%s", %s)?;'
                                            % (name, field.number)
                                        )
                                        self.write(
                                            "let len = ::pb_jelly::varint::ensure_read(&mut buf)?;"
                                        )
                                        self.write(
                                            "let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;"
                                        )
                                        self.write(
                                            "let mut val: %s = ::std::default::Default::default();"
                                            % typ.rust_type()
                                        )
                                        self.write(
                                            "::pb_jelly::Message::deserialize(&mut val, &mut next)?;"
                                        )
                                    else:
                                        self.write(
                                            '::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::%s, "%s", %s)?;'
                                            % (typ.wire_format(), name, field.number)
                                        )
                                        self.write(
                                            "let mut val: %s = ::std::default::Default::default();"
                                            % typ.rust_type()
                                        )
                                        self.write(
                                            "::pb_jelly::Message::deserialize(&mut val, buf)?;"
                                        )

                                    if typ.is_repeated():
                                        self.write("self.%s.push(val);" % field.name)
                                    else:
                                        field_val = (
                                            "Box::new(val)" if typ.is_boxed() else "val"
                                        )

                                        if typ.oneof:
                                            if oneof_nullable(typ.oneof):
                                                self.write(
                                                    "self.%s = %s;"
                                                    % (
                                                        typ.oneof.name,
                                                        typ.oneof_val(name, field_val),
                                                    )
                                                )
                                            else:
                                                self.write(
                                                    "oneof_%s = Some(%s);"
                                                    % (
                                                        typ.oneof.name,
                                                        typ.oneof_val(name, field_val),
                                                    )
                                                )
                                        elif typ.is_nullable():
                                            self.write(
                                                "self.%s = Some(%s);"
                                                % (field.name, field_val)
                                            )
                                        elif field.name in err_if_default_field_names:
                                            self.write(
                                                "%s = Some(%s);"
                                                % (field.name, field_val)
                                            )
                                        else:
                                            self.write(
                                                "self.%s = %s;"
                                                % (field.name, field_val)
                                            )
                        with block(self, "_ =>"):
                            if preserve_unrecognized:
                                self.write(
                                    "unrecognized.gather(field_number, typ, &mut buf)?;"
                                )
                            else:
                                self.write("::pb_jelly::skip(typ, &mut buf)?;")
                for oneof in msg_type.oneof_decl:
                    if not oneof_nullable(oneof):
                        with block(self, "match oneof_" + oneof.name):
                            self.write("Some(v) => self.%s = v," % oneof.name)
                            self.write(
                                "None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, \"missing value for non-nullable oneof '%s' while parsing message %s.%s\")),"
                                % (oneof.name, self.proto_file.package, msg_type.name)
                            )

                for field_name in err_if_default_field_names:
                    with block(self, "match " + field_name):
                        self.write("Some(v) => self.%s = v," % field_name)
                        self.write(
                            "None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, \"err_if_default_or_unknown '%s' had no value while parsing message %s.%s\")),"
                            % (field_name, self.proto_file.package, msg_type.name)
                        )

                if preserve_unrecognized:
                    self.write("unrecognized.serialize(&mut self._unrecognized)?;")
                self.write("Ok(())")

    def gen_msg_descriptor(self, path, desc_proto, package, scl):
        # type: (List[Text], DescriptorProto, Optional[Text], SourceCodeLocation) -> None
        assert self.indentation == 0

        name = "_".join(path + [desc_proto.name])
        full_name = ".".join([package, name]) if package else name

        with block(self, "impl ::pb_jelly::MessageDescriptor for " + name):
            self.write('const NAME: &\'static str = "%s";' % name)
            self.write('const FULL_NAME: &\'static str = "%s";' % full_name)


# SourceCodeLocation is defined by `message Location` here
# https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto
SourceCodeLocation = List[int]
ProtoTypes = Union[FileDescriptorProto, EnumDescriptorProto, DescriptorProto]
WalkRet = Tuple[
    List[Tuple[List[Text], EnumDescriptorProto, SourceCodeLocation]],
    List[Tuple[List[Text], DescriptorProto, SourceCodeLocation]],
]


def walk(proto):
    # type: (FileDescriptorProto) -> WalkRet
    enums, messages = [], []

    def _walk(proto, parents, scl_prefix):
        # type: (ProtoTypes, List[Text], SourceCodeLocation) -> None
        if isinstance(proto, EnumDescriptorProto):
            enums.append((parents, proto, scl_prefix))
        elif isinstance(proto, DescriptorProto):
            messages.append((parents, proto, scl_prefix))

            for i, nested_enum in enumerate(proto.enum_type):
                etfn = DescriptorProto.ENUM_TYPE_FIELD_NUMBER  # type: ignore
                _walk(nested_enum, parents + [proto.name], scl_prefix + [etfn, i])

            for i, nested_message in enumerate(proto.nested_type):
                ntfn = DescriptorProto.NESTED_TYPE_FIELD_NUMBER  # type: ignore
                _walk(nested_message, parents + [proto.name], scl_prefix + [ntfn, i])
        elif isinstance(proto, FileDescriptorProto):
            for i, enum_type in enumerate(proto.enum_type):
                etfn = FileDescriptorProto.ENUM_TYPE_FIELD_NUMBER  # type: ignore
                _walk(enum_type, parents, scl_prefix + [etfn, i])

            for i, message_type in enumerate(proto.message_type):
                mtfn = FileDescriptorProto.MESSAGE_TYPE_FIELD_NUMBER  # type: ignore
                _walk(message_type, parents, scl_prefix + [mtfn, i])

    _walk(proto, [], [])
    return enums, messages


class ProtoType(object):
    def __init__(self, ctx, proto_file, path, typ):
        # type: (Context, FileDescriptorProto, List[Text], Message) -> None
        self.ctx = ctx
        self.proto_file = proto_file
        self.path = path  # inside proto file
        self.typ = typ  # type: Any
        self.crate, self.mod_parts = ctx.crate_from_proto_filename(proto_file.name)

    def __repr__(self):
        # type: () -> str
        return "{} {} {} {}".format(
            self.proto_file.package, self.proto_file.name, self.path, self.typ.name
        )

    def proto_name(self):
        # type: () -> Text
        """Name as used by proto eg. .mp.BlockMetadata.CompressionFormat"""
        r = "." + ".".join(self.path + [self.typ.name])
        if self.proto_file.package != "":
            r = "." + self.proto_file.package + r
        return r

    def rust_name(self, other_crate, other_mod_parts):
        # type: (Text, List[Text]) -> Text
        """Name as used in rust code for proto_file"""
        if self.ctx.crate_from_proto_filename(self.proto_file.name) == (
            other_crate,
            other_mod_parts,
        ):
            # In same rust binary, directly use typename
            return "_".join(self.path + [self.typ.name])

        mod_parts = self.mod_parts + ["_".join(self.path + [self.typ.name])]
        if other_crate != self.crate:
            # Different crate. Insert crate name in fully qualified module.
            mod_parts.insert(0, "::" + self.crate)
        else:
            # Same crate. Use super::<module_name>::<local_type>
            num_supers = len(other_mod_parts)
            supers = "::".join(num_supers * ["super"])
            mod_parts.insert(0, supers)
        return "::".join(mod_parts)


Impls = namedtuple("Impls", ["Eq", "Copy"])


class Context(object):
    def __init__(self, crate_per_dir, prefix_to_clear):
        # type: (bool, Text) -> None
        self.proto_types = {}  # type: Dict[Text, ProtoType]

        # Set iteration order is nondeterministic, but this is ok, because we can
        # emit crates in any order
        self.deps_map = defaultdict(set)  # type: DefaultDict[Text, Set[Text]]
        self.extra_crates = defaultdict(set)  # type: DefaultDict[Text, Set[Text]]

        # Map from msg.type_name => whether it impls Eq / Copy
        # We have to build this on the fly as we process the types.
        self.impls_by_msg = {}  # type: Dict[Tuple[Text, Text], Impls]

        # Indiciator if every directory should be their own crate.
        self.crate_per_dir = crate_per_dir

        # Prefix of the crate path which should be cleared before making a determination
        # of how to split the crates apart.
        self.prefix_to_clear = prefix_to_clear

    def calc_impls(self, proto_file, crate, msg_type, fq_msg):
        # type: (FileDescriptorProto, Text, DescriptorProto, Tuple[Text, Text]) -> None
        # Determine if each message can implement Eq / Ord Copy traits. Using Dynamic Programming.
        # See if it's cached
        if fq_msg in self.impls_by_msg:
            return

        # Place a false in the cache to prevent recursion loops
        self.impls_by_msg[fq_msg] = Impls(Eq=False, Copy=False)

        # Generate whether or not this msg implements Eq / Ord
        (msg_impls_eq, msg_impls_copy) = (True, True)

        if msg_type.options.Extensions[extensions_pb2.preserve_unrecognized]:
            msg_impls_copy = False  # Preserve unparsed has a Vec which is not Copy

        for field in msg_type.field:
            typ = field.type
            rust_type = RustType(self, proto_file, msg_type, field)
            if rust_type.has_custom_type():
                self.extra_crates[crate].update(
                    CRATE_NAME_REGEX.findall(rust_type.custom_type())
                )

            if field.type_name:
                field_type = self.find(field.type_name)
                if crate in self.deps_map:
                    dep_crate, _ = self.crate_from_proto_filename(
                        field_type.proto_file.name
                    )
                    if dep_crate != crate:
                        self.deps_map[crate].add(dep_crate)

            if field.label == FieldDescriptorProto.LABEL_REPEATED:
                msg_impls_copy = False  # Vecs are not Copy.

            # If we use a Blob type, or GRPC Slice
            if typ == FieldDescriptorProto.TYPE_BYTES and (
                field.options.ctype == FieldOptions.CORD
                or field.options.Extensions[extensions_pb2.grpc_slices]
            ):
                (msg_impls_eq, msg_impls_copy) = (False, False)  # Blob is not eq/copy
                self.extra_crates[crate].add("blob_pb")
            # If we use a Bytes type
            elif typ == FieldDescriptorProto.TYPE_BYTES and field.options.Extensions[extensions_pb2.zero_copy]:
                (msg_impls_eq, msg_impls_copy) = (False, False)
                self.extra_crates[crate].add("bytes")
            elif typ in PRIMITIVE_TYPES:
                if not PRIMITIVE_TYPES[typ][1]:
                    msg_impls_eq = False
                if not PRIMITIVE_TYPES[typ][2]:
                    msg_impls_copy = False
            elif typ == FieldDescriptorProto.TYPE_ENUM:
                pass  # Enums are Eq / Copy
            elif typ == FieldDescriptorProto.TYPE_MESSAGE:
                field_fq_msg = (
                    field_type.proto_file.name,
                    "_".join(field_type.path + [field_type.typ.name]),
                )

                if msg_type.options.Extensions[extensions_pb2.preserve_unrecognized]:
                    assert field_type.typ.options.Extensions[
                        extensions_pb2.preserve_unrecognized
                    ], "%s preserves unrecognized but child message %s does not" % (
                        fq_msg,
                        field_fq_msg,
                    )

                self.calc_impls(
                    field_type.proto_file, crate, field_type.typ, field_fq_msg
                )

                if not self.impls_by_msg[field_fq_msg].Eq:
                    msg_impls_eq = False
                if not self.impls_by_msg[field_fq_msg].Copy:
                    msg_impls_copy = False
            else:
                raise RuntimeError(
                    "Unsupported type: {!r}".format(FieldDescriptorProto.Type.Name(typ))
                )

        self.impls_by_msg[fq_msg] = Impls(Eq=msg_impls_eq, Copy=msg_impls_copy)

    def feed(self, proto_file, to_generate):
        # type: (FileDescriptorProto, List[Text]) -> None
        enums, messages = walk(proto_file)

        for name in to_generate:
            crate, _ = self.crate_from_proto_filename(name)
            self.deps_map[crate]

        for path, typ, _ in itertools.chain(enums, messages):
            pt = ProtoType(self, proto_file, path, typ)
            self.proto_types[pt.proto_name()] = pt

        for path, typ, _ in messages:
            fq_msg = (proto_file.name, "_".join(path + [typ.name]))
            crate, _ = self.crate_from_proto_filename(proto_file.name)
            self.calc_impls(proto_file, crate, typ, fq_msg)

    def find(self, typename):
        # type: (Text) -> ProtoType
        if typename in self.proto_types:
            return self.proto_types[typename]

        raise RuntimeError("Could not find type by proto name: {}".format(typename))

    def _set_boxed_if_recursive(self, visited, looking_for, pt):
        # type: (Set[Text], Text, ProtoType) -> bool
        visited.add(pt.proto_name())
        any_field_boxed = False
        for field in pt.typ.field:
            if (
                field.type == FieldDescriptorProto.TYPE_MESSAGE
                and field.label != FieldDescriptorProto.LABEL_REPEATED
            ):
                need_box = False
                if field.type_name not in visited:
                    need_box = self._set_boxed_if_recursive(
                        visited, looking_for, self.find(field.type_name)
                    )
                if need_box or field.type_name == looking_for:
                    # We only box normal fields, not oneof variants
                    #
                    # TODO: We are restricting this case because the codegen
                    # can't currently box oneof variants.  This means there are
                    # cases won't work with the Rust codegen.  Specifically, if
                    # you have a oneof variant that directly references the
                    # containing message or is co-recursive to another message,
                    # the codegen won't box the variant and the resulting code
                    # won't compile.
                    if not (
                        field.HasField("oneof_index")
                        and pt.typ.oneof_decl[field.oneof_index]
                    ):
                        field.options.Extensions[extensions_pb2.box_it] = True
                    any_field_boxed = True
        return any_field_boxed

    # This will recursively descend through a message and see if there any
    # recursive nesting.  In those cases, it will automatically box the fields.
    def set_boxed_if_recursive(self, pt):
        # type: (ProtoType) -> None
        visited = set()  # type: Set[Text]
        self._set_boxed_if_recursive(visited, pt.proto_name(), pt)

    def get_lib_and_mod_rs(self, mod_tree, derive_serde):
        # type: (ModTree, bool) -> Iterator[Tuple[Text, Text]]
        for crate, deps in six.iteritems(self.deps_map):
            librs = CodeWriter(None, None, None, None)  # type: ignore
            librs.write("#[macro_use]")
            librs.write("extern crate lazy_static;")
            if derive_serde:
                librs.write("#[macro_use]")
                librs.write("extern crate serde;")
            librs.write("")

            def mod_tree_dfs(mod_prefix_path, sub_mod_tree):
                # type: (Text, ModTree) -> Iterator[Tuple[Text, Text]]
                if not sub_mod_tree:
                    return

                filename = mod_prefix_path + "/mod.rs"
                content = "\n".join(
                    ["// @" + "generated, do not edit", ""]
                    + [
                        # TODO(ilevkivskyi): mypy reports a str/bytes issue.
                        "pub mod %s;" % mod  # type: ignore[misc]
                        for mod in sorted(six.iterkeys(sub_mod_tree))
                    ]
                    + [""]
                )
                yield filename, content

                for child_mod_name, child_mod_tree in six.iteritems(sub_mod_tree):
                    for res in mod_tree_dfs(
                        mod_prefix_path + "/" + child_mod_name, child_mod_tree
                    ):
                        yield res

            crate_mod_tree = mod_tree[crate]
            for mod_name, child_mod_tree in sorted(six.iteritems(crate_mod_tree)):
                librs.write("pub mod %s;" % mod_name)

                for res in mod_tree_dfs(crate + "/src/" + mod_name, child_mod_tree):
                    yield res

            filename = crate + "/src/lib.rs"
            content = RS_HEADER + librs.content.getvalue()
            yield filename, content

    def get_build_file(self):
        # type: () -> Iterator[Tuple[Text, Text]]
        for crate in six.iterkeys(self.deps_map):
            build_file = BUILD_TEMPLATE.format(crate=crate)
            yield crate, build_file

    def get_spec_toml_file(self, derive_serde):
        # type: (bool) -> Iterator[Tuple[Text, Text]]
        for crate, deps in six.iteritems(self.deps_map):
            all_deps = ({"lazy_static", "pb-jelly"} | deps | self.extra_crates[crate]) - {
                "std"
            }
            if derive_serde:
                all_deps.update({"serde"})
            features = {u"serde": u'features=["serde_derive"]'}
            deps_str = "\n".join(
                "{dep} = {{{feat}}}".format(dep=dep, feat=features.get(dep, ""))
                for dep in sorted(all_deps)
            )
            targets = SPEC_TOML_TEMPLATE.format(crate=crate, deps=deps_str)
            yield crate, targets

    def get_cargo_toml_file(self, derive_serde):
        # type: (Text, bool) -> Iterator[Tuple[Text, Text]]
        for crate, deps in six.iteritems(self.deps_map):
            all_deps = ({"lazy_static", "pb-jelly"} | deps | self.extra_crates[crate]) - {
                "std"
            }
            if derive_serde:
                all_deps.update({"serde"})
            features = {u"serde": u' features = ["serde_derive"]'}
            versions = {
                u"lazy_static": u' version = "1.4.0" ',
                u"pb-jelly": u' version = "0.0.1" ',
                u"serde": u' version = "1.0.114" ',
                u"bytes": u' version = "0.5.6" '
            }

            deps_lines = []
            for dep in sorted(all_deps):
                fields = []
                if dep in features:
                    fields.append(features[dep])
                if dep in versions:
                    fields.append(versions[dep])
                else:
                    fields.append(u'path = "../{dep}"'.format(dep=dep))
                deps_lines.append(
                    "{dep} = {{{fields}}}".format(dep=dep, fields=",".join(fields))
                )

            targets = CARGO_TOML_TEMPLATE.format(crate=crate, deps="\n".join(deps_lines))
            yield crate, targets

    def crate_from_proto_filename(self, proto_filename):
        # type: (Text) -> Tuple[Text, List[Text]]
        filename = proto_filename.replace(self.prefix_to_clear, "").replace(".proto", "")
        mod_parts_unsanitized = filename.split("/")
        mod_parts = [
            mod + "_" if mod in RESERVED_KEYWORDS else mod
            for mod in mod_parts_unsanitized
        ]
        # Each proto module will become its own crate. We append "_proto" to the crate name
        # to disambiguate the top level crate names.
        if self.crate_per_dir:
            crate_name = "proto_" + "_".join(mod_parts[:-1])
            return crate_name, [mod_parts[-1]]
        crate_name = "proto_" + mod_parts[0]
        return crate_name, mod_parts[1:]


BUILD_TEMPLATE = """
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "{crate}",
    crate_type = "lib",
)
"""

SPEC_TOML_TEMPLATE = (
    """# @"""
    + """generated, do not edit
name = "{crate}"
edition = "2018"

[dependencies]
{deps}
"""
)

CARGO_TOML_TEMPLATE = (
    """# @"""
    + """generated, do not edit
[package]
name = "{crate}"
version = "0.0.1"
edition = "2018"

[dependencies]
{deps}
"""
)

RS_HEADER = (
    """// @"""
    + """generated, do not edit
#![warn(rust_2018_idioms)]
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

"""
)

ModTree = DefaultDict[Text, DefaultDict]


def generate_single_crate(ctx, file_prefix, file_to_generate, response):
    # type: (Context, Text, List[Text], plugin.CodeGeneratorResponse) -> None
    def new_mod_tree():
        # type: () -> ModTree
        return defaultdict(new_mod_tree)

    mod_tree = new_mod_tree()

    # Set iteration order is nondeterministic, but this is ok, because we never iterate through this
    processed_files = set()  # type: Set[Text]
    derive_serde = False

    for proto_file_name in file_to_generate:

        # Detect packages which collide with filenames. The rust codegen does not support those due
        # to the rust module structure.
        #
        # eg. edgestore/engine.proto and edgestore/engine/service.proto
        # engine would be both a file and container module
        package_path = proto_file_name.rsplit("/", 1)[0]
        if package_path in processed_files:
            raise Exception(
                "Unable to process proto {}. It collides with package {}.".format(
                    proto_file_name, package_path
                )
            )
        processed_files.add(proto_file_name[: -len(".proto")])  # Strip the .proto

        crate_name, mod_parts = ctx.crate_from_proto_filename(proto_file_name)
        parent_mods = mod_parts[:-1]
        mod_name = mod_parts[-1]

        def add_mod(writer):
            # type: (CodeWriter) -> None
            rs_file_name = (
                file_prefix + "/".join([crate_name, "src"] + writer.mod_parts) + ".rs"
            )

            output = response.file.add()
            output.name = rs_file_name
            output.content = str(RS_HEADER) + writer.content.getvalue()

            curr = mod_tree[crate_name]
            for mod in writer.mod_parts:
                curr = curr[mod]

        # Now generate code!
        proto_file = next(f for f in request.proto_file if f.name == proto_file_name)
        writer = CodeWriter(ctx, proto_file, crate_name, parent_mods + [mod_name])
        if writer.derive_serde:
            derive_serde = True

        enums, messages = walk(proto_file)

        for path, enum_typ, scl in enums:
            writer.gen_enum(path, enum_typ, scl)
            writer.write("")

        for path, msg_typ, scl in messages:
            ctx.set_boxed_if_recursive(ProtoType(ctx, proto_file, path, msg_typ))
            writer.gen_msg(path, msg_typ, scl)
            writer.gen_msg_descriptor(path, msg_typ, proto_file.package, scl)
            writer.write("")

        add_mod(writer=writer)

    # Note that output filenames must use "/" even on windows. It is part of the
    # protoc plugin protocol. The plugin speaks os-independent in "/". Thus, we
    # should not use os.path.sep or os.path.join

    for filename, content in ctx.get_lib_and_mod_rs(mod_tree, derive_serde):
        output = response.file.add()
        output.name = file_prefix + filename
        output.content = content

    if "generate_build_files" in request.parameter:
        for crate, build_file in ctx.get_build_file():
            output = response.file.add()
            output.name = file_prefix + crate + "/BUILD.in-gen-proto~"
            output.content = build_file
    elif "generate_spec_toml" in request.parameter:
        for crate, spec_toml_file in ctx.get_spec_toml_file(derive_serde):
            output = response.file.add()
            output.name = file_prefix + crate + "/Spec.toml"
            output.content = spec_toml_file
    else:
        # Generate good ol Cargo.toml files
        for crate, cargo_toml_file in ctx.get_cargo_toml_file(derive_serde):
            output = response.file.add()
            output.name = file_prefix + crate + "/Cargo.toml"
            output.content = cargo_toml_file


def generate_code(request, response):
    # type: (plugin.CodeGeneratorRequest, plugin.CodeGeneratorResponse) -> None
    to_generate = list(request.file_to_generate)

    prefix_to_clear = ""
    if "prefix_to_clear" in request.parameter:
        prefix_to_clear = [arg for arg in request.parameter.split() if "prefix_to_clear" in arg][0].split("=")[1]

    if "crate_per_dir" in request.parameter:
        files_by_dir = defaultdict(list)  # type: Dict[Text, List[Text]]
        for file_path in to_generate:
            dir_path, file_name = os.path.split(file_path)
            files_by_dir[dir_path].append(file_path)

        for dir_path, to_generate in sorted(files_by_dir.items()):
            file_prefix = dir_path.replace(prefix_to_clear, "").split("/")[0] + "/"
            ctx = Context(crate_per_dir=True, prefix_to_clear=prefix_to_clear)
            for proto_file in request.proto_file:
                ctx.feed(proto_file, to_generate)
            generate_single_crate(ctx, file_prefix, to_generate, response)
    else:
        ctx = Context(crate_per_dir=False, prefix_to_clear=prefix_to_clear)
        for proto_file in request.proto_file:
            ctx.feed(proto_file, to_generate)
        generate_single_crate(ctx, "", to_generate, response)


if __name__ == "__main__":
    # Read request message from stdin
    if six.PY3:
        data = sys.stdin.buffer.read()
    else:
        data = sys.stdin.read()

    # Parse request
    request = plugin.CodeGeneratorRequest()
    request.ParseFromString(data)

    # Create response
    response = plugin.CodeGeneratorResponse()

    # Generate code
    generate_code(request, response)

    # Serialise response message
    output = response.SerializeToString()

    # Write to stdout
    if six.PY3:
        sys.stdout.buffer.write(output)
    else:
        sys.stdout.write(output)
