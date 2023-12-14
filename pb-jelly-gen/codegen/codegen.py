#!/usr/bin/env python3

import os
import re
import sys

from collections import defaultdict, namedtuple, OrderedDict
from contextlib import contextmanager
from typing import (
    Any,
    Callable,
    DefaultDict,
    Dict,
    Generic,
    Iterable,
    Iterator,
    List,
    NamedTuple,
    Optional,
    Set,
    Text,
    Tuple,
    TypeVar,
    Union,
)

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
SMALL_STRING_OPT_TYPE = "::compact_str::CompactString"
# pull out `x` from every instance of `::x::y::z`, but not `y` or `z`
CRATE_NAME_REGEX = re.compile(r"(?:^|\W)::(\w+)(?:::\w+)*")

# Keywords in rust which cannot be module names.
RESERVED_KEYWORDS = {
    "Self",
    "abstract",
    "alignof",
    "as",
    "async",
    "await",
    "become",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "do",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "final",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "macro",
    "match",
    "mod",
    "move",
    "mut",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pub",
    "pure",
    "ref",
    "return",
    "self",
    "sizeof",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "typeof",
    "unsafe",
    "unsized",
    "use",
    "virtual",
    "where",
    "while",
    "yield",
}


def escape_name(s: str) -> str:
    if s in RESERVED_KEYWORDS:
        return "r#" + s
    return s


# SourceCodeLocation is defined by `message Location` here
# https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto
SourceCodeLocation = List[int]
ProtoTypes = Union[FileDescriptorProto, EnumDescriptorProto, DescriptorProto]
WalkRet = Tuple[
    List[Tuple[List[Text], EnumDescriptorProto, SourceCodeLocation]],
    List[Tuple[List[Text], DescriptorProto, SourceCodeLocation]],
]
ModTree = DefaultDict[Text, DefaultDict[Text, Any]]


T = TypeVar("T")


class StronglyConnectedComponents(Generic[T]):
    __slots__ = ("S", "B", "index", "next_component")

    def __init__(self) -> None:
        self.S: List[T] = []
        self.B: List[int] = []
        self.index: Dict[T, int] = {}
        # Since we don't know the number of nodes in advance, just start counting from a reasonably high number
        self.next_component = 2**32

    def process(
        self,
        node: T,
        edges_from: Callable[[T], Iterable[T]],
        callback: Callable[[Set[T]], None],
    ) -> None:
        """
        Computes the strongly connected components of a directed graph on the fly.

        Calls `callback` with each component in topological order.
        Specifically, each component will appears after the component containing `edges_from(node)`.
        All nodes reachable from `node` will be processed, if they have not already been.

        After, `self.index` will also be populated with component IDs for each visited node.
        """
        if node not in self.index:
            self._dfs(node, edges_from, callback)

    # a variant of https://en.wikipedia.org/wiki/Path-based_strong_component_algorithm;
    # see "Path-based depth-first search for strong and biconnected components" by Harold N. Gabow,
    # https://www.cs.princeton.edu/courses/archive/spr04/cos423/handouts/path%20based...pdf
    def _dfs(
        self,
        node: T,
        edges_from: Callable[[T], Iterable[T]],
        callback: Callable[[Set[T]], None],
    ) -> None:
        self.S.append(node)
        my_index = len(self.S)
        self.index[node] = my_index
        self.B.append(my_index)

        for next_node in edges_from(node):
            if next_node in self.index:
                while self.index[next_node] < self.B[-1]:
                    self.B.pop()
            else:
                self._dfs(next_node, edges_from, callback)

        if my_index == self.B[-1]:
            self.B.pop()
            component = set()
            while len(self.S) >= my_index:
                v = self.S.pop()
                self.index[v] = self.next_component
                component.add(v)
            self.next_component += 1
            callback(component)


def camelcase(underscored: Text) -> Text:
    return "".join(s.capitalize() for s in underscored.split("_"))


class StringIO(object):
    def __init__(self) -> None:
        self.contents: List[Text] = []

    def write(self, s: Text) -> None:
        self.contents.append(s)

    def getvalue(self) -> Text:
        return "".join(self.contents)


class RustType(object):
    def __init__(
        self,
        ctx: "Context",
        proto_file: FileDescriptorProto,
        msg_type: DescriptorProto,
        field: FieldDescriptorProto,
    ) -> None:
        self.ctx = ctx
        self.proto_file = proto_file
        self.field = field
        self.is_proto3 = proto_file.syntax == "proto3"
        # note that proto3 optional is considered a oneof, but we don't emit it as such
        self.oneof = (
            field.HasField("oneof_index")
            and not field.proto3_optional
            and msg_type.oneof_decl[field.oneof_index]
        )

    def default(self, msg_name: Text) -> Text:
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

    def is_length_delimited(self) -> bool:
        length_delimited_types = [
            FieldDescriptorProto.TYPE_MESSAGE,
            FieldDescriptorProto.TYPE_STRING,
            FieldDescriptorProto.TYPE_BYTES,
        ]

        return self.field.type in length_delimited_types

    def wire_format(self) -> Text:
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

    def is_grpc_slices(self) -> bool:
        return (
            self.field.type == FieldDescriptorProto.TYPE_BYTES
            and self.field.options.Extensions[extensions_pb2.grpc_slices]
        )

    def is_blob(self) -> bool:
        return (
            self.field.type == FieldDescriptorProto.TYPE_BYTES
            and self.field.options.ctype == FieldOptions.CORD
        )

    def is_lazy_bytes(self) -> bool:
        return (
            self.field.type == FieldDescriptorProto.TYPE_BYTES
            and self.field.options.Extensions[extensions_pb2.zero_copy]
        )

    def is_small_string_optimization(self) -> bool:
        return (
            self.field.type == FieldDescriptorProto.TYPE_STRING
            and self.field.options.Extensions[extensions_pb2.sso]
        )

    def is_boxed(self) -> bool:
        return (
            self.field.type == FieldDescriptorProto.TYPE_MESSAGE
            and self.field.options.Extensions[extensions_pb2.box_it]
        )

    def has_custom_type(self) -> bool:
        return self.field.options.HasExtension(extensions_pb2.type)

    def custom_type(self) -> Text:
        return self.field.options.Extensions[extensions_pb2.type]

    def is_nullable(self) -> bool:
        if self.oneof:
            return False
        if (
            self.field.type in PRIMITIVE_TYPES
            and self.is_proto3
            and not self.field.proto3_optional
        ):
            # Primitive types in proto3 are not nullable by default;
            # if missing, they are decoded as 0-value ("" or 0).
            # proto3_optional overrides this and treats those fields like 1-variant oneofs on the wire,
            # enabling them to be truly optional
            return False
        if self.field.options.HasExtension(extensions_pb2.nullable_field):
            # We still allow overriding nullability as an extension
            return self.field.options.Extensions[extensions_pb2.nullable_field]
        return (
            not self.is_proto3
            or self.field.type == FieldDescriptorProto.TYPE_MESSAGE
            or self.field.proto3_optional
        )

    def is_empty_oneof_field(self) -> bool:
        assert self.oneof
        return self.field.type_name == ".google.protobuf.Empty" and not self.is_boxed()

    def can_be_packed(self) -> bool:
        # Return true if incoming messages could be packed on the wire
        return (
            self.field.label == FieldDescriptorProto.LABEL_REPEATED
            and self.wire_format()
            in (
                "Varint",
                "Fixed64",
                "Fixed32",
            )
        )

    def should_serialize_packed(self) -> bool:
        return self.can_be_packed() and (self.field.options.packed or self.is_proto3)

    def is_repeated(self) -> bool:
        return self.field.label == FieldDescriptorProto.LABEL_REPEATED

    def set_method(self) -> Tuple[Text, Text]:
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
            if self.is_small_string_optimization():
                return SMALL_STRING_OPT_TYPE, "v"
            else:
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
                return "::std::boxed::Box<%s>" % self.rust_type(), "v"
            else:
                return self.rust_type(), "v"
        raise AssertionError("Unexpected field type")

    def take_method(self) -> Tuple[Optional[Text], Optional[Text]]:
        assert not self.is_repeated()
        has_take_method = [
            FieldDescriptorProto.TYPE_STRING,
            FieldDescriptorProto.TYPE_BYTES,
            FieldDescriptorProto.TYPE_MESSAGE,
        ]

        if not self.field.type in has_take_method:
            return None, None

        expr = "self.%s.take().unwrap_or_default()" % escape_name(self.field.name)

        if self.field.type == FieldDescriptorProto.TYPE_STRING:
            if self.is_small_string_optimization():
                return SMALL_STRING_OPT_TYPE, expr
            else:
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
                return "::std::boxed::Box<%s>" % self.rust_type(), expr
            else:
                return self.rust_type(), expr
        raise AssertionError("Unexpected field type")

    def get_method(self) -> Tuple[Text, Text]:
        assert not self.is_repeated()
        name = escape_name(self.field.name)

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
            return ("&str", 'self.%s.as_deref().unwrap_or("")' % name)
        elif self.field.type == FieldDescriptorProto.TYPE_BYTES:
            assert not (
                self.is_blob() or self.is_grpc_slices() or self.is_lazy_bytes()
            ), "Can't generate get method for lazy field"
            return "&[u8]", "self.%s.as_deref().unwrap_or(&[])" % name
        elif self.field.type == FieldDescriptorProto.TYPE_ENUM:
            return self.rust_type(), "self.%s.unwrap_or_default()" % name
        elif self.field.type == FieldDescriptorProto.TYPE_MESSAGE:
            deref = "" if not self.is_boxed() else ".map(::std::ops::Deref::deref)"
            return (
                "&" + self.rust_type(),
                "self.%s.as_ref()%s.unwrap_or(&%s_default)"
                % (name, deref, self.rust_type()),
            )
        raise AssertionError("Unexpected field type")

    def may_use_grpc_slices(self) -> bool:
        if (
            self.has_custom_type()
            or self.is_blob()
            or self.is_grpc_slices()
            or self.is_lazy_bytes()
        ):
            return True
        if self.field.type == FieldDescriptorProto.TYPE_MESSAGE:
            return self.ctx.impls_by_msg[self.field.type_name].may_use_grpc_slices
        return False

    def rust_type(self) -> Text:
        typ = self.field.type

        if self.has_custom_type():
            return self.custom_type()

        if self.is_blob():
            return BLOB_TYPE

        if self.is_grpc_slices():
            return VEC_SLICE_TYPE

        if self.is_lazy_bytes():
            return LAZY_BYTES_TYPE

        if self.is_small_string_optimization():
            return SMALL_STRING_OPT_TYPE

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

    def storage_type(self) -> str:
        rust_type = self.rust_type()

        if self.is_boxed():
            rust_type = "::std::boxed::Box<%s>" % rust_type

        if self.is_repeated():
            rust_type = "::std::vec::Vec<%s>" % rust_type
        elif self.is_nullable():
            rust_type = "::std::option::Option<%s>" % rust_type

        return rust_type

    def oneof_field_match(self, var: Text) -> Text:
        if self.is_empty_oneof_field():
            return camelcase(self.field.name)
        else:
            return "%s(%s)" % (camelcase(self.field.name), var)

    def oneof_val(self, msg_name: Text, var: Text) -> Text:
        assert self.oneof
        oneofv = "%s::%s" % (
            oneof_msg_name(msg_name, self.oneof),
            self.oneof_field_match(var),
        )

        if oneof_nullable(self.oneof):
            oneofv = "Some(%s)" % oneofv

        return oneofv


def oneof_msg_name(parent_msg_name: Text, oneof: OneofDescriptorProto) -> Text:
    return "%s_%s" % (parent_msg_name, camelcase(oneof.name))


def oneof_nullable(oneof: OneofDescriptorProto) -> bool:
    return (
        not oneof.options.HasExtension(extensions_pb2.nullable)
        or oneof.options.Extensions[extensions_pb2.nullable]
    )


def enum_err_if_default_or_unknown(enum: EnumDescriptorProto) -> bool:
    return (
        enum.options.HasExtension(extensions_pb2.err_if_default_or_unknown)
        and enum.options.Extensions[extensions_pb2.err_if_default_or_unknown]
    )


def enum_closed(enum: EnumDescriptorProto) -> bool:
    return (
        enum.options.HasExtension(extensions_pb2.closed_enum)
        and enum.options.Extensions[extensions_pb2.closed_enum]
    )


@contextmanager
def block(
    ctx: "CodeWriter", header: Text, start: Text = " {", end: Text = "}"
) -> Iterator[None]:
    ctx.write("%s%s" % (header, start))
    ctx.indentation += 1
    yield
    ctx.indentation -= 1
    ctx.write(end)


@contextmanager
def field_iter(
    ctx: "CodeWriter",
    var: Text,
    msg_name: Text,
    msg_type: DescriptorProto,
    field: FieldDescriptorProto,
) -> Iterator[None]:
    typ = ctx.rust_type(msg_type, field)

    if typ.oneof:
        # For oneofs, we always emit, even if the primitive field is set to a 0 value
        # This is so we can distinguish which field of oneof is set.
        with block(
            ctx,
            "if let %s = self.%s"
            % (typ.oneof_val(msg_name, "ref " + var), escape_name(typ.oneof.name)),
        ):
            if typ.is_empty_oneof_field():
                ctx.write(
                    "let %s: &%s = &::std::default::Default::default();"
                    % (var, typ.rust_type())
                )
            elif typ.is_boxed():
                ctx.write(
                    "let %(var)s: &%(typ)s = &**%(var)s;"
                    % dict(var=var, typ=typ.rust_type())
                )
            yield
    elif (
        field.type == FieldDescriptorProto.TYPE_MESSAGE
        and not typ.is_nullable()
        and not typ.is_repeated()
    ):
        # Always emit messages explicitly marked as non-nullable
        deref = "*" if typ.is_boxed() else ""
        with block(ctx, ""):
            ctx.write("let %s = &%sself.%s;" % (var, deref, escape_name(field.name)))
            yield
    elif (
        field.type == FieldDescriptorProto.TYPE_ENUM
        and not typ.is_repeated()
        and enum_err_if_default_or_unknown(ctx.ctx.find_enum(field.type_name).typ)
    ):
        # The default value (as considered by proto) doesn't appear in the generated enum and
        # doesn't map to .default(). All of the values that actually get generated need to get
        # encoded.
        ctx.write("let %s = &self.%s;" % (var, escape_name(field.name)))
        yield
    elif not typ.is_nullable() and not typ.is_repeated():
        # For proto3, we remove the Option for primitive fields.
        # We only run internal code if the primitive field is non-default for proto3
        # Rather than looping, we set the variable once and run the inner code once
        with block(
            ctx,
            "if self.%s != <%s as ::std::default::Default>::default()"
            % (escape_name(field.name), typ.storage_type()),
        ):
            if typ.is_boxed():
                ctx.write("let %s = &*self.%s;" % (var, escape_name(field.name)))
            else:
                ctx.write("let %s = &self.%s;" % (var, escape_name(field.name)))
            yield
    elif typ.is_nullable() and not typ.is_repeated():
        with block(
            ctx, "if let Some(ref %s) = self.%s" % (var, escape_name(field.name))
        ):
            if typ.is_boxed():
                ctx.write("let %s = &**%s;" % (var, var))
            yield
    else:
        with block(ctx, "for %s in &self.%s" % (var, escape_name(field.name))):
            if typ.is_boxed():
                ctx.write("let %s = &**%s;" % (var, var))
            yield


class CodeWriter(object):
    def __init__(
        self,
        ctx: "Context",
        proto_file: FileDescriptorProto,
        crate: Text,
        mod_parts: List[Text],
    ) -> None:
        self.ctx = ctx
        self.proto_file = proto_file
        self.crate = crate
        self.mod_parts = mod_parts
        self.indentation = 0
        self.content = StringIO()
        self.is_proto3 = proto_file and proto_file.syntax == "proto3"
        self.uses_sso = False
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

    def write(self, s: Text) -> None:
        if s == "":
            self.content.write("\n")
            return

        for _ in range(self.indentation):
            self.content.write("  ")
        self.content.write(s)
        self.content.write("\n")

    def write_line_broken_text_with_prefix(
        self, text_block: Text, prefix: Text
    ) -> None:
        if not text_block:
            return
        for l in text_block.rstrip().split("\n"):
            if l:
                self.write("{}{}".format(prefix, l))
            else:
                self.write("")

    def write_comments(self, sci_loc: Optional[SourceCodeInfo.Location]) -> None:
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

    def rust_type(
        self, msg_type: DescriptorProto, field: FieldDescriptorProto
    ) -> RustType:
        rust_type = RustType(self.ctx, self.proto_file, msg_type, field)

        # checks if any of our types use a small string optimization
        if not self.uses_sso and rust_type.is_small_string_optimization():
            self.uses_sso = True

        return rust_type

    def gen_closed_enum(
        self,
        name: Text,
        enum_variants: List[Tuple[int, EnumValueDescriptorProto]],
        scl: SourceCodeLocation,
    ) -> None:
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
                vfn = EnumDescriptorProto.VALUE_FIELD_NUMBER
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

    def gen_open_enum(
        self,
        name: Text,
        enum_variants: List[Tuple[int, EnumValueDescriptorProto]],
        scl: SourceCodeLocation,
    ) -> None:
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
                vfn = EnumDescriptorProto.VALUE_FIELD_NUMBER
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
            self.write("type Closed = {};".format(closed_name))
            with block(
                self,
                "fn into_known(self) -> ::std::option::Option<%s>" % closed_name,
            ):
                with block(self, "match self"):
                    for _, value in enum_variants:
                        self.write(
                            "%s::%s => Some(%s::%s),"
                            % (name, value.name, closed_name, value.name)
                        )
                    self.write("_ => None,")

        with block(self, "impl ::std::fmt::Debug for " + name):
            with block(
                self,
                "fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result",
            ):
                with block(
                    self, "match <Self as ::pb_jelly::OpenProtoEnum>::name(*self)"
                ):
                    self.write('Some(s) => write!(f, "{}", s),')
                    self.write('None => write!(f, "Unknown({})", self.0),')

        self.gen_closed_enum(closed_name, enum_variants, scl)

    def gen_enum(
        self, path: List[Text], enum_type: EnumDescriptorProto, scl: SourceCodeLocation
    ) -> None:
        assert self.indentation == 0
        name = "_".join(path + [enum_type.name])
        if enum_err_if_default_or_unknown(enum_type):
            assert not enum_closed(enum_type)
            self.gen_closed_enum(
                name, [x for x in enumerate(enum_type.value) if x[1].number != 0], scl
            )
        elif enum_closed(enum_type):
            self.gen_closed_enum(name, list(enumerate(enum_type.value)), scl)
        else:
            self.gen_open_enum(name, list(enumerate(enum_type.value)), scl)

    def gen_msg(
        self, path: List[Text], msg_type: DescriptorProto, scl: SourceCodeLocation
    ) -> None:
        assert self.indentation == 0
        name = "_".join(path + [msg_type.name])
        escaped_name = escape_name(name)

        oneof_fields: DefaultDict[Text, List[FieldDescriptorProto]] = defaultdict(list)
        proto3_optional_synthetic_oneofs: Set[int] = {
            field.oneof_index for field in msg_type.field if field.proto3_optional
        }
        # Filter out oneofs synthesized by proto3 optional; we treat these as plain `Option`al fields, not oneofs
        oneof_decls = [
            oneof
            for ix, oneof in enumerate(msg_type.oneof_decl)
            if ix not in proto3_optional_synthetic_oneofs
        ]

        derives = ["Clone", "Debug", "PartialEq"]
        if self.derive_serde:
            derives.extend(["Deserialize", "Serialize"])

        impls = self.ctx.impls_by_msg[
            ProtoType(self.ctx, self.proto_file, path, msg_type).proto_name()
        ]
        if impls.impls_eq:
            derives.extend(["Eq", "PartialOrd", "Ord", "Hash"])
        if impls.impls_copy:
            derives.append("Copy")

        self.write_comments(self.source_code_info_by_scl.get(tuple(scl)))

        self.write("#[derive(%s)]" % ", ".join(sorted(derives)))
        with block(self, "pub struct " + escaped_name):
            for idx, field in enumerate(msg_type.field):
                ffn = DescriptorProto.FIELD_FIELD_NUMBER
                self.write_comments(
                    self.source_code_info_by_scl.get(tuple(scl + [ffn, idx]))
                )

                typ = self.rust_type(msg_type, field)
                if typ.oneof:
                    oneof_fields[typ.oneof.name].append(field)
                else:
                    self.write(
                        "pub %s: %s," % (escape_name(field.name), typ.storage_type())
                    )

            for oneof in oneof_decls:
                if oneof_nullable(oneof):
                    self.write(
                        "pub %s: ::std::option::Option<%s>,"
                        % (escape_name(oneof.name), oneof_msg_name(name, oneof))
                    )
                else:
                    self.write(
                        "pub %s: %s,"
                        % (escape_name(oneof.name), oneof_msg_name(name, oneof))
                    )

            if msg_type.options.Extensions[extensions_pb2.preserve_unrecognized]:
                self.write("pub _unrecognized: Vec<u8>,")

        # Generate any oneof enum structs
        for oneof in oneof_decls:
            self.write("#[derive(%s)]" % ", ".join(sorted(derives)))
            with block(self, "pub enum " + oneof_msg_name(name, oneof)):
                for oneof_field in oneof_fields[oneof.name]:
                    typ = self.rust_type(msg_type, oneof_field)
                    self.write("%s," % typ.oneof_field_match(typ.storage_type()))

        if not self.is_proto3:
            with block(self, "impl " + escaped_name):
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
                            self.write("self.%s = v;" % escape_name(field.name))

                        with block(
                            self,
                            "pub fn take_%s(&mut self) -> ::std::vec::Vec<%s>"
                            % (field.name, typ.rust_type()),
                        ):
                            self.write(
                                "::std::mem::take(&mut self.%s)"
                                % escape_name(field.name)
                            )

                        with block(
                            self,
                            "pub fn get_%s(&self) -> &[%s]"
                            % (field.name, typ.rust_type()),
                        ):
                            self.write("&self.%s" % escape_name(field.name))

                        with block(
                            self,
                            "pub fn mut_%s(&mut self) -> &mut ::std::vec::Vec<%s>"
                            % (field.name, typ.rust_type()),
                        ):
                            self.write("&mut self." + field.name)

                    elif typ.is_nullable():
                        with block(self, "pub fn has_%s(&self) -> bool" % field.name):
                            self.write("self.%s.is_some()" % escape_name(field.name))

                        input_type, input_expr = typ.set_method()
                        with block(
                            self,
                            "pub fn set_%s(&mut self, v: %s)"
                            % (field.name, input_type),
                        ):
                            self.write(
                                "self.%s = Some(%s);"
                                % (escape_name(field.name), input_expr)
                            )

                        return_type, return_expr = typ.take_method()
                        if return_type is not None and return_expr is not None:
                            with block(
                                self,
                                "pub fn take_%s(&mut self) -> %s"
                                % (field.name, return_type),
                            ):
                                self.write(return_expr)

                        if not (
                            typ.is_blob() or typ.is_grpc_slices() or typ.is_lazy_bytes()
                        ):
                            # It's hard to make this make sense, so let's not generate `get` method for lazy things.
                            return_type, return_expr = typ.get_method()
                            with block(
                                self,
                                "pub fn get_%s(&self) -> %s"
                                % (field.name, return_type),
                            ):
                                self.write(return_expr)

        with block(self, "impl ::std::default::Default for " + escaped_name):
            with block(self, "fn default() -> Self"):
                with block(self, escaped_name):
                    for field in msg_type.field:
                        typ = self.rust_type(msg_type, field)
                        if not typ.oneof:
                            self.write(
                                "%s: %s," % (escape_name(field.name), typ.default(name))
                            )
                    for oneof in oneof_decls:
                        oneof_field = oneof_fields[oneof.name][0]
                        typ = self.rust_type(msg_type, oneof_field)
                        self.write(
                            "%s: %s," % (escape_name(oneof.name), typ.default(name))
                        )
                    if msg_type.options.Extensions[
                        extensions_pb2.preserve_unrecognized
                    ]:
                        self.write("_unrecognized: Vec::new(),")

        with block(self, "lazy_static!"):
            self.write(
                "pub static ref %s_default: %s = %s::default();"
                % (name, escaped_name, escaped_name)
            )

        with block(self, "impl ::pb_jelly::Message for " + escaped_name):
            with block(
                self,
                "fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor>",
            ):
                name = "_".join(path + [msg_type.name])
                full_name = (
                    ".".join([self.proto_file.package, name])
                    if self.proto_file.package
                    else name
                )

                with block(
                    self, "Some(::pb_jelly::MessageDescriptor", start=" {", end="})"
                ):
                    self.write('name: "%s",' % name)
                    self.write('full_name: "%s",' % full_name)
                    with block(self, "fields:", start=" &[", end="],"):
                        for i, field in enumerate(msg_type.field):
                            with block(
                                self,
                                "::pb_jelly::FieldDescriptor",
                                start=" {",
                                end="},",
                            ):
                                full_name = ".".join(
                                    [self.proto_file.package, name, field.name]
                                    if self.proto_file.package
                                    else [name, field.name]
                                )

                                typ = self.rust_type(msg_type, field)
                                self.write('name: "%s",' % field.name)
                                self.write('full_name: "%s",' % full_name)
                                self.write("index: %d," % i)
                                self.write("number: %d," % field.number)
                                self.write(
                                    "typ: ::pb_jelly::wire_format::Type::%s,"
                                    % typ.wire_format()
                                )
                                if field.label == FieldDescriptorProto.LABEL_OPTIONAL:
                                    self.write("label: ::pb_jelly::Label::Optional,")
                                elif field.label == FieldDescriptorProto.LABEL_REQUIRED:
                                    self.write("label: ::pb_jelly::Label::Required,")
                                elif field.label == FieldDescriptorProto.LABEL_REPEATED:
                                    self.write("label: ::pb_jelly::Label::Repeated,")

                                if (
                                    field.HasField("oneof_index")
                                    and not field.proto3_optional
                                ):
                                    self.write(
                                        "oneof_index: Some(%d)," % field.oneof_index
                                    )
                                else:
                                    self.write("oneof_index: None,")

                    with block(self, "oneofs:", start=" &[", end="],"):
                        # Note that synthetic oneofs are always located at the end of `msg_type.oneof_decl`,
                        # so the oneof indices will still match
                        for oneof in oneof_decls:
                            with block(
                                self,
                                "::pb_jelly::OneofDescriptor",
                                start=" {",
                                end="},",
                            ):
                                self.write('name: "%s",' % oneof.name)

            with block(self, "fn compute_size(&self) -> usize"):
                if (
                    len(msg_type.field) > 0
                    or msg_type.options.Extensions[extensions_pb2.preserve_unrecognized]
                ):
                    self.write("let mut size = 0;")
                    for field in msg_type.field:
                        typ = self.rust_type(msg_type, field)

                        if (
                            not typ.oneof
                            and field.type != FieldDescriptorProto.TYPE_MESSAGE
                            and not (
                                field.type == FieldDescriptorProto.TYPE_ENUM
                                and enum_err_if_default_or_unknown(
                                    self.ctx.find_enum(field.type_name).typ
                                )
                            )
                            and not typ.is_nullable()
                            and not typ.is_repeated()
                            and not typ.is_boxed()
                        ):
                            # Special case this fairly common case to reduce codegen.
                            self.write(
                                "size += ::pb_jelly::helpers::compute_size_scalar::<{typ}>(&self.{escaped_name}, {field_number}, ::pb_jelly::wire_format::Type::{wire_format});".format(
                                    typ=typ.rust_type(),
                                    escaped_name=escape_name(field.name),
                                    field_number=field.number,
                                    wire_format=typ.wire_format(),
                                )
                            )
                        else:
                            self.write("let mut %s_size = 0;" % field.name)
                            with field_iter(self, "val", name, msg_type, field):
                                self.write(
                                    "let l = ::pb_jelly::Message::compute_size(val);"
                                )
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

            if impls.may_use_grpc_slices:
                with block(self, "fn compute_grpc_slices_size(&self) -> usize"):
                    self.write("let mut size = 0;")
                    for field in msg_type.field:
                        rust_type = RustType(self.ctx, self.proto_file, msg_type, field)
                        if rust_type.may_use_grpc_slices():
                            with field_iter(self, "val", name, msg_type, field):
                                self.write(
                                    "size += ::pb_jelly::Message::compute_grpc_slices_size(val);"
                                )
                    self.write("size")

            with block(
                self,
                "fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()>",
            ):
                for field in sorted(msg_type.field, key=lambda f: f.number):
                    typ = self.rust_type(msg_type, field)
                    # In proto2, this ensures we don't emit fields set to None
                    # In proto3, this ensures we don't emit fields set to their default value
                    if typ.should_serialize_packed():
                        with block(
                            self, "if !self.%s.is_empty()" % escape_name(field.name)
                        ):
                            self.write("let mut size = 0;")
                            with field_iter(self, "val", name, msg_type, field):
                                self.write(
                                    "size += ::pb_jelly::Message::compute_size(val);"
                                )
                            self.write(
                                "::pb_jelly::wire_format::write(%s, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;"
                                % field.number
                            )
                            self.write("::pb_jelly::varint::write(size as u64, w)?;")

                    if (
                        not typ.oneof
                        and field.type != FieldDescriptorProto.TYPE_MESSAGE
                        and not (
                            field.type == FieldDescriptorProto.TYPE_ENUM
                            and enum_err_if_default_or_unknown(
                                self.ctx.find_enum(field.type_name).typ
                            )
                        )
                        and not typ.is_nullable()
                        and not typ.is_repeated()
                        and not typ.is_boxed()
                    ):
                        # Special case this fairly common case to reduce codegen.
                        self.write(
                            "::pb_jelly::helpers::serialize_scalar::<W, {typ}>(w, &self.{escaped_name}, {field_number}, ::pb_jelly::wire_format::Type::{wire_format})?;".format(
                                typ=typ.rust_type(),
                                escaped_name=escape_name(field.name),
                                field_number=field.number,
                                wire_format=typ.wire_format(),
                            )
                        )
                    else:
                        with field_iter(self, "val", name, msg_type, field):
                            if not typ.should_serialize_packed():
                                self.write(
                                    "::pb_jelly::wire_format::write(%s, ::pb_jelly::wire_format::Type::%s, w)?;"
                                    % (field.number, typ.wire_format())
                                )
                            if typ.is_length_delimited():
                                self.write(
                                    "let l = ::pb_jelly::Message::compute_size(val);"
                                )
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
                    self.write(
                        "let mut unrecognized = ::pb_jelly::Unrecognized::default();"
                    )

                for oneof in oneof_decls:
                    if not oneof_nullable(oneof):
                        oneof_typ = oneof_msg_name(name, oneof)
                        self.write(
                            "let mut oneof_%s: ::std::option::Option<%s> = None;"
                            % (oneof.name, oneof_typ)
                        )
                err_if_default_field_names: OrderedDict[Text, None] = OrderedDict()
                for field in msg_type.field:
                    typ = self.rust_type(msg_type, field)
                    if (
                        field.type == FieldDescriptorProto.TYPE_ENUM
                        and not typ.is_repeated()
                    ):
                        enum_type = self.ctx.find_enum(field.type_name).typ
                        if enum_err_if_default_or_unknown(enum_type) and not typ.oneof:
                            self.write(
                                "let mut %s: ::std::option::Option<%s> = None;"
                                % (escape_name(field.name), typ.rust_type())
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
                                    self.write(
                                        '::pb_jelly::helpers::deserialize_packed::<B, {typ}>(\
buf, typ, ::pb_jelly::wire_format::Type::{expected_wire_format}, "{msg_name}", {field_number}, &mut self.{escaped_name})?;'.format(
                                            typ=typ.rust_type(),
                                            expected_wire_format=typ.wire_format(),
                                            msg_name=name,
                                            field_number=field.number,
                                            escaped_name=escape_name(field.name),
                                        )
                                    )
                                else:
                                    if typ.is_length_delimited():
                                        self.write(
                                            'let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, {typ}>(\
buf, typ, "{msg_name}", {field_number})?;'.format(
                                                typ=typ.rust_type(),
                                                msg_name=name,
                                                field_number=field.number,
                                            )
                                        )
                                    else:
                                        self.write(
                                            'let val = ::pb_jelly::helpers::deserialize_known_length::<B, {typ}>(\
buf, typ, ::pb_jelly::wire_format::Type::{expected_wire_format}, "{msg_name}", {field_number})?;'.format(
                                                typ=typ.rust_type(),
                                                expected_wire_format=typ.wire_format(),
                                                msg_name=name,
                                                field_number=field.number,
                                            )
                                        )

                                    if typ.is_repeated():
                                        self.write(
                                            "self.%s.push(val);"
                                            % escape_name(field.name)
                                        )
                                    else:
                                        field_val = (
                                            "Box::new(val)" if typ.is_boxed() else "val"
                                        )

                                        if typ.oneof:
                                            if oneof_nullable(typ.oneof):
                                                self.write(
                                                    "self.%s = %s;"
                                                    % (
                                                        escape_name(typ.oneof.name),
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
                                                % (escape_name(field.name), field_val)
                                            )
                                        elif field.name in err_if_default_field_names:
                                            self.write(
                                                "%s = Some(%s);"
                                                % (escape_name(field.name), field_val)
                                            )
                                        else:
                                            self.write(
                                                "self.%s = %s;"
                                                % (escape_name(field.name), field_val)
                                            )
                        with block(self, "_ =>"):
                            if preserve_unrecognized:
                                self.write(
                                    "unrecognized.gather(field_number, typ, &mut buf)?;"
                                )
                            else:
                                self.write("::pb_jelly::skip(typ, &mut buf)?;")
                for oneof in oneof_decls:
                    if not oneof_nullable(oneof):
                        with block(self, "match oneof_" + oneof.name):
                            self.write(
                                "Some(v) => self.%s = v," % escape_name(oneof.name)
                            )
                            self.write(
                                "None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, \"missing value for non-nullable oneof '%s' while parsing message %s.%s\")),"
                                % (oneof.name, self.proto_file.package, msg_type.name)
                            )

                for field_name in err_if_default_field_names:
                    with block(self, "match %s" % escape_name(field_name)):
                        self.write("Some(v) => self.%s = v," % escape_name(field_name))
                        self.write(
                            "None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, \"err_if_default_or_unknown '%s' had no value while parsing message %s.%s\")),"
                            % (field_name, self.proto_file.package, msg_type.name)
                        )

                if preserve_unrecognized:
                    self.write("unrecognized.serialize(&mut self._unrecognized)?;")
                self.write("Ok(())")

        with block(self, "impl ::pb_jelly::Reflection for " + name):
            with block(
                self,
                "fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str>",
            ):
                with block(self, "match oneof_name"):
                    for oneof in oneof_decls:
                        oneof_field = oneof_fields[oneof.name][0]

                    for oneof in oneof_decls:
                        with block(self, '"%s" =>' % oneof.name):
                            for oneof_field in oneof_fields[oneof.name]:
                                with field_iter(
                                    self, "val", name, msg_type, oneof_field
                                ):
                                    self.write('return Some("%s");' % oneof_field.name)
                            self.write("None")
                    with block(self, "_ =>"):
                        self.write('panic!("unknown oneof name given");')

            with block(
                self,
                "fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_>",
            ):
                with block(self, "match field_name"):
                    for field in msg_type.field:
                        typ = self.rust_type(msg_type, field)
                        with block(self, '"%s" =>' % field.name):
                            if typ.oneof:
                                if len(
                                    oneof_fields[typ.oneof.name]
                                ) > 1 or oneof_nullable(typ.oneof):
                                    # Only useful to generate this logic if there is more than one
                                    # possible value for this oneof.
                                    with block(
                                        self,
                                        "match self.%s" % escape_name(typ.oneof.name),
                                    ):
                                        self.write(
                                            "%s => ()," % typ.oneof_val(name, "_")
                                        )
                                        with block(self, "_ =>", start=" {", end="},"):
                                            # If this oneof is not currently set to this variant, we explicitly
                                            # set it to this variant.
                                            self.write(
                                                "self.%s = %s;"
                                                % (
                                                    escape_name(typ.oneof.name),
                                                    typ.oneof_val(
                                                        name,
                                                        "::std::default::Default::default()",
                                                    ),
                                                )
                                            )
                                if typ.is_empty_oneof_field():
                                    self.write(
                                        "::pb_jelly::reflection::FieldMut::Empty"
                                    )
                                else:
                                    with block(
                                        self,
                                        "if let %s = self.%s"
                                        % (
                                            typ.oneof_val(name, "ref mut val"),
                                            escape_name(typ.oneof.name),
                                        ),
                                    ):
                                        if typ.is_boxed():
                                            self.write("let val = &mut **val;")
                                        self.write(
                                            "return ::pb_jelly::reflection::FieldMut::Value(val);"
                                        )
                                    self.write("unreachable!()")
                            elif typ.is_repeated():
                                # TODO: Would be nice to support this, but some more thought would
                                # need to be put into what the API for it looks like.
                                # self.write("::pb_jelly::reflection::FieldMut::Repeated(&mut self.%s)" % field.name)
                                self.write(
                                    'unimplemented!("Repeated fields are not currently supported.")'
                                )
                            elif typ.is_nullable() and typ.is_boxed():
                                self.write(
                                    "::pb_jelly::reflection::FieldMut::Value(self.%s.get_or_insert_with(::std::default::Default::default).as_mut())"
                                    % escape_name(field.name)
                                )
                            elif typ.is_boxed():
                                self.write(
                                    "::pb_jelly::reflection::FieldMut::Value(self.%s.as_mut())"
                                    % escape_name(field.name)
                                )
                            elif typ.is_nullable():
                                self.write(
                                    "::pb_jelly::reflection::FieldMut::Value(self.%s.get_or_insert_with(::std::default::Default::default))"
                                    % escape_name(field.name)
                                )
                            else:
                                self.write(
                                    "::pb_jelly::reflection::FieldMut::Value(&mut self.%s)"
                                    % escape_name(field.name)
                                )
                    with block(self, "_ =>"):
                        self.write('panic!("unknown field name given")')


def walk(proto: FileDescriptorProto) -> WalkRet:
    enums, messages = [], []

    def _walk(
        proto: ProtoTypes, parents: List[Text], scl_prefix: SourceCodeLocation
    ) -> None:
        if isinstance(proto, EnumDescriptorProto):
            enums.append((parents, proto, scl_prefix))
        elif isinstance(proto, DescriptorProto):
            messages.append((parents, proto, scl_prefix))

            for i, nested_enum in enumerate(proto.enum_type):
                etfn = DescriptorProto.ENUM_TYPE_FIELD_NUMBER
                _walk(nested_enum, parents + [proto.name], scl_prefix + [etfn, i])

            for i, nested_message in enumerate(proto.nested_type):
                ntfn = DescriptorProto.NESTED_TYPE_FIELD_NUMBER
                _walk(nested_message, parents + [proto.name], scl_prefix + [ntfn, i])
        elif isinstance(proto, FileDescriptorProto):
            for i, enum_type in enumerate(proto.enum_type):
                etfn = FileDescriptorProto.ENUM_TYPE_FIELD_NUMBER
                _walk(enum_type, parents, scl_prefix + [etfn, i])

            for i, message_type in enumerate(proto.message_type):
                mtfn = FileDescriptorProto.MESSAGE_TYPE_FIELD_NUMBER
                _walk(message_type, parents, scl_prefix + [mtfn, i])

    _walk(proto, [], [])
    return enums, messages


M = TypeVar("M", DescriptorProto, EnumDescriptorProto)


class ProtoType(Generic[M]):
    def __init__(
        self, ctx: "Context", proto_file: FileDescriptorProto, path: List[Text], typ: M
    ) -> None:
        self.ctx = ctx
        self.proto_file = proto_file
        self.path = path  # inside proto file
        self.typ: M = typ
        self.crate, self.mod_parts = ctx.crate_from_proto_filename(proto_file.name)

    def __repr__(self) -> str:
        return "{} {} {} {}".format(
            self.proto_file.package, self.proto_file.name, self.path, self.typ.name
        )

    def proto_name(self) -> Text:
        """Name as used by proto eg. .mp.BlockMetadata.CompressionFormat"""
        r = "." + ".".join(self.path + [self.typ.name])
        if self.proto_file.package != "":
            r = "." + self.proto_file.package + r
        return r

    def rust_name(self, other_crate: Text, other_mod_parts: List[Text]) -> Text:
        """Name as used in rust code for proto_file"""
        if self.ctx.crate_from_proto_filename(self.proto_file.name) == (
            other_crate,
            other_mod_parts,
        ):
            # In same rust binary, directly use typename
            return "_".join(self.path + [self.typ.name])

        mod_parts = self.mod_parts + ["_".join(self.path + [self.typ.name])]
        mod_parts = [escape_name(part) for part in mod_parts]
        if other_crate != self.crate:
            # Different crate. Insert crate name in fully qualified module.
            mod_parts.insert(0, "::" + self.crate)
        else:
            # Same crate. Use super::<module_name>::<local_type>
            num_supers = len(other_mod_parts)
            supers = "::".join(num_supers * ["super"])
            mod_parts.insert(0, supers)
        return "::".join(mod_parts)


class Impls(NamedTuple):
    impls_eq: bool
    impls_copy: bool
    may_use_grpc_slices: bool


def box_recursive_fields(types: Dict[Text, ProtoType[DescriptorProto]]) -> None:
    """
    Given message types, keyed by their `proto_name()`s, detect recursive fields
    that would otherwise cause an infinite-size type and add the `box_it` extension to them.
    """
    scc: StronglyConnectedComponents[Text] = StronglyConnectedComponents()

    def edges(type_name: Text) -> List[Text]:
        field_type = types[type_name]
        return [
            field.type_name
            for field in field_type.typ.field
            if field.type == FieldDescriptorProto.TYPE_MESSAGE
            and field.type_name in types
            and field.label != FieldDescriptorProto.LABEL_REPEATED
            and not field.options.Extensions[extensions_pb2.box_it]
        ]

    def handle_scc(type_scc: Set[Text]) -> None:
        # For simplicity, apply box_it to all edges within the SCC.
        # Not all edges (i.e. fields) need to be boxed - just enough to cut the SCC -
        # but deciding which to box would be unintuitive and possibly not deterministic.
        for type_name in type_scc:
            field_type = types[type_name]
            for field in field_type.typ.field:
                if (
                    field.type == FieldDescriptorProto.TYPE_MESSAGE
                    and field.type_name in type_scc
                    and field.label != FieldDescriptorProto.LABEL_REPEATED
                ):
                    field.options.Extensions[extensions_pb2.box_it] = True

    for type_name in types:
        scc.process(type_name, edges, handle_scc)


class Context(object):
    def __init__(self, crate_per_dir: bool, prefix_to_clear: Text) -> None:
        self.proto_types: Dict[
            Text, Union[ProtoType[DescriptorProto], ProtoType[EnumDescriptorProto]]
        ] = {}

        # Set iteration order is nondeterministic, but this is ok, because we can
        # emit crates in any order
        self.deps_map: DefaultDict[Text, Set[Text]] = defaultdict(set)
        self.extra_crates: DefaultDict[Text, Set[Text]] = defaultdict(set)

        # Map from msg.proto_name() => cached impls
        # We have to build this on the fly as we process the types.
        self.impls_by_msg: Dict[Text, Impls] = {}
        self.scc: StronglyConnectedComponents[Text] = StronglyConnectedComponents()

        # Indiciator if every directory should be their own crate.
        self.crate_per_dir = crate_per_dir

        # Prefix of the crate path which should be cleared before making a determination
        # of how to split the crates apart.
        self.prefix_to_clear = prefix_to_clear

    def calc_impls(
        self,
        types: Set[Text],
    ) -> None:
        impls_eq = True
        impls_copy = True
        may_use_grpc_slices = False

        for type_name in types:
            msg_type = self.find(type_name)
            assert isinstance(msg_type.typ, DescriptorProto)

            crate, _ = self.crate_from_proto_filename(msg_type.proto_file.name)

            if msg_type.typ.options.Extensions[extensions_pb2.preserve_unrecognized]:
                impls_copy = False  # Preserve unparsed has a Vec which is not Copy

            for field in msg_type.typ.field:
                typ = field.type
                rust_type = RustType(self, msg_type.proto_file, msg_type.typ, field)
                if rust_type.has_custom_type():
                    self.extra_crates[crate].update(
                        CRATE_NAME_REGEX.findall(rust_type.custom_type())
                    )
                    may_use_grpc_slices = True

                if field.type_name:
                    field_type = self.find(field.type_name)
                    if crate in self.deps_map:
                        dep_crate, _ = self.crate_from_proto_filename(
                            field_type.proto_file.name
                        )
                        if dep_crate != crate:
                            self.deps_map[crate].add(dep_crate)

                if field.label == FieldDescriptorProto.LABEL_REPEATED:
                    impls_copy = False  # Vecs are not Copy.

                # If we use a Blob type, or GRPC Slice
                if typ == FieldDescriptorProto.TYPE_BYTES and (
                    field.options.ctype == FieldOptions.CORD
                    or field.options.Extensions[extensions_pb2.grpc_slices]
                ):
                    (impls_eq, impls_copy) = (False, False)  # Blob is not eq/copy
                    self.extra_crates[crate].add("blob_pb")
                    may_use_grpc_slices = True
                # If we use a Bytes type
                elif (
                    typ == FieldDescriptorProto.TYPE_BYTES
                    and field.options.Extensions[extensions_pb2.zero_copy]
                ):
                    (impls_eq, impls_copy) = (False, False)
                    self.extra_crates[crate].add("bytes")
                    may_use_grpc_slices = True
                elif typ in PRIMITIVE_TYPES:
                    if not PRIMITIVE_TYPES[typ][1]:
                        impls_eq = False
                    if not PRIMITIVE_TYPES[typ][2]:
                        impls_copy = False
                elif typ == FieldDescriptorProto.TYPE_ENUM:
                    pass  # Enums are Eq / Copy
                elif typ == FieldDescriptorProto.TYPE_MESSAGE:
                    assert isinstance(field_type.typ, DescriptorProto)
                    if msg_type.typ.options.Extensions[
                        extensions_pb2.preserve_unrecognized
                    ]:
                        assert field_type.typ.options.Extensions[
                            extensions_pb2.preserve_unrecognized
                        ], (
                            "%s preserves unrecognized but child message %s does not"
                            % (
                                msg_type.proto_name(),
                                field.type,
                            )
                        )
                    if field.type_name not in types:
                        field_impls = self.impls_by_msg[field.type_name]
                        impls_eq = impls_eq and field_impls.impls_eq
                        impls_copy = impls_copy and field_impls.impls_copy
                        may_use_grpc_slices = (
                            may_use_grpc_slices or field_impls.may_use_grpc_slices
                        )

                    if rust_type.is_boxed():
                        impls_copy = False
                else:
                    raise RuntimeError(
                        "Unsupported type: {!r}".format(
                            FieldDescriptorProto.Type.Name(typ)
                        )
                    )

        for type_name in types:
            self.impls_by_msg[type_name] = Impls(
                impls_eq=impls_eq,
                impls_copy=impls_copy,
                may_use_grpc_slices=may_use_grpc_slices,
            )

    def feed(self, proto_file: FileDescriptorProto, to_generate: List[Text]) -> None:
        enums, messages = walk(proto_file)

        for name in to_generate:
            crate, _ = self.crate_from_proto_filename(name)
            self.deps_map[crate]

        for enum_path, enum_typ, _ in enums:
            enum_pt = ProtoType(self, proto_file, enum_path, enum_typ)
            self.proto_types[enum_pt.proto_name()] = enum_pt

        message_types: Dict[Text, ProtoType[DescriptorProto]] = {}

        for path, typ, _ in messages:
            msg_pt = ProtoType(self, proto_file, path, typ)
            self.proto_types[msg_pt.proto_name()] = msg_pt
            message_types[msg_pt.proto_name()] = msg_pt

        # Note that there can't be mutual recursion across files,
        # so it suffices to examine one file at a time for the purposes of `box_recursive_fields`
        box_recursive_fields(message_types)

        for path, typ, _ in messages:
            msg_pt = ProtoType(self, proto_file, path, typ)

            def edges(type_name: Text) -> List[Text]:
                field_type = self.find(type_name)
                assert isinstance(field_type.typ, DescriptorProto)
                return [
                    field.type_name
                    for field in field_type.typ.field
                    if field.type == FieldDescriptorProto.TYPE_MESSAGE
                ]

            self.scc.process(msg_pt.proto_name(), edges, self.calc_impls)

    def find_enum(self, typename: Text) -> ProtoType[EnumDescriptorProto]:
        pt = self.find(typename)
        assert isinstance(pt.typ, EnumDescriptorProto)
        return pt

    def find_msg(self, typename: Text) -> ProtoType[DescriptorProto]:
        pt = self.find(typename)
        assert isinstance(pt.typ, DescriptorProto)
        return pt

    def find(
        self, typename: Text
    ) -> Union[ProtoType[DescriptorProto], ProtoType[EnumDescriptorProto]]:
        if typename in self.proto_types:
            return self.proto_types[typename]

        raise RuntimeError("Could not find type by proto name: {}".format(typename))

    def get_lib_and_mod_rs(
        self, mod_tree: ModTree, derive_serde: bool
    ) -> Iterator[Tuple[Text, Text]]:
        for crate, deps in self.deps_map.items():
            librs = CodeWriter(None, None, None, None)  # type: ignore
            librs.write("#[macro_use]")
            librs.write("extern crate lazy_static;")
            if derive_serde:
                librs.write("#[macro_use]")
                librs.write("extern crate serde;")
            librs.write("")

            def mod_tree_dfs(
                mod_prefix_path: Text, sub_mod_tree: ModTree
            ) -> Iterator[Tuple[Text, Text]]:
                if not sub_mod_tree:
                    return

                filename = mod_prefix_path + "/mod.rs"
                content = "\n".join(
                    ["// @" + "generated, do not edit", ""]
                    + [
                        "pub mod %s;" % escape_name(mod)
                        for mod in sorted(sub_mod_tree.keys())
                    ]
                    + [""]
                )
                yield filename, content

                for child_mod_name, child_mod_tree in sub_mod_tree.items():
                    for res in mod_tree_dfs(
                        mod_prefix_path + "/" + child_mod_name, child_mod_tree
                    ):
                        yield res

            crate_mod_tree: ModTree = mod_tree[crate]
            for mod_name, child_mod_tree in sorted(crate_mod_tree.items()):
                librs.write("pub mod %s;" % escape_name(mod_name))

                for res in mod_tree_dfs(crate + "/src/" + mod_name, child_mod_tree):
                    yield res

            filename = crate + "/src/lib.rs"
            content = RS_HEADER + LIB_RS_HEADER + librs.content.getvalue()
            yield filename, content

    def get_spec_toml_file(
        self, derive_serde: bool, include_sso: bool
    ) -> Iterator[Tuple[Text, Text]]:
        for crate, deps in self.deps_map.items():
            all_deps = (
                {"lazy_static", "pb-jelly"} | deps | self.extra_crates[crate]
            ) - {"std"}
            features = {
                "serde": 'features=["serde_derive"]',
                "compact_str": 'features=["bytes"]',
            }

            if derive_serde:
                all_deps.update({"serde"})
            if include_sso:
                all_deps.update({"compact_str"})
                if derive_serde:
                    features.update({"compact_str": 'features=["bytes", "serde"]'})

            deps_str = "\n".join(
                "{dep} = {{{feat}}}".format(dep=dep, feat=features.get(dep, ""))
                for dep in sorted(all_deps)
            )
            targets = SPEC_TOML_TEMPLATE.format(crate=crate, deps=deps_str)
            yield crate, targets

    def get_cargo_toml_file(
        self, derive_serde: bool, include_sso: bool
    ) -> Iterator[Tuple[Text, Text]]:
        for crate, deps in self.deps_map.items():
            all_deps = (
                {"lazy_static", "pb-jelly"} | deps | self.extra_crates[crate]
            ) - {"std"}
            features = {
                "serde": 'features=["serde_derive"]',
                "compact_str": 'features=["bytes"]',
            }

            if derive_serde:
                all_deps.update({"serde"})
            if include_sso:
                all_deps.update({"compact_str"})
                if derive_serde:
                    features.update({"compact_str": 'features=["bytes", "serde"]'})

            versions = {
                "lazy_static": ' version = "1.4.0" ',
                "pb-jelly": ' version = "0.0.14" ',
                "serde": ' version = "1.0" ',
                "bytes": ' version = "1.0" ',
                "compact_str": ' version = "0.5" ',
            }

            deps_lines = []
            for dep in sorted(all_deps):
                fields = []
                if dep in features:
                    fields.append(features[dep])
                if dep in versions:
                    fields.append(versions[dep])
                else:
                    fields.append('path = "../{dep}"'.format(dep=dep))
                deps_lines.append(
                    "{dep} = {{{fields}}}".format(dep=dep, fields=",".join(fields))
                )

            targets = CARGO_TOML_TEMPLATE.format(
                crate=crate, deps="\n".join(deps_lines)
            )
            yield crate, targets

    def crate_from_proto_filename(
        self, proto_filename: Text
    ) -> Tuple[Text, List[Text]]:
        filename = proto_filename.replace(self.prefix_to_clear, "").replace(
            ".proto", ""
        )
        mod_parts = filename.split("/")
        # Each proto module will become its own crate. We append "_proto" to the crate name
        # to disambiguate the top level crate names.
        if self.crate_per_dir:
            crate_name = "proto_" + "_".join(mod_parts[:-1])
            return crate_name, [mod_parts[-1]]
        crate_name = "proto_" + mod_parts[0]
        return crate_name, mod_parts[1:]


SPEC_TOML_TEMPLATE = (
    """# @"""
    + """generated, do not edit
[package]
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

RS_HEADER = "// @" + "generated, do not edit\n"

LIB_RS_HEADER = """
#![warn(rust_2018_idioms)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(rustdoc::broken_intra_doc_links)]

// Modules are generated based on the naming conventions of protobuf, which might cause "module inception"
#![allow(clippy::module_inception)]
// This is all generated code, so "manually" implementing derivable impls is okay
#![allow(clippy::derivable_impls)]
// For enums with many variants, the matches!(...) macro isn't obviously better
#![allow(clippy::match_like_matches_macro)]
// TODO: Ideally we don't allow this
#![allow(clippy::option_as_ref_deref)]
// TODO: Ideally we don't allow this
#![allow(clippy::match_single_binding)]

"""


def generate_single_crate(
    ctx: Context,
    file_prefix: Text,
    file_to_generate: List[Text],
    request: plugin.CodeGeneratorRequest,
    response: plugin.CodeGeneratorResponse,
) -> None:
    def new_mod_tree() -> ModTree:
        return defaultdict(new_mod_tree)

    mod_tree = new_mod_tree()

    # Set iteration order is nondeterministic, but this is ok, because we never iterate through this
    processed_files: Set[Text] = set()
    derive_serde = False
    include_sso = False

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
        mod_name = mod_parts[-1] if mod_parts else crate_name

        def add_mod(writer: CodeWriter) -> None:
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
            writer.gen_msg(path, msg_typ, scl)
            writer.write("")

        add_mod(writer=writer)

        # check if the writer ever used a small string optimization
        if writer.uses_sso:
            include_sso = True

    # Note that output filenames must use "/" even on windows. It is part of the
    # protoc plugin protocol. The plugin speaks os-independent in "/". Thus, we
    # should not use os.path.sep or os.path.join

    for filename, content in ctx.get_lib_and_mod_rs(mod_tree, derive_serde):
        output = response.file.add()
        output.name = file_prefix + filename
        output.content = content

    if "generate_build_files" in request.parameter:
        for crate in ctx.deps_map:
            # Create a stub file for later generation
            output = response.file.add()
            output.name = file_prefix + crate + "/BUILD.in-gen-proto~"
            output.content = ""
    elif "generate_spec_toml" in request.parameter:
        for crate, spec_toml_file in ctx.get_spec_toml_file(derive_serde, include_sso):
            output = response.file.add()
            output.name = file_prefix + crate + "/Spec.toml"
            output.content = spec_toml_file
    else:
        # Generate good ol Cargo.toml files
        for crate, cargo_toml_file in ctx.get_cargo_toml_file(
            derive_serde, include_sso
        ):
            output = response.file.add()
            output.name = file_prefix + crate + "/Cargo.toml"
            output.content = cargo_toml_file


def generate_code(
    request: plugin.CodeGeneratorRequest, response: plugin.CodeGeneratorResponse
) -> None:
    to_generate = list(request.file_to_generate)

    prefix_to_clear = ""
    if "prefix_to_clear" in request.parameter:
        prefix_to_clear = [
            arg for arg in request.parameter.split() if "prefix_to_clear" in arg
        ][0].split("=")[1]

    if "crate_per_dir" in request.parameter:
        files_by_dir: DefaultDict[Text, List[Text]] = defaultdict(list)
        for file_path in to_generate:
            dir_path, file_name = os.path.split(file_path)
            files_by_dir[dir_path].append(file_path)

        for dir_path, to_generate in sorted(files_by_dir.items()):
            file_prefix = dir_path.replace(prefix_to_clear, "").split("/")[0] + "/"
            ctx = Context(crate_per_dir=True, prefix_to_clear=prefix_to_clear)
            for proto_file in request.proto_file:
                ctx.feed(proto_file, to_generate)
            generate_single_crate(ctx, file_prefix, to_generate, request, response)
    else:
        ctx = Context(crate_per_dir=False, prefix_to_clear=prefix_to_clear)
        for proto_file in request.proto_file:
            ctx.feed(proto_file, to_generate)
        generate_single_crate(ctx, "", to_generate, request, response)


def main() -> None:
    # Read request message from stdin
    data = sys.stdin.buffer.read()

    # Parse request
    request = plugin.CodeGeneratorRequest()
    request.ParseFromString(data)

    # Create response
    response = plugin.CodeGeneratorResponse()
    response.supported_features = (
        plugin.CodeGeneratorResponse.Feature.FEATURE_PROTO3_OPTIONAL
    )

    # Generate code
    generate_code(request, response)

    # Serialise response message
    output = response.SerializeToString()

    # Write to stdout
    sys.stdout.buffer.write(output)


if __name__ == "__main__":
    main()
