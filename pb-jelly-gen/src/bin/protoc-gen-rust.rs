use std::cell::RefCell;
use std::fmt::Write as _;
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    io::{self, Read, Write},
    iter,
};

use indexmap::{IndexMap, IndexSet};
use lazy_static::lazy_static;
use pb_jelly::{extensions::Extensible, Message};
use proto_google::protobuf::compiler::plugin;
use proto_google::protobuf::descriptor::{
    DescriptorProto, EnumDescriptorProto, EnumValueDescriptorProto, FieldDescriptorProto, FieldDescriptorProto_Label,
    FieldDescriptorProto_Type, FieldOptions, FieldOptions_CType, FileDescriptorProto, OneofDescriptorProto,
    SourceCodeInfo_Location,
};
use proto_rust::extensions::{self, PRESERVE_UNRECOGNIZED};
use regex::Regex;

struct PrimitiveType {
    rust_type: &'static str,
    impls_eq: bool,
    impls_copy: bool,
}

fn get_primitive_type(field_type: FieldDescriptorProto_Type) -> Option<PrimitiveType> {
    Some(match field_type {
        FieldDescriptorProto_Type::TYPE_FLOAT => PrimitiveType {
            rust_type: "f32",
            impls_eq: false,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_DOUBLE => PrimitiveType {
            rust_type: "f64",
            impls_eq: false,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_INT32 => PrimitiveType {
            rust_type: "i32",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_INT64 => PrimitiveType {
            rust_type: "i64",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_UINT32 => PrimitiveType {
            rust_type: "u32",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_UINT64 => PrimitiveType {
            rust_type: "u64",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_SINT32 => PrimitiveType {
            rust_type: "::pb_jelly::Signed32",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_SINT64 => PrimitiveType {
            rust_type: "::pb_jelly::Signed64",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_FIXED32 => PrimitiveType {
            rust_type: "::pb_jelly::Fixed32",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_FIXED64 => PrimitiveType {
            rust_type: "::pb_jelly::Fixed64",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_SFIXED32 => PrimitiveType {
            rust_type: "::pb_jelly::Sfixed32",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_SFIXED64 => PrimitiveType {
            rust_type: "::pb_jelly::Sfixed64",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_BOOL => PrimitiveType {
            rust_type: "bool",
            impls_eq: true,
            impls_copy: true,
        },
        FieldDescriptorProto_Type::TYPE_STRING => PrimitiveType {
            rust_type: "::std::string::String",
            impls_eq: true,
            impls_copy: false,
        },
        FieldDescriptorProto_Type::TYPE_BYTES => PrimitiveType {
            rust_type: "::std::vec::Vec<u8>",
            impls_eq: true,
            impls_copy: false,
        },
        _ => return None,
    })
}

static BLOB_TYPE: &str = "::pb_jelly::Lazy<::blob_pb::WrappedBlob>";
static VEC_SLICE_TYPE: &str = "::pb_jelly::Lazy<::blob_pb::VecSlice>";
static LAZY_BYTES_TYPE: &str = "::pb_jelly::Lazy<::bytes::Bytes>";
static SMALL_STRING_OPT_TYPE: &str = "::compact_str::CompactString";

lazy_static! {
    static ref CRATE_NAME_REGEX: Regex = Regex::new(r"(?:^|\W)::(\w+)(?:::\w+)*").unwrap();
}

/// Keywords in rust which cannot be module names.
const RESERVED_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "Self", "self", "static", "struct", "super",
    "trait", "true", "type", "unsafe", "use", "where", "while", "abstract", "alignof", "become", "box", "do", "final",
    "macro", "offsetof", "override", "priv", "proc", "pure", "sizeof", "typeof", "unsized", "virtual", "yield",
];

type SourceCodeLocation = Vec<i32>;
#[derive(Default)]
struct ModTree(BTreeMap<String, ModTree>);

fn camelcase(underscored: &str) -> String {
    underscored
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}
struct RustType<'a> {
    ctx: &'a Context<'a>,
    proto_file: &'a FileDescriptorProto,
    msg_type: Option<&'a DescriptorProto>,
    field: &'a FieldDescriptorProto,
    is_proto3: bool,
    oneof: Option<&'a OneofDescriptorProto>,
}

impl<'a> RustType<'a> {
    fn new(
        ctx: &'a Context,
        proto_file: &'a FileDescriptorProto,
        msg_type: Option<&'a DescriptorProto>,
        field: &'a FieldDescriptorProto,
    ) -> Self {
        let is_proto3 = proto_file.syntax == Some("proto3".to_string());
        let oneof = if field.has_oneof_index() && !field.get_proto3_optional() && msg_type.is_some() {
            Some(&msg_type.unwrap().get_oneof_decl()[field.get_oneof_index() as usize])
        } else {
            None
        };

        RustType {
            ctx,
            proto_file,
            msg_type,
            field,
            is_proto3,
            oneof,
        }
    }

    fn default(&self, msg_name: &str) -> String {
        if let Some(oneof) = self.oneof {
            if oneof_nullable(oneof) {
                return "None".to_string();
            } else {
                return self.oneof_val(msg_name, "::std::default::Default::default()");
            }
        }

        // Proto 3 doesn't have configurable default values.
        if !self.is_proto3 {
            if let Some(ref default_value) = self.field.default_value {
                if self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_STRING) {
                    return format!("Some(\"{}\".into())", default_value);
                }

                if self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_BYTES) {
                    return format!("Some(b\"{}\".to_vec())", default_value);
                }

                if let Some(primitive) = self.field.type_.and_then(get_primitive_type) {
                    let typ_name = primitive.rust_type;
                    if typ_name.contains("::pb") {
                        return format!("Some({}({}))", typ_name, default_value);
                    }
                    if typ_name.starts_with("f") && !default_value.contains(".") {
                        return format!("Some({}.)", default_value);
                    }
                    return format!("Some({})", default_value);
                }

                if self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_ENUM) {
                    let proto_type = self.ctx.find(self.field.get_type_name());
                    let (crate_, mod_parts) = self.ctx.crate_from_proto_filename(self.proto_file.get_name());
                    let value = format!(
                        "{}::{}",
                        proto_type.rust_name(self.ctx, &crate_, &mod_parts),
                        default_value
                    );
                    return format!("Some({})", value);
                }

                panic!(
                    "Default not supported on field {:?} of type {:?}",
                    self.field.get_name(),
                    self.field.type_
                );
            }
        }

        "::std::default::Default::default()".to_string()
    }

    fn is_length_delimited(&self) -> bool {
        matches!(
            self.field.type_,
            Some(FieldDescriptorProto_Type::TYPE_MESSAGE)
                | Some(FieldDescriptorProto_Type::TYPE_STRING)
                | Some(FieldDescriptorProto_Type::TYPE_BYTES)
        )
    }
    fn wire_format(&self) -> &str {
        if self.is_length_delimited() {
            return "LengthDelimited";
        }

        match self.field.type_ {
            Some(FieldDescriptorProto_Type::TYPE_DOUBLE)
            | Some(FieldDescriptorProto_Type::TYPE_FIXED64)
            | Some(FieldDescriptorProto_Type::TYPE_SFIXED64) => "Fixed64",
            Some(FieldDescriptorProto_Type::TYPE_FLOAT)
            | Some(FieldDescriptorProto_Type::TYPE_FIXED32)
            | Some(FieldDescriptorProto_Type::TYPE_SFIXED32) => "Fixed32",
            _ => "Varint",
        }
    }

    fn is_grpc_slices(&self) -> bool {
        return self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_BYTES)
            && self
                .field
                .get_options()
                .get_extension(extensions::GRPC_SLICES)
                .unwrap()
                .unwrap_or(false);
    }

    fn is_blob(&self) -> bool {
        return self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_BYTES)
            && matches!(
                self.field.options,
                Some(FieldOptions {
                    ctype: Some(FieldOptions_CType::CORD),
                    ..
                })
            );
    }

    fn is_lazy_bytes(&self) -> bool {
        return self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_BYTES)
            && self
                .field
                .get_options()
                .get_extension(extensions::ZERO_COPY)
                .unwrap()
                .unwrap_or(false);
    }

    fn is_small_string_optimization(&self) -> bool {
        return self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_STRING)
            && self
                .field
                .get_options()
                .get_extension(extensions::SSO)
                .unwrap()
                .unwrap_or(false);
    }

    fn is_boxed(&self) -> bool {
        return self.field.type_ == Some(FieldDescriptorProto_Type::TYPE_MESSAGE)
            && self
                .field
                .get_options()
                .get_extension(extensions::BOX_IT)
                .unwrap()
                .unwrap_or(false);
    }

    fn has_custom_type(&self) -> bool {
        self.field
            .get_options()
            .get_extension(extensions::TYPE)
            .unwrap()
            .is_some()
    }

    fn custom_type(&self) -> Option<String> {
        self.field.get_options().get_extension(extensions::TYPE).unwrap()
    }

    fn is_nullable(&self) -> bool {
        if self.oneof.is_some() {
            return false;
        }
        if get_primitive_type(self.field.get_type_()).is_some() && self.is_proto3 && !self.field.get_proto3_optional() {
            // Primitive types in proto3 are not nullable by default;
            // if missing, they are decoded as 0-value ("" or 0).
            // proto3_optional overrides this and treats those fields like 1-variant oneofs on the wire,
            // enabling them to be truly optional
            return false;
        }
        if let Some(nullable_field) = self
            .field
            .get_options()
            .get_extension(extensions::NULLABLE_FIELD)
            .unwrap()
        {
            // We still allow overriding nullability as an extension
            return nullable_field;
        }
        return !self.is_proto3
            || self.field.get_type_() == FieldDescriptorProto_Type::TYPE_MESSAGE
            || self.field.get_proto3_optional();
    }

    fn is_empty_oneof_field(&self) -> bool {
        assert!(self.oneof.is_some());
        return self.field.type_name.as_deref() == Some(".google.protobuf.Empty") && !self.is_boxed();
    }

    fn can_be_packed(&self) -> bool {
        // Return true if incoming messages could be packed on the wire
        return self.field.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED
            && matches!(self.wire_format(), "Varint" | "Fixed64" | "Fixed32");
    }

    fn should_serialize_packed(&self) -> bool {
        self.can_be_packed() && (self.field.get_options().get_packed() || self.is_proto3)
    }

    fn is_repeated(&self) -> bool {
        self.field.label == Some(FieldDescriptorProto_Label::LABEL_REPEATED)
    }

    fn set_method(&self) -> (String, String) {
        assert!(!self.is_repeated());
        match self.field.type_.unwrap() {
            FieldDescriptorProto_Type::TYPE_FLOAT => ("f32".to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_DOUBLE => ("f64".to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_INT32 => ("i32".to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_INT64 => ("i64".to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_UINT32 => ("u32".to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_UINT64 => ("u64".to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_SINT32 => ("i32".to_string(), "::pb_jelly::Signed32(v)".to_string()),
            FieldDescriptorProto_Type::TYPE_SINT64 => ("i64".to_string(), "::pb_jelly::Signed64(v)".to_string()),
            FieldDescriptorProto_Type::TYPE_FIXED64 => ("u64".to_string(), "::pb_jelly::Fixed64(v)".to_string()),
            FieldDescriptorProto_Type::TYPE_SFIXED64 => ("i64".to_string(), "::pb_jelly::Sfixed64(v)".to_string()),
            FieldDescriptorProto_Type::TYPE_FIXED32 => ("u32".to_string(), "::pb_jelly::Fixed32(v)".to_string()),
            FieldDescriptorProto_Type::TYPE_SFIXED32 => ("i32".to_string(), "::pb_jelly::Sfixed32(v)".to_string()),
            FieldDescriptorProto_Type::TYPE_BOOL => ("bool".to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_STRING => {
                if self.is_small_string_optimization() {
                    (SMALL_STRING_OPT_TYPE.to_string(), "v".to_string())
                } else {
                    ("::std::string::String".to_string(), "v".to_string())
                }
            },
            FieldDescriptorProto_Type::TYPE_BYTES => {
                if self.is_blob() {
                    (BLOB_TYPE.to_string(), "v".to_string())
                } else if self.is_grpc_slices() {
                    (VEC_SLICE_TYPE.to_string(), "v".to_string())
                } else if self.is_lazy_bytes() {
                    (LAZY_BYTES_TYPE.to_string(), "v".to_string())
                } else {
                    ("::std::vec::Vec<u8>".to_string(), "v".to_string())
                }
            },
            FieldDescriptorProto_Type::TYPE_ENUM => (self.rust_type().to_string(), "v".to_string()),
            FieldDescriptorProto_Type::TYPE_MESSAGE => {
                if self.is_boxed() {
                    (
                        "::std::boxed::Box<".to_string() + &self.rust_type() + ">",
                        "v".to_string(),
                    )
                } else {
                    (self.rust_type().to_string(), "v".to_string())
                }
            },
            _ => panic!("Unexpected field type"),
        }
    }
    fn take_method(&self) -> (Option<String>, Option<String>) {
        assert!(!self.is_repeated());
        let expr = format!("self.{}.take().unwrap_or_default()", self.field.get_name());

        match self.field.type_ {
            Some(FieldDescriptorProto_Type::TYPE_STRING) => {
                if self.is_small_string_optimization() {
                    return (Some(SMALL_STRING_OPT_TYPE.to_string()), Some(expr));
                } else {
                    return (Some("::std::string::String".to_string()), Some(expr));
                }
            },
            Some(FieldDescriptorProto_Type::TYPE_BYTES) => {
                if self.is_blob() {
                    return (Some(BLOB_TYPE.to_string()), Some(expr));
                } else if self.is_grpc_slices() {
                    return (Some(VEC_SLICE_TYPE.to_string()), Some(expr));
                } else if self.is_lazy_bytes() {
                    return (Some(LAZY_BYTES_TYPE.to_string()), Some(expr));
                } else {
                    return (Some("::std::vec::Vec<u8>".to_string()), Some(expr));
                }
            },
            Some(FieldDescriptorProto_Type::TYPE_MESSAGE) => {
                if self.is_boxed() {
                    return (Some(format!("::std::boxed::Box<{}>", self.rust_type())), Some(expr));
                } else {
                    return (Some(self.rust_type()), Some(expr));
                }
            },
            _ => return (None, None),
        }
    }

    fn get_method(&self) -> (String, String) {
        assert!(!self.is_repeated());
        let name = self.field.get_name();

        match self.field.type_ {
            Some(FieldDescriptorProto_Type::TYPE_FLOAT) => {
                return ("f32".to_string(), format!("self.{}.unwrap_or(0.)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_DOUBLE) => {
                return ("f64".to_string(), format!("self.{}.unwrap_or(0.)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_INT32) => {
                return ("i32".to_string(), format!("self.{}.unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_INT64) => {
                return ("i64".to_string(), format!("self.{}.unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_UINT32) => {
                return ("u32".to_string(), format!("self.{}.unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_UINT64) => {
                return ("u64".to_string(), format!("self.{}.unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_SINT32) => {
                return ("i32".to_string(), format!("self.{}.map(|v| v.0).unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_SINT64) => {
                return ("i64".to_string(), format!("self.{}.map(|v| v.0).unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_FIXED64) => {
                return ("u64".to_string(), format!("self.{}.map(|v| v.0).unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_SFIXED64) => {
                return ("i64".to_string(), format!("self.{}.map(|v| v.0).unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_FIXED32) => {
                return ("u32".to_string(), format!("self.{}.map(|v| v.0).unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_SFIXED32) => {
                return ("i32".to_string(), format!("self.{}.map(|v| v.0).unwrap_or(0)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_BOOL) => {
                return ("bool".to_string(), format!("self.{}.unwrap_or(false)", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_STRING) => {
                return ("&str".to_string(), format!("self.{}.as_deref().unwrap_or(\"\")", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_BYTES) => {
                assert!(
                    !self.is_blob() || self.is_grpc_slices() || self.is_lazy_bytes(),
                    "Can't generate get method for lazy field"
                );
                return ("&[u8]".to_string(), format!("self.{}.as_deref().unwrap_or(&[])", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_ENUM) => {
                return (self.rust_type().clone(), format!("self.{}.unwrap_or_default()", name));
            },
            Some(FieldDescriptorProto_Type::TYPE_MESSAGE) => {
                let deref = if !self.is_boxed() {
                    ""
                } else {
                    ".map(::std::ops::Deref::deref)"
                };
                return (
                    format!("&{}", self.rust_type()),
                    format!(
                        "self.{}.as_ref(){} .unwrap_or(&{}_default)",
                        name,
                        deref,
                        self.rust_type()
                    ),
                );
            },
            _ => panic!("Unexpected field type"),
        }
    }

    fn rust_type(&self) -> String {
        let typ = self.field.get_type_();

        if let Some(rust_type) = self.custom_type() {
            return rust_type.to_string();
        }

        if self.is_blob() {
            return BLOB_TYPE.to_string();
        }

        if self.is_grpc_slices() {
            return VEC_SLICE_TYPE.to_string();
        }

        if self.is_lazy_bytes() {
            return LAZY_BYTES_TYPE.to_string();
        }

        if self.is_small_string_optimization() {
            return SMALL_STRING_OPT_TYPE.to_string();
        }

        if let Some(prim) = get_primitive_type(typ) {
            return prim.rust_type.to_string();
        }

        if typ == FieldDescriptorProto_Type::TYPE_MESSAGE || typ == FieldDescriptorProto_Type::TYPE_ENUM {
            if !self.field.get_type_name().starts_with(".") {
                panic!("We only support fully qualified type names");
            }

            let proto_type = self.ctx.find(&self.field.get_type_name());
            let (crate_, mod_parts) = self.ctx.crate_from_proto_filename(self.proto_file.get_name());
            return proto_type.rust_name(self.ctx, &crate_, &mod_parts).to_string();
        }

        panic!("Unsupported type: {:?}", typ);
    }

    fn storage_type(&self) -> String {
        let mut rust_type = self.rust_type().to_string();

        if self.is_boxed() {
            rust_type = format!("::std::boxed::Box<{}>", rust_type);
        }

        if self.is_repeated() {
            rust_type = format!("::std::vec::Vec<{}>", rust_type);
        } else if self.is_nullable() {
            rust_type = format!("::std::option::Option<{}>", rust_type);
        }

        rust_type
    }

    fn oneof_field_match(&self, var: &str) -> String {
        if self.is_empty_oneof_field() {
            return camelcase(self.field.get_name());
        } else {
            return format!("{}({})", camelcase(self.field.get_name()), var);
        }
    }

    fn oneof_val(&self, msg_name: &str, var: &str) -> String {
        let oneof = self.oneof.unwrap();

        let oneofv = format!("{}::{}", oneof_msg_name(msg_name, oneof), self.oneof_field_match(var));

        if oneof_nullable(oneof) {
            return format!("Some({})", oneofv);
        }

        oneofv
    }
}

fn oneof_msg_name(parent_msg_name: &str, oneof: &OneofDescriptorProto) -> String {
    format!("{}_{}", parent_msg_name, camelcase(&oneof.get_name()))
}

fn oneof_nullable(oneof: &OneofDescriptorProto) -> bool {
    !oneof
        .get_options()
        .get_extension(extensions::NULLABLE)
        .unwrap()
        .is_some()
        || oneof.get_options().get_extension(extensions::NULLABLE).unwrap() == Some(true)
}

fn enum_err_if_default_or_unknown(enum_: &EnumDescriptorProto) -> bool {
    enum_
        .get_options()
        .get_extension(extensions::ERR_IF_DEFAULT_OR_UNKNOWN)
        .unwrap()
        .is_some()
        && enum_
            .get_options()
            .get_extension(extensions::ERR_IF_DEFAULT_OR_UNKNOWN)
            .unwrap()
            == Some(true)
}

fn enum_closed(enum_: &EnumDescriptorProto) -> bool {
    enum_
        .get_options()
        .get_extension(extensions::CLOSED_ENUM)
        .unwrap()
        .is_some()
        && enum_.get_options().get_extension(extensions::CLOSED_ENUM).unwrap() == Some(true)
}

fn block_with<'a, 'ctx>(
    ctx: &mut CodeWriter<'a, 'ctx>,
    header: impl AsRef<str>,
    open: &str,
    close: &str,
    f: impl FnOnce(&mut CodeWriter<'a, 'ctx>),
) {
    ctx.write(format!("{}{}", header.as_ref(), open));
    ctx.indentation += 1;
    f(&mut *ctx);
    ctx.indentation -= 1;
    ctx.write(close);
}
fn block<'a, 'ctx>(ctx: &mut CodeWriter<'a, 'ctx>, header: impl AsRef<str>, f: impl FnOnce(&mut CodeWriter<'a, 'ctx>)) {
    block_with(ctx, header, " {", "}", f)
}

fn field_iter<'a, 'ctx, F>(
    ctx: &mut CodeWriter<'a, 'ctx>,
    var: &str,
    msg_name: &str,
    msg_type: &'a DescriptorProto,
    field: &'a FieldDescriptorProto,
    mut f: F,
) where
    F: FnMut(&mut CodeWriter<'a, 'ctx>),
{
    let typ = ctx.rust_type(Some(&msg_type), &field);

    if let Some(oneof) = typ.oneof {
        // For oneofs, we always emit, even if the primitive field is set to a 0 value
        // This is so we can distinguish which field of oneof is set.
        let oneof_val = typ.oneof_val(&msg_name, &format!("ref {}", var));
        block(
            &mut *ctx,
            &format!("if let {} = self.{}", oneof_val, oneof.get_name()),
            |ctx| {
                if typ.is_empty_oneof_field() {
                    ctx.write(&format!(
                        "let {}: &{} = &::std::default::Default::default();",
                        var,
                        typ.rust_type()
                    ));
                } else if typ.is_boxed() {
                    ctx.write(&format!("let {}: &{} = &**{};", var, typ.rust_type(), var));
                }
                f(ctx);
            },
        );
    } else if field.get_type_() == FieldDescriptorProto_Type::TYPE_MESSAGE && !typ.is_nullable() && !typ.is_repeated() {
        // Always emit messages explicitly marked as non-nullable
        let deref = if typ.is_boxed() { "*" } else { "" };
        block(ctx, "", |ctx| {
            ctx.write(&format!("let {} = &{}self.{};", var, deref, field.get_name()));
            f(ctx);
        });
    } else if field.get_type_() == FieldDescriptorProto_Type::TYPE_ENUM
        && !typ.is_repeated()
        && enum_err_if_default_or_unknown(ctx.ctx.find(field.get_type_name()).enum_typ())
    {
        // The default value (as considered by proto) doesn't appear in the generated enum and
        // doesn't map to .default(). All of the values that actually get generated need to get
        // encoded.
        ctx.write(&format!("let {} = &self.{};", var, field.get_name()));
        f(ctx);
    } else if !typ.is_nullable() && !typ.is_repeated() {
        // For proto3, we remove the Option for primitive fields.
        // We only run internal code if the primitive field is non-default for proto3
        // Rather than looping, we set the variable once and run the inner code once
        block(
            ctx,
            &format!(
                "if self.{} != <{} as ::std::default::Default>::default()",
                field.get_name(),
                typ.storage_type()
            ),
            |ctx| {
                if typ.is_boxed() {
                    ctx.write(&format!("let {} = &*self.{};", var, field.get_name()));
                } else {
                    ctx.write(&format!("let {} = &self.{};", var, field.get_name()));
                }
                f(ctx);
            },
        );
    } else {
        // This iterates through Vec and the Option<> type for optional fieldds
        block(ctx, format!("for {} in &self.{}", var, field.get_name()), |ctx| {
            if typ.is_boxed() {
                ctx.write(format!("let {var} = &**{var};"));
            }
            f(ctx)
        });
    }
}

struct CodeWriter<'a, 'ctx> {
    ctx: &'ctx Context<'a>,
    proto_file: &'a FileDescriptorProto,
    crate_name: &'a str,
    mod_parts: &'a [String],
    indentation: u32,
    content: String,
    is_proto3: bool,
    uses_sso: bool,
    derive_serde: bool,
    source_code_info_by_scl: HashMap<Vec<i32>, &'a SourceCodeInfo_Location>,
}

impl<'a, 'ctx> CodeWriter<'a, 'ctx> {
    fn new(
        ctx: &'ctx Context<'a>,
        proto_file: &'a FileDescriptorProto,
        crate_name: &'a str,
        mod_parts: &'a [String],
    ) -> CodeWriter<'a, 'ctx> {
        CodeWriter {
            ctx,
            proto_file,
            crate_name,
            mod_parts,
            indentation: 0,
            content: String::new(),
            is_proto3: proto_file.get_syntax() == "proto3",
            uses_sso: false,
            derive_serde: proto_file
                .get_options()
                .get_extension(extensions::SERDE_DERIVE)
                .unwrap()
                .unwrap_or(false),
            source_code_info_by_scl: proto_file
                .get_source_code_info()
                .location
                .iter()
                .map(|location| (location.path.clone(), location))
                .collect(),
        }
    }

    fn write(&mut self, s: impl ToString) {
        let s = s.to_string();
        if s.is_empty() {
            writeln!(&mut self.content);
            return;
        }

        for _ in 0..self.indentation {
            write!(&mut self.content, "  ").unwrap();
        }
        writeln!(&mut self.content, "{}", s);
    }

    fn write_line_broken_text_with_prefix(&mut self, text_block: &str, prefix: &str) {
        if text_block.is_empty() {
            return;
        }
        for l in text_block.trim_end().split('\n') {
            if !l.is_empty() {
                self.write(&format!("{}{}", prefix, l));
            } else {
                self.write("");
            }
        }
    }
    fn write_comments(&mut self, sci_loc: Option<&SourceCodeInfo_Location>) {
        if let Some(sci_loc) = sci_loc {
            for leading_detached_comment in &sci_loc.leading_detached_comments {
                self.write_line_broken_text_with_prefix(leading_detached_comment, "//");
                self.write("");
            }
            if let Some(leading_comments) = &sci_loc.leading_comments {
                self.write_line_broken_text_with_prefix(leading_comments, "///");
            }
            // Trailing comments also go in the header - to make sure it gets into the docstring
            if let Some(trailing_comments) = &sci_loc.trailing_comments {
                self.write_line_broken_text_with_prefix(trailing_comments, "///");
            }
        }
    }
    fn rust_type(&mut self, msg_type: Option<&'a DescriptorProto>, field: &'a FieldDescriptorProto) -> RustType<'ctx> {
        let rust_type = RustType::new(self.ctx, self.proto_file, msg_type, field);

        // checks if any of our types use a small string optimization
        if !self.uses_sso && rust_type.is_small_string_optimization() {
            self.uses_sso = true;
        }

        rust_type
    }

    /// Generate a closed enum
    fn gen_closed_enum(
        &mut self,
        name: &str,
        enum_variants: &[(i32, &EnumValueDescriptorProto)],
        scl: &SourceCodeLocation,
    ) {
        let ctx = self;
        ctx.write_comments(ctx.source_code_info_by_scl.get(scl).copied());
        if ctx.derive_serde {
            ctx.write("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Deserialize, Serialize)]");
        } else {
            ctx.write("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]");
        }
        ctx.write("#[repr(i32)]");
        block(ctx, format!("pub enum {}", name), |ctx| {
            for &(idx, value) in enum_variants {
                let vfn = EnumDescriptorProto::default()
                    .descriptor()
                    .unwrap()
                    .get_field("value")
                    .unwrap()
                    .number as i32;
                let mut scl2 = scl.clone();
                scl2.extend_from_slice(&[vfn, idx]);
                ctx.write_comments(ctx.source_code_info_by_scl.get(&scl2).copied());
                ctx.write(format!("{} = {},", value.get_name(), value.get_number()));
            }
        });

        block(ctx, format!("impl {}", name), |ctx| {
            ctx.write(format!(
                "pub const KNOWN_VARIANTS: [{}; {}] = [{}];",
                name,
                enum_variants.len(),
                enum_variants
                    .iter()
                    .map(|(_, value)| format!("{}::{}", name, value.get_name()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        });

        block(ctx, &format!("impl ::std::default::Default for {}", name), |ctx| {
            block(ctx, "fn default() -> Self", |ctx| {
                // It's not actually clear what to do in this case. We take the first-defined
                // value that isn't 0-valued.
                ctx.write(&format!("{}::{}", name, enum_variants[0].1.get_name()));
            });
        });

        block(ctx, &format!("impl From<{}> for i32", name), |ctx| {
            block(ctx, &format!("fn from(v: {}) -> i32", name), |ctx| {
                block(ctx, "match v", |ctx| {
                    for (_, value) in enum_variants {
                        ctx.write(&format!("{}::{} => {},", name, value.get_name(), value.get_number()));
                    }
                });
            });
        });

        block(ctx, &format!("impl ::std::convert::TryFrom<i32> for {}", name), |ctx| {
            ctx.write("type Error = i32;");
            block(ctx, "fn try_from(v: i32) -> ::std::result::Result<Self, i32>", |ctx| {
                block(ctx, "match v", |ctx| {
                    for (_, value) in enum_variants {
                        ctx.write(&format!(
                            "{} => Ok({}::{}),",
                            value.get_number(),
                            name,
                            value.get_name()
                        ));
                    }
                    ctx.write("_ => Err(v),");
                });
            });
        });

        block(ctx, &format!("impl ::pb_jelly::ProtoEnum for {}", name), |ctx| {});

        block(ctx, &format!("impl ::pb_jelly::ClosedProtoEnum for {}", name), |ctx| {
            block(ctx, "fn name(self) -> &'static str", |ctx| {
                block(ctx, "match self", |ctx| {
                    for (_, value) in enum_variants {
                        ctx.write(&format!("{}::{} => \"{}\",", name, value.get_name(), value.get_name()));
                    }
                });
            });
        });
    }

    fn gen_open_enum(
        &mut self,
        name: &str,
        enum_variants: &[(i32, &EnumValueDescriptorProto)],
        scl: &SourceCodeLocation,
    ) {
        let ctx = self;
        let closed_name = format!("{}_Closed", name);

        // Generate an open enum
        ctx.write_comments(ctx.source_code_info_by_scl.get(scl).copied());
        if ctx.derive_serde {
            ctx.write("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]");
        } else {
            ctx.write("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]");
        }
        ctx.write("#[repr(transparent)]");
        ctx.write(format!("pub struct {}(i32);", name));
        block(ctx, format!("impl {name}"), |ctx| {
            for &(idx, value) in enum_variants {
                let vfn = EnumDescriptorProto::default()
                    .descriptor()
                    .unwrap()
                    .get_field("value")
                    .unwrap()
                    .number as i32;
                let mut scl2 = scl.clone();
                scl2.extend_from_slice(&[vfn, idx]);
                ctx.write_comments(ctx.source_code_info_by_scl.get(&scl2).copied());
                ctx.write(format!(
                    "pub const {}: {} = {}({});",
                    value.get_name(),
                    name,
                    name,
                    value.get_number()
                ));
            }
            ctx.write(format!(
                "pub const KNOWN_VARIANTS: [{}; {}] = [{}];",
                name,
                enum_variants.len(),
                enum_variants
                    .iter()
                    .map(|(_, value)| format!("{}::{}", name, value.get_name()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));

            block(ctx, "pub const fn value(self) -> i32", |ctx| {
                ctx.write("self.0");
            });

            block(
                ctx,
                format!("pub fn into_known(self) -> ::std::option::Option<{}>", closed_name),
                |ctx| {
                    block(ctx, "match self", |ctx| {
                        for (_, value) in enum_variants {
                            ctx.write(format!(
                                "{}::{} => Some({}::{}),",
                                name,
                                value.get_name(),
                                closed_name,
                                value.get_name()
                            ));
                        }
                        ctx.write("_ => None,");
                    });
                },
            );
        });

        block(ctx, format!("impl ::std::default::Default for {name}"), |ctx| {
            block(ctx, "fn default() -> Self", |ctx| {
                // Under proto2, the default value is the first defined.
                // Under proto3, the default value is the 0-valued variant, but it's required to
                // be defined first.
                ctx.write(format!("{}::{}", name, enum_variants[0].1.get_name()))
            });
        });

        block(ctx, format!("impl From<{}> for i32", name), |ctx| {
            block(ctx, format!("fn from(v: {}) -> i32", name), |ctx| ctx.write("v.0"));
        });

        block(ctx, format!("impl From<i32> for {}", name), |ctx| {
            block(ctx, format!("fn from(v: i32) -> {}", name), |ctx| {
                ctx.write(format!("{}(v)", name))
            });
        });

        block(ctx, format!("impl From<{}> for {}", closed_name, name), |ctx| {
            block(ctx, format!("fn from(v: {}) -> {}", closed_name, name), |ctx| {
                ctx.write(format!("{}(v as i32)", name))
            });
        });

        block(ctx, format!("impl ::pb_jelly::ProtoEnum for {}", name), |ctx| {});

        block(ctx, format!("impl ::pb_jelly::OpenProtoEnum for {}", name), |ctx| {
            block(
                ctx,
                format!("fn name(self) -> ::std::option::Option<&'static str>"),
                |ctx| {
                    block(ctx, "match self", |ctx| {
                        for (_, value) in enum_variants {
                            ctx.write(format!(
                                "{}::{} => Some(\"{}\"),",
                                name,
                                value.get_name(),
                                value.get_name()
                            ))
                        }
                        ctx.write("_ => None,")
                    });
                },
            );

            block(ctx, "fn is_known(self) -> bool", |ctx| {
                block(ctx, "match self", |ctx| {
                    for (_, value) in enum_variants {
                        ctx.write(format!("{}::{} => true,", name, value.get_name()))
                    }
                    ctx.write("_ => false,")
                });
            });
        });

        block(ctx, format!("impl ::std::fmt::Debug for {name}"), |ctx| {
            block(
                ctx,
                "fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result",
                |ctx| {
                    ctx.write("use ::pb_jelly::OpenProtoEnum;");
                    block(ctx, "match self.name()", |ctx| {
                        ctx.write("Some(s) => write!(f, \"{}\", s),");
                        ctx.write("None => write!(f, \"Unknown({})\", self.0),")
                    });
                },
            );
        });

        ctx.gen_closed_enum(&closed_name, enum_variants, scl);
    }

    fn gen_enum(&mut self, path: &[&str], enum_type: &EnumDescriptorProto, scl: &SourceCodeLocation) {
        assert_eq!(self.indentation, 0);
        let name = [path, &[enum_type.get_name()]].concat().join("_");
        if enum_err_if_default_or_unknown(enum_type) {
            assert!(!enum_closed(enum_type));
            self.gen_closed_enum(
                &name,
                &enum_type
                    .value
                    .iter()
                    .enumerate()
                    .map(|(idx, v)| (idx as i32, v))
                    .filter(|(_, x)| x.get_number() != 0)
                    .collect::<Vec<_>>(),
                scl,
            );
        } else if enum_closed(enum_type) {
            self.gen_closed_enum(
                &name,
                &enum_type
                    .value
                    .iter()
                    .enumerate()
                    .map(|(idx, v)| (idx as i32, v))
                    .collect::<Vec<_>>(),
                scl,
            );
        } else {
            self.gen_open_enum(
                &name,
                &enum_type
                    .value
                    .iter()
                    .enumerate()
                    .map(|(idx, v)| (idx as i32, v))
                    .collect::<Vec<_>>(),
                scl,
            );
        }
    }
    fn gen_msg(&mut self, path: &[&str], msg_type: &'a DescriptorProto, scl: &SourceCodeLocation) {
        let ctx = self;
        assert_eq!(ctx.indentation, 0);
        let name = [path, &[msg_type.get_name()]].concat().join("_");

        let preserve_unrecognized = msg_type.get_options().get_extension(PRESERVE_UNRECOGNIZED).unwrap() == Some(true);
        let has_extensions = !msg_type.extension_range.is_empty();

        // Adjust some field names
        // for field in &msg_type.field {
        //     if RESERVED_KEYWORDS.contains(&field.get_name()) {
        //         field.get_name().push('_');
        //     }
        // }
        // for oneof in &msg_type.oneof_decl {
        //     if RESERVED_KEYWORDS.contains(&oneof.get_name()) {
        //         oneof.get_name().push('_');
        //     }
        // }

        let mut oneof_fields: HashMap<&str, Vec<&'a FieldDescriptorProto>> = HashMap::new();
        let proto3_optional_synthetic_oneofs: HashSet<i32> = msg_type
            .field
            .iter()
            .filter(|f| f.get_proto3_optional())
            .map(|f| f.get_oneof_index())
            .collect();
        // Filter out oneofs synthesized by proto3 optional; we treat these as plain `Option`al fields, not oneofs
        let oneof_decls: Vec<_> = msg_type
            .oneof_decl
            .iter()
            .enumerate()
            .filter(|&(ix, _)| !proto3_optional_synthetic_oneofs.contains(&(ix as i32)))
            .map(|(_, oneof)| oneof)
            .collect();

        let mut derives = vec!["Clone", "Debug", "PartialEq"];
        if ctx.derive_serde {
            derives.extend(["Deserialize", "Serialize"]);
        }

        let fq_msg = (ctx.proto_file.get_name().to_string(), name.clone());
        if ctx.ctx.impls_by_msg[dbg!(&fq_msg)].Eq {
            derives.extend(["Eq", "PartialOrd", "Ord", "Hash"]);
        }
        if ctx.ctx.impls_by_msg[&fq_msg].Copy {
            derives.push("Copy");
        }

        ctx.write_comments(ctx.source_code_info_by_scl.get(scl).copied());
        derives.sort_unstable();

        ctx.write(format!("#[derive({})]", derives.join(", ")));
        block(ctx, format!("pub struct {name}"), |ctx| {
            for (idx, field) in msg_type.field.iter().enumerate() {
                let ffn = DescriptorProto::default()
                    .descriptor()
                    .unwrap()
                    .get_field("field")
                    .unwrap()
                    .number as i32;
                let mut scl2 = scl.clone();
                scl2.extend_from_slice(&[ffn, idx as i32]);
                ctx.write_comments(ctx.source_code_info_by_scl.get(&scl2).copied());

                let typ = ctx.rust_type(Some(msg_type), field);
                if let Some(oneof) = typ.oneof {
                    oneof_fields.entry(oneof.get_name()).or_default().push(field);
                } else {
                    ctx.write(format!("pub {}: {},", field.get_name(), typ.storage_type()));
                }
            }

            for &oneof in &oneof_decls {
                if oneof_nullable(oneof) {
                    ctx.write(format!(
                        "pub {}: ::std::option::Option<{}>,",
                        oneof.get_name(),
                        oneof_msg_name(&name, oneof)
                    ));
                } else {
                    ctx.write(format!("pub {}: {},", oneof.get_name(), oneof_msg_name(&name, oneof)));
                }
            }

            if preserve_unrecognized {
                ctx.write("pub _unrecognized: Vec<u8>,")
            }

            if has_extensions {
                ctx.write("pub _extensions: ::pb_jelly::Unrecognized,")
            }
        });

        // Generate any oneof enum structs
        for &oneof in &oneof_decls {
            ctx.write(format!("#[derive({})]", derives.join(", ")));
            block(ctx, format!("pub enum {}", oneof_msg_name(&name, oneof)), |ctx| {
                for oneof_field in &oneof_fields[oneof.get_name()] {
                    let typ = ctx.rust_type(Some(msg_type), oneof_field);
                    ctx.write(format!("{},", typ.oneof_field_match(&typ.storage_type())));
                }
            });
        }

        if !ctx.is_proto3 {
            block(ctx, format!("impl {name}"), |ctx| {
                for field in &msg_type.field {
                    let typ = ctx.rust_type(Some(msg_type), field);
                    if typ.oneof.is_some() {
                        continue;
                    }
                    if typ.is_repeated() {
                        block(
                            ctx,
                            format!(
                                "pub fn set_{}(&mut self, v: ::std::vec::Vec<{}>)",
                                field.get_name(),
                                typ.rust_type()
                            ),
                            |ctx| {
                                ctx.write(format!("self.{} = v;", field.get_name()));
                            },
                        );

                        block(
                            ctx,
                            format!(
                                "pub fn take_{}(&mut self) -> ::std::vec::Vec<{}>",
                                field.get_name(),
                                typ.rust_type()
                            ),
                            |ctx| {
                                ctx.write(format!("::std::mem::take(&mut self.{})", field.get_name()));
                            },
                        );

                        block(
                            ctx,
                            format!("pub fn get_{}(&self) -> &[{}]", field.get_name(), typ.rust_type()),
                            |ctx| {
                                ctx.write(format!("&self.{}", field.get_name()));
                            },
                        );
                        block(
                            ctx,
                            format!(
                                "pub fn mut_{}(&mut self) -> &mut ::std::vec::Vec<{}>",
                                field.get_name(),
                                typ.rust_type()
                            ),
                            |ctx| {
                                ctx.write(format!("&mut self.{}", field.get_name()));
                            },
                        );
                    } else if typ.is_nullable() {
                        block(ctx, format!("pub fn has_{}(&self) -> bool", field.get_name()), |ctx| {
                            ctx.write(format!("self.{}.is_some()", field.get_name()));
                        });

                        let (input_type, input_expr) = typ.set_method();
                        block(
                            ctx,
                            format!("pub fn set_{}(&mut self, v: {})", field.get_name(), input_type),
                            |ctx| {
                                ctx.write(format!("self.{} = Some({});", field.get_name(), input_expr));
                            },
                        );

                        if let (Some(return_type), Some(return_expr)) = typ.take_method() {
                            block(
                                ctx,
                                format!("pub fn take_{}(&mut self) -> {}", field.get_name(), return_type),
                                |ctx| ctx.write(return_expr),
                            );
                        }

                        if !(typ.is_blob() || typ.is_grpc_slices() || typ.is_lazy_bytes()) {
                            // It's hard to make this make sense, so let's not generate `get` method for lazy things.
                            let (return_type, return_expr) = typ.get_method();
                            block(
                                ctx,
                                format!("pub fn get_{}(&self) -> {}", field.get_name(), return_type),
                                |ctx| {
                                    ctx.write(return_expr);
                                },
                            );
                        }
                    }
                }
            });
        }

        block(ctx, format!("impl ::std::default::Default for {name}"), |ctx| {
            block(ctx, "fn default() -> Self", |ctx| {
                block(ctx, name.clone(), |ctx| {
                    for field in &msg_type.field {
                        let typ = ctx.rust_type(Some(msg_type), field);
                        if typ.oneof.is_none() {
                            ctx.write(format!("{}: {},", field.get_name(), typ.default(&name)))
                        }
                    }
                    for &oneof in &oneof_decls {
                        let oneof_field = oneof_fields[oneof.get_name()][0];
                        let typ = ctx.rust_type(Some(msg_type), oneof_field);
                        ctx.write(format!("{}: {},", oneof.get_name(), typ.default(&name)))
                    }
                    if preserve_unrecognized {
                        ctx.write("_unrecognized: Vec::new(),")
                    }
                    if has_extensions {
                        ctx.write("_extensions: ::pb_jelly::Unrecognized::default(),")
                    }
                });
            });
        });

        block(ctx, "lazy_static!", |ctx| {
            ctx.write(format!(
                "pub static ref {}_default: {} = {}::default();",
                name, name, name
            ));
        });

        block(ctx, format!("impl ::pb_jelly::Message for {name}"), |ctx| {
            block(
                ctx,
                "fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor>",
                |ctx| {
                    let name = format!("{}_{}", path.join("_"), msg_type.get_name());
                    let full_name = if let Some(ref package) = ctx.proto_file.package {
                        format!("{package}.{name}")
                    } else {
                        name.clone()
                    };

                    block_with(ctx, "Some(::pb_jelly::MessageDescriptor", " {", "})", |ctx| {
                        ctx.write(format!("name: \"{}\",", name));
                        ctx.write(format!("full_name: \"{}\",", full_name));
                        block_with(ctx, "fields:", " &[", "],", |ctx| {
                            for (i, field) in msg_type.field.iter().enumerate() {
                                block_with(ctx, "::pb_jelly::FieldDescriptor", " {", "},", |ctx| {
                                    let full_name = if let Some(ref package) = ctx.proto_file.package {
                                        format!("{package}.{name}.{field_name}", field_name = field.get_name())
                                    } else {
                                        format!("{name}.{field_name}", field_name = field.get_name())
                                    };

                                    let typ = ctx.rust_type(Some(msg_type), field);
                                    ctx.write(format!("name: \"{}\",", field.get_name()));
                                    ctx.write(format!("full_name: \"{}\",", full_name));
                                    ctx.write(format!("index: {},", i));
                                    ctx.write(format!("number: {},", field.get_number()));
                                    ctx.write(format!("typ: ::pb_jelly::wire_format::Type::{},", typ.wire_format()));
                                    if field.get_label() == FieldDescriptorProto_Label::LABEL_OPTIONAL {
                                        ctx.write("label: ::pb_jelly::Label::Optional,")
                                    } else if field.get_label() == FieldDescriptorProto_Label::LABEL_REQUIRED {
                                        ctx.write("label: ::pb_jelly::Label::Required,")
                                    } else if field.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED {
                                        ctx.write("label: ::pb_jelly::Label::Repeated,")
                                    }

                                    if (field.has_oneof_index() && !field.get_proto3_optional()) {
                                        ctx.write(format!("oneof_index: Some({}),", field.get_oneof_index()))
                                    } else {
                                        ctx.write("oneof_index: None,");
                                    }
                                });
                            }
                        });

                        block_with(ctx, "oneofs:", " &[", "],", |ctx| {
                            // Note that synthetic oneofs are always located at the end of `msg_type.oneof_decl`,
                            // so the oneof indices will still match
                            for &oneof in &oneof_decls {
                                block_with(ctx, "::pb_jelly::OneofDescriptor", " {", "},", |ctx| {
                                    ctx.write(format!("name: \"{}\",", oneof.get_name()));
                                });
                            }
                        });
                    });
                },
            );

            block(ctx, "fn compute_size(&self) -> usize", |ctx| {
                if msg_type.field.len() > 0 || preserve_unrecognized || has_extensions {
                    ctx.write("let mut size = 0;");
                    for field in &msg_type.field {
                        let typ = ctx.rust_type(Some(msg_type), field);

                        ctx.write(format!("let mut {}_size = 0;", field.get_name()));
                        field_iter(ctx, "val", &name, msg_type, field, |ctx| {
                            ctx.write("let l = ::pb_jelly::Message::compute_size(val);");
                            if !typ.should_serialize_packed() {
                                ctx.write(format!(
                                    "{}_size += ::pb_jelly::wire_format::serialized_length({});",
                                    field.get_name(),
                                    field.get_number()
                                ));
                            }
                            if typ.is_length_delimited() {
                                ctx.write(format!(
                                    "{}_size += ::pb_jelly::varint::serialized_length(l as u64);",
                                    field.get_name()
                                ));
                            }
                            ctx.write(format!("{}_size += l;", field.get_name()));
                        });
                        if typ.should_serialize_packed() {
                            block(ctx, format!("if !self.{}.is_empty()", field.get_name()), |ctx| {
                                ctx.write(format!(
                                    "size += ::pb_jelly::wire_format::serialized_length({});",
                                    field.get_number()
                                ));
                                ctx.write(format!(
                                    "size += ::pb_jelly::varint::serialized_length({}_size as u64);",
                                    field.get_name()
                                ));
                            });
                        }
                        ctx.write(format!("size += {}_size;", field.get_name()));
                    }
                    if preserve_unrecognized {
                        ctx.write("size += self._unrecognized.len();");
                    }
                    if has_extensions {
                        ctx.write("size += self._extensions.compute_size();");
                    }
                    ctx.write("size");
                } else {
                    ctx.write("0");
                }
            });

            block(ctx, "fn compute_grpc_slices_size(&self) -> usize", |ctx| {
                if !msg_type.field.is_empty() {
                    ctx.write("let mut size = 0;");
                    for field in &msg_type.field {
                        field_iter(ctx, "val", &name, msg_type, field, |ctx| {
                            ctx.write("size += ::pb_jelly::Message::compute_grpc_slices_size(val);")
                        });
                    }
                    ctx.write("size");
                } else {
                    ctx.write("0");
                }
            });

            block(
                ctx,
                "fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()>",
                |ctx| {
                    let mut fields: Vec<_> = msg_type.field.iter().collect();
                    fields.sort_unstable_by_key(|f| f.get_number());
                    for field in fields {
                        let typ = ctx.rust_type(Some(msg_type), field);
                        // In proto2, this ensures we don't emit fields set to None
                        // In proto3, this ensures we don't emit fields set to their default value
                        if typ.should_serialize_packed() {
                            block(ctx, format!("if !self.{}.is_empty()", field.get_name()), |ctx| {
                                ctx.write("let mut size = 0;");
                                field_iter(ctx, "val", &name, msg_type, field, |ctx| {
                                    ctx.write("size += ::pb_jelly::Message::compute_size(val);")
                                });
                                ctx.write(format!("::pb_jelly::wire_format::write({}, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;", field.get_number()));
                                ctx.write("::pb_jelly::varint::write(size as u64, w)?;");
                            });
                        }

                        field_iter(ctx, "val", &name, msg_type, field, |ctx| {
                            if !typ.should_serialize_packed() {
                                ctx.write(format!(
                                    "::pb_jelly::wire_format::write({}, ::pb_jelly::wire_format::Type::{}, w)?;",
                                    field.get_number(),
                                    typ.wire_format()
                                ));
                            }
                            if typ.is_length_delimited() {
                                ctx.write("let l = ::pb_jelly::Message::compute_size(val);");
                                ctx.write("::pb_jelly::varint::write(l as u64, w)?;");
                            }
                            ctx.write("::pb_jelly::Message::serialize(val, w)?;");
                        });
                    }
                    if preserve_unrecognized {
                        ctx.write("w.write_all(&self._unrecognized)?;");
                    }
                    if has_extensions {
                        ctx.write("self._extensions.serialize(w)?;");
                    }
                    ctx.write("Ok(())");
                },
            );

            block(
                ctx,
                "fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()>",
                |ctx| {
                    if preserve_unrecognized {
                        ctx.write("let mut unrecognized = ::pb_jelly::Unrecognized::default();");
                    }

                    for &oneof in &oneof_decls {
                        if !oneof_nullable(oneof) {
                            let oneof_typ = oneof_msg_name(&name, oneof);
                            ctx.write(format!(
                                "let mut oneof_{}: ::std::option::Option<{}> = None;",
                                oneof.get_name(),
                                oneof_typ
                            ));
                        }
                    }
                    let mut err_if_default_field_names = IndexSet::new();
                    for field in &msg_type.field {
                        let typ = ctx.rust_type(Some(msg_type), field);
                        if field.get_type_() == FieldDescriptorProto_Type::TYPE_ENUM && !typ.is_repeated() {
                            let enum_type = ctx.ctx.find(field.get_type_name()).enum_typ();
                            if enum_err_if_default_or_unknown(enum_type) && typ.oneof.is_none() {
                                ctx.write(format!(
                                    "let mut {}: ::std::option::Option<{}> = None;",
                                    field.get_name(),
                                    typ.rust_type()
                                ));
                                err_if_default_field_names.insert(field.get_name());
                            }
                        }
                    }

                    block(
                        ctx,
                        "while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)?",
                        |ctx| {
                            block(ctx, "match field_number", |ctx| {
                                for field in &msg_type.field {
                                    let typ = ctx.rust_type(Some(msg_type), field);
                                    block(ctx, format!("{} =>", field.get_number()), |ctx| {
                                        if typ.can_be_packed() {
                                            block(ctx, "match typ", |ctx| {
                                                block(
                                                    ctx,
                                                    "::pb_jelly::wire_format::Type::LengthDelimited =>",
                                                    |ctx| {
                                                        ctx.write(
                                                            "let len = ::pb_jelly::varint::ensure_read(&mut buf)?;",
                                                        );
                                                        ctx.write("let mut vals = ::pb_jelly::ensure_split(buf, len as usize)?;");
                                                        block(ctx, "while vals.has_remaining()", |ctx| {
                                                            ctx.write(format!(
                                                                "let mut val: {} = ::std::default::Default::default();",
                                                                typ.rust_type()
                                                            ));
                                                            ctx.write("::pb_jelly::Message::deserialize(&mut val, &mut vals)?;");
                                                            ctx.write(format!("self.{}.push(val);", field.get_name()));
                                                        });
                                                    },
                                                );
                                                block(ctx, "_ =>", |ctx| {
                                                    ctx.write(format!("::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::{}, \"{}\", {})?;", typ.wire_format(), name, field.get_number(),));
                                                    ctx.write(format!(
                                                        "let mut val: {} = ::std::default::Default::default();",
                                                        typ.rust_type()
                                                    ));
                                                    ctx.write("::pb_jelly::Message::deserialize(&mut val, buf)?;");
                                                    ctx.write(format!("self.{}.push(val);", field.get_name()));
                                                });
                                            });
                                        } else {
                                            if typ.is_length_delimited() {
                                                ctx.write(format!("::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, \"{}\", {})?;", name, field.get_number()));
                                                ctx.write("let len = ::pb_jelly::varint::ensure_read(&mut buf)?;");
                                                ctx.write(
                                                    "let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;",
                                                );
                                                ctx.write(format!(
                                                    "let mut val: {} = ::std::default::Default::default();",
                                                    typ.rust_type()
                                                ));
                                                ctx.write("::pb_jelly::Message::deserialize(&mut val, &mut next)?;");
                                            } else {
                                                ctx.write(format!("::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::{}, \"{}\", {})?;", typ.wire_format(), name, field.get_number()));
                                                ctx.write(format!(
                                                    "let mut val: {} = ::std::default::Default::default();",
                                                    typ.rust_type()
                                                ));
                                                ctx.write("::pb_jelly::Message::deserialize(&mut val, buf)?;");
                                            }

                                            if typ.is_repeated() {
                                                ctx.write(format!("self.{}.push(val);", field.get_name()))
                                            } else {
                                                let field_val = if typ.is_boxed() { "Box::new(val)" } else { "val" };

                                                if let Some(oneof) = typ.oneof {
                                                    if oneof_nullable(oneof) {
                                                        ctx.write(format!(
                                                            "self.{} = {};",
                                                            oneof.get_name(),
                                                            typ.oneof_val(&name, field_val)
                                                        ));
                                                    } else {
                                                        ctx.write(format!(
                                                            "oneof_{} = Some({});",
                                                            oneof.get_name(),
                                                            typ.oneof_val(&name, field_val)
                                                        ))
                                                    }
                                                } else if typ.is_nullable() {
                                                    ctx.write(format!(
                                                        "self.{} = Some({});",
                                                        field.get_name(),
                                                        field_val
                                                    ));
                                                } else if err_if_default_field_names.contains(&field.get_name()) {
                                                    ctx.write(format!("{} = Some({});", field.get_name(), field_val));
                                                } else {
                                                    ctx.write(format!("self.{} = {};", field.get_name(), field_val));
                                                }
                                            }
                                        }
                                    });
                                }
                                if has_extensions {
                                    let pattern = msg_type
                                        .extension_range
                                        .iter()
                                        .map(|r| format!("{}..={}", r.get_start(), r.get_end() - 1))
                                        .collect::<Vec<_>>()
                                        .join(" | ");
                                    block(ctx, format!("{pattern} =>"), |ctx| {
                                        ctx.write("self._extensions.gather(field_number, typ, &mut buf)?;");
                                    });
                                }
                                block(ctx, "_ =>", |ctx| {
                                    if preserve_unrecognized {
                                        ctx.write("unrecognized.gather(field_number, typ, &mut buf)?;");
                                    } else {
                                        ctx.write("::pb_jelly::skip(typ, &mut buf)?;");
                                    }
                                });
                            });
                        },
                    );
                    for &oneof in &oneof_decls {
                        if !oneof_nullable(oneof) {
                            block(ctx, format!("match oneof_{}", oneof.get_name()), |ctx| {
                                ctx.write(format!("Some(v) => self.{} = v,", oneof.get_name()));
                                ctx.write(format!("None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, \"missing value for non-nullable oneof '{}' while parsing message {}.{}\")),", oneof.get_name(), ctx.proto_file.get_package(), msg_type.get_name()))
                            });
                        }
                    }

                    for field_name in err_if_default_field_names {
                        block(ctx, format!("match {field_name}"), |ctx| {
                            ctx.write(format!("Some(v) => self.{} = v,", field_name));
                            ctx.write(format!("None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, \"err_if_default_or_unknown '{}' had no value while parsing message {}.{}\")),", field_name, ctx.proto_file.get_package(), msg_type.get_name()));
                        });
                    }

                    if preserve_unrecognized {
                        ctx.write("self._unrecognized.reserve(unrecognized.compute_size());");
                        ctx.write("unrecognized.serialize(&mut std::io::Cursor::new(&mut self._unrecognized))?;");
                    }
                    ctx.write("Ok(())");
                },
            );
        });

        block(ctx, format!("impl ::pb_jelly::Reflection for {name}"), |ctx| {
            block(
                ctx,
                "fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str>",
                |ctx| {
                    block(ctx, "match oneof_name", |ctx| {
                        for &oneof in &oneof_decls {
                            block(ctx, format!("\"{}\" =>", oneof.get_name()), |ctx| {
                                for &oneof_field in &oneof_fields[oneof.get_name()] {
                                    field_iter(ctx, "val", &name, msg_type, oneof_field, |ctx| {
                                        ctx.write(format!("return Some(\"{}\");", oneof_field.get_name()));
                                    });
                                }
                                ctx.write("None");
                            });
                        }
                        block(ctx, "_ =>", |ctx| {
                            ctx.write("panic!(\"unknown oneof name given\");");
                        });
                    });
                },
            );

            block(
                ctx,
                "fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_>",
                |ctx| {
                    block(ctx, "match field_name", |ctx| {
                        for field in &msg_type.field {
                            let typ = ctx.rust_type(Some(msg_type), field);
                            block(ctx, format!("\"{}\" =>", field.get_name()), |ctx| {
                                if let Some(oneof) = typ.oneof {
                                    if oneof_fields[oneof.get_name()].len() > 1 || oneof_nullable(oneof) {
                                        // Only useful to generate this logic if there is more than one
                                        // possible value for this oneof.
                                        block(ctx, format!("match self.{}", oneof.get_name()), |ctx| {
                                            ctx.write(format!("{} => (),", typ.oneof_val(&name, "_")));
                                            block_with(ctx, "_ =>", " {", "},", |ctx| {
                                                // If this oneof is not currently set to this variant, we explicitly
                                                // set it to this variant.
                                                ctx.write(format!(
                                                    "self.{} = {};",
                                                    oneof.get_name(),
                                                    typ.oneof_val(&name, "::std::default::Default::default()"),
                                                ));
                                            });
                                        });
                                    }
                                    if typ.is_empty_oneof_field() {
                                        ctx.write("::pb_jelly::reflection::FieldMut::Empty");
                                    } else {
                                        block(
                                            ctx,
                                            format!(
                                                "if let {} = self.{}",
                                                typ.oneof_val(&name, "ref mut val"),
                                                oneof.get_name()
                                            ),
                                            |ctx| {
                                                if typ.is_boxed() {
                                                    ctx.write("let val = &mut **val;");
                                                }
                                                ctx.write("return ::pb_jelly::reflection::FieldMut::Value(val);");
                                            },
                                        );
                                        ctx.write("unreachable!()");
                                    }
                                } else if typ.is_repeated() {
                                    // TODO: Would be nice to support this, but some more thought would
                                    // need to be put into what the API for it looks like.
                                    // self.write(format!("::pb_jelly::reflection::FieldMut::Repeated(&mut self.{})", field.get_name()));
                                    ctx.write("unimplemented!(\"Repeated fields are not currently supported.\")");
                                } else if typ.is_nullable() && typ.is_boxed() {
                                    ctx.write(format!("::pb_jelly::reflection::FieldMut::Value(self.{}.get_or_insert_with(::std::default::Default::default).as_mut())", field.get_name()));
                                } else if typ.is_boxed() {
                                    ctx.write(format!(
                                        "::pb_jelly::reflection::FieldMut::Value(self.{}.as_mut())",
                                        field.get_name()
                                    ));
                                } else if typ.is_nullable() {
                                    ctx.write(format!("::pb_jelly::reflection::FieldMut::Value(self.{}.get_or_insert_with(::std::default::Default::default))", field.get_name()));
                                } else {
                                    ctx.write(format!(
                                        "::pb_jelly::reflection::FieldMut::Value(&mut self.{})",
                                        field.get_name()
                                    ));
                                }
                            });
                        }
                        block(ctx, "_ =>", |ctx| {
                            ctx.write("panic!(\"unknown field name given\")");
                        });
                    });
                },
            );
        });

        if has_extensions {
            block(
                ctx,
                format!("impl ::pb_jelly::extensions::Extensible for {name}"),
                |ctx| {
                    block(ctx, "fn _extensions(&self) -> &::pb_jelly::Unrecognized", |ctx| {
                        ctx.write("&self._extensions");
                    });
                },
            );
        }
    }

    fn gen_extension(&mut self, path: &[&str], extension_field: &'a FieldDescriptorProto, scl: &SourceCodeLocation) {
        let (crate_, mod_parts) = self.ctx.crate_from_proto_filename(self.proto_file.get_name());

        self.write_comments(self.source_code_info_by_scl.get(scl).copied());
        let name = [path, &[&extension_field.get_name().to_ascii_uppercase()]].concat().join("_");
        let rust_type = self.rust_type(None, extension_field);
        let extendee = self.ctx.find(extension_field.get_extendee());
        let kind = if extension_field.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED {
            "RepeatedExtension"
        } else {
            "SingularExtension"
        };

        self.write(format!(
            "pub const {}: ::pb_jelly::extensions::{}<{}, {}> = 
    ::pb_jelly::extensions::{}::new(
        {},
        ::pb_jelly::wire_format::Type::{},
        \"{}\",
    );",
            name,
            kind,
            extendee.rust_name(self.ctx, &crate_, &mod_parts),
            rust_type.rust_type(),
            kind,
            extension_field.get_number(),
            rust_type.wire_format(),
            extension_field.get_name(),
        ));
    }
}

#[derive(Debug)]
enum ProtoTypeDescriptor<'a> {
    Message(&'a DescriptorProto),
    Enum(&'a EnumDescriptorProto),
}

impl<'a> ProtoTypeDescriptor<'a> {
    fn get_name(&self) -> &str {
        match self {
            ProtoTypeDescriptor::Message(m) => m.get_name(),
            ProtoTypeDescriptor::Enum(e) => e.get_name(),
        }
    }
}

#[derive(Default)]
struct WalkResult<'a> {
    enums: Vec<(Vec<&'a str>, &'a EnumDescriptorProto, SourceCodeLocation)>,
    messages: Vec<(Vec<&'a str>, &'a DescriptorProto, SourceCodeLocation)>,
    extensions: Vec<(Vec<&'a str>, &'a FieldDescriptorProto, SourceCodeLocation)>,
}

fn walk<'a>(proto: &'a FileDescriptorProto) -> WalkResult<'a> {
    let mut result = WalkResult::default();

    fn _walk_file<'a>(
        proto: &'a FileDescriptorProto,
        parents: Vec<&'a str>,
        scl_prefix: Vec<i32>,
        result: &mut WalkResult<'a>,
    ) {
        for (i, enum_type) in proto.enum_type.iter().enumerate() {
            let etfn = FileDescriptorProto::default()
                .descriptor()
                .unwrap()
                .get_field("enum_type")
                .unwrap()
                .number as i32;
            let mut scl2 = scl_prefix.clone();
            scl2.extend_from_slice(&[etfn, i as i32]);
            _walk_enum(enum_type, parents.clone(), scl2, result);
        }

        for (i, message_type) in proto.message_type.iter().enumerate() {
            let mtfn = FileDescriptorProto::default()
                .descriptor()
                .unwrap()
                .get_field("message_type")
                .unwrap()
                .number as i32;
            let mut scl2 = scl_prefix.clone();
            scl2.extend_from_slice(&[mtfn, i as i32]);
            _walk_message(message_type, parents.clone(), scl2, result);
        }

        for (i, nested_extension) in proto.extension.iter().enumerate() {
            let extfn = FileDescriptorProto::default()
                .descriptor()
                .unwrap()
                .get_field("extension")
                .unwrap()
                .number as i32;
            let mut scl2 = scl_prefix.clone();
            scl2.extend_from_slice(&[extfn, i as i32]);
            result.extensions.push((parents.clone(), nested_extension, scl2))
        }
    }

    fn _walk_enum<'a>(
        proto: &'a EnumDescriptorProto,
        parents: Vec<&'a str>,
        scl_prefix: Vec<i32>,
        result: &mut WalkResult<'a>,
    ) {
        result.enums.push((parents, proto, scl_prefix));
    }

    fn _walk_message<'a>(
        proto: &'a DescriptorProto,
        parents: Vec<&'a str>,
        scl_prefix: Vec<i32>,
        result: &mut WalkResult<'a>,
    ) {
        result.messages.push((parents.clone(), proto, scl_prefix.clone()));
        let mut parents2 = parents.clone();
        parents2.push(proto.get_name());

        for (i, nested_enum) in proto.enum_type.iter().enumerate() {
            let etfn = DescriptorProto::default()
                .descriptor()
                .unwrap()
                .get_field("enum_type")
                .unwrap()
                .number as i32;
            let mut scl2 = scl_prefix.clone();
            scl2.extend_from_slice(&[etfn, i as i32]);
            _walk_enum(nested_enum, parents2.clone(), scl2, result);
        }

        for (i, nested_message) in proto.nested_type.iter().enumerate() {
            let ntfn = DescriptorProto::default()
                .descriptor()
                .unwrap()
                .get_field("nested_type")
                .unwrap()
                .number as i32;
            let mut scl2 = scl_prefix.clone();
            scl2.extend_from_slice(&[ntfn, i as i32]);
            _walk_message(nested_message, parents2.clone(), scl2, result);
        }

        for (i, nested_extension) in proto.extension.iter().enumerate() {
            let extfn = DescriptorProto::default()
                .descriptor()
                .unwrap()
                .get_field("extension_type")
                .unwrap()
                .number as i32;
            let mut scl2 = scl_prefix.clone();
            scl2.extend_from_slice(&[extfn, i as i32]);
            result.extensions.push((parents2.clone(), nested_extension, scl2))
        }
    }

    _walk_file(proto, vec![], vec![], &mut result);
    result
}

#[derive(Debug)]
struct ProtoType<'a> {
    proto_file: &'a FileDescriptorProto,
    path: Vec<&'a str>, // inside proto file
    typ: ProtoTypeDescriptor<'a>,
    crate_: String,
    mod_parts: Vec<String>,
}

impl<'a> ProtoType<'a> {
    fn new(
        ctx: &Context<'a>,
        proto_file: &'a FileDescriptorProto,
        path: Vec<&'a str>,
        typ: ProtoTypeDescriptor<'a>,
    ) -> ProtoType<'a> {
        let (crate_, mod_parts) = ctx.crate_from_proto_filename(proto_file.get_name());
        if typ.get_name() == "Label" {
            assert!(!path.is_empty());
        }

        ProtoType {
            proto_file,
            path,
            typ,
            crate_,
            mod_parts,
        }
    }

    fn proto_name(&self) -> String {
        let mut r = String::new();
        for part in &self.path {
            r.push('.');
            r.push_str(part);
        }
        r.push('.');
        r.push_str(self.typ.get_name());
        if let Some(ref package) = self.proto_file.package {
            r = format!(".{}{}", package, r);
        }
        r
    }

    fn rust_name(&self, ctx: &Context<'a>, other_crate: &str, other_mod_parts: &[impl AsRef<str>]) -> String {
        let (my_crate, my_mod_parts) = ctx.crate_from_proto_filename(self.proto_file.get_name());
        if my_crate == other_crate
            && my_mod_parts
                .iter()
                .map(|s| s.as_str())
                .eq(other_mod_parts.iter().map(|s| s.as_ref()))
        {
            // In the same Rust binary, directly use typename
            return self.path.join("_") + "_" + &self.typ.get_name();
        }

        let mut mod_parts = self.mod_parts.clone();
        mod_parts.push(self.path.join("_") + "_" + &self.typ.get_name());
        if other_crate != self.crate_ {
            // Different crate. Insert crate name in fully qualified module.
            mod_parts.insert(0, "::".to_owned() + &self.crate_);
        } else {
            // Same crate. Use super::<module_name>::<local_type>
            let num_supers = other_mod_parts.len();
            let supers = vec!["super"; num_supers].join("::");
            mod_parts.insert(0, supers);
        }
        mod_parts.join("::")
    }

    fn msg_typ(&self) -> &'a DescriptorProto {
        match self.typ {
            ProtoTypeDescriptor::Message(m) => m,
            _ => panic!("not a message type"),
        }
    }

    fn enum_typ(&self) -> &EnumDescriptorProto {
        match self.typ {
            ProtoTypeDescriptor::Enum(e) => e,
            _ => panic!("not an enum type"),
        }
    }
}

struct Impls {
    Eq: bool,
    Copy: bool,
}

struct Context<'a> {
    proto_types: HashMap<String, ProtoType<'a>>,
    deps_map: RefCell<HashMap<String, HashSet<String>>>,
    extra_crates: HashMap<String, HashSet<String>>,
    /// Map from msg.type_name => whether it impls Eq / Copy
    /// We have to build this on the fly as we process the types.
    impls_by_msg: HashMap<(String, String), Impls>,
    /// Indicator if every directory should be their own crate.
    crate_per_dir: bool,
    /// Prefix of the crate path which should be cleared before making a determination
    /// of how to split the crates apart.
    prefix_to_clear: String,
}

impl<'a> Context<'a> {
    fn new(crate_per_dir: bool, prefix_to_clear: String) -> Self {
        Context {
            proto_types: HashMap::new(),
            deps_map: RefCell::new(HashMap::new()),
            extra_crates: HashMap::new(),
            impls_by_msg: HashMap::new(),
            crate_per_dir,
            prefix_to_clear,
        }
    }
    fn calc_impls(
        &mut self,
        proto_file: &FileDescriptorProto,
        crate_: &str,
        msg_type: &DescriptorProto,
        fq_msg: (String, String),
    ) {
        // Determine if each message can implement Eq / Ord Copy traits. Using Dynamic Programming.
        // See if it's cached
        if self.impls_by_msg.contains_key(&fq_msg) {
            return;
        }

        // Place a false in the cache to prevent recursion loops
        self.impls_by_msg
            .insert(fq_msg.clone(), Impls { Eq: false, Copy: false });

        // Generate whether or not this msg implements Eq / Ord
        let mut msg_impls_eq = true;
        let mut msg_impls_copy = true;

        if msg_type
            .get_options()
            .get_extension(extensions::PRESERVE_UNRECOGNIZED)
            .unwrap()
            == Some(true)
        {
            // TODO: copy pasta
            msg_impls_copy = false; // Preserve unparsed has a Vec which is not Copy
        }

        if !msg_type.get_extension_range().is_empty() {
            // `Unrecognized` is neither Copy nor Eq
            msg_impls_eq = false;
            msg_impls_copy = false;
        }

        for field in msg_type.get_field() {
            let typ = field.get_type_();
            let rust_type = RustType::new(self, proto_file, Some(msg_type), field);
            if let Some(custom_type) = rust_type.custom_type() {
                self.extra_crates
                    .entry(crate_.to_string())
                    .or_default()
                    .extend(CRATE_NAME_REGEX.find_iter(&custom_type).map(|m| m.as_str().to_owned()));
            }

            let field_type = if let Some(ref type_name) = field.type_name {
                let field_type = self.find(&type_name);
                let mut deps_map = self.deps_map.borrow_mut();
                if let Some(dep_crate) = deps_map.get(crate_) {
                    let (dep_crate, _) = self.crate_from_proto_filename(field_type.proto_file.get_name());
                    if dep_crate != crate_ {
                        deps_map.entry(crate_.to_string()).or_default().insert(dep_crate);
                    }
                }
                Some(field_type)
            } else {
                None
            };

            if field.get_label() == FieldDescriptorProto_Label::LABEL_REPEATED {
                msg_impls_copy = false; // Vecs are not Copy
            }

            // If we use a Blob type, or GRPC Slice
            if typ == FieldDescriptorProto_Type::TYPE_BYTES
                && (field.get_options().get_ctype() == FieldOptions_CType::CORD
                    || field.get_options().get_extension(extensions::GRPC_SLICES).unwrap() == Some(true))
            {
                // Blob is not eq/copy
                msg_impls_eq = false;
                msg_impls_copy = false;
                self.extra_crates
                    .entry(crate_.to_string())
                    .or_default()
                    .insert("blob_pb".to_owned());
            }
            // If we use a Bytes type
            else if typ == FieldDescriptorProto_Type::TYPE_BYTES
                && field.get_options().get_extension(extensions::ZERO_COPY).unwrap() == Some(true)
            {
                msg_impls_eq = false;
                msg_impls_copy = false;
                self.extra_crates
                    .entry(crate_.to_string())
                    .or_default()
                    .insert("bytes".to_owned());
            } else if let Some(primitive_type) = get_primitive_type(typ) {
                if !primitive_type.impls_eq {
                    msg_impls_eq = false;
                }
                if !primitive_type.impls_copy {
                    msg_impls_copy = false;
                }
            } else if typ == FieldDescriptorProto_Type::TYPE_ENUM {
                // Enums are Eq / Copy
            } else if typ == FieldDescriptorProto_Type::TYPE_MESSAGE {
                let field_type = field_type.unwrap();
                let field_fq_msg = (
                    field_type.proto_file.get_name().to_owned(),
                    [&field_type.path[..], &[field_type.typ.get_name()]].concat().join("_"),
                );

                if msg_type
                    .get_options()
                    .get_extension(extensions::PRESERVE_UNRECOGNIZED)
                    .unwrap()
                    == Some(true)
                {
                    // TODO: this check isn't really necessary, but it is useful
                    assert!(
                        field_type
                            .msg_typ()
                            .get_options()
                            .get_extension(extensions::PRESERVE_UNRECOGNIZED)
                            .unwrap()
                            == Some(true),
                        "{:?} preserves unrecognized but child message {:?} does not",
                        fq_msg,
                        field_fq_msg,
                    );
                }

                self.calc_impls(
                    field_type.proto_file,
                    crate_,
                    field_type.msg_typ(),
                    field_fq_msg.clone(),
                );

                if !self.impls_by_msg[&field_fq_msg].Eq {
                    msg_impls_eq = false;
                }
                if !self.impls_by_msg[&field_fq_msg].Copy {
                    msg_impls_copy = false;
                }

                let rust_type = RustType::new(self, proto_file, Some(msg_type), field);
                if rust_type.is_boxed() {
                    msg_impls_copy = false;
                }
            } else {
                panic!("Unsupported type: {:?}", typ)
            }
        }

        self.impls_by_msg.insert(
            fq_msg,
            Impls {
                Eq: msg_impls_eq,
                Copy: msg_impls_copy,
            },
        );
    }
    fn feed(&mut self, proto_file: &'a FileDescriptorProto, to_generate: &[&str]) {
        let WalkResult {
            enums,
            messages,
            extensions,
        } = walk(&proto_file);

        for name in to_generate {
            let (crate_name, _) = self.crate_from_proto_filename(name);
            self.deps_map.borrow_mut().insert(crate_name, HashSet::new());
        }

        for (enum_path, enum_typ, _) in enums {
            let enum_pt = ProtoType::new(self, &proto_file, enum_path, ProtoTypeDescriptor::Enum(enum_typ));
            self.proto_types.insert(enum_pt.proto_name(), enum_pt);
        }

        for &(ref path, typ, _) in &messages {
            let msg_pt = ProtoType::new(self, &proto_file, path.clone(), ProtoTypeDescriptor::Message(typ));
            self.proto_types.insert(msg_pt.proto_name(), msg_pt);
        }

        let (crate_name, _) = self.crate_from_proto_filename(proto_file.get_name());

        for (path, typ, _) in messages {
            let fq_msg = (
                proto_file.get_name().to_owned(),
                path.iter()
                    .copied()
                    .chain(iter::once(typ.get_name()))
                    .collect::<Vec<_>>()
                    .join("_"),
            );
            self.calc_impls(&proto_file, &crate_name, &typ, fq_msg);
        }

        if !self.deps_map.borrow().contains_key(&crate_name) {
            for (path, field, _) in extensions {
                for type_name in &[field.type_name.as_ref(), field.extendee.as_ref()] {
                    if let Some(type_name) = type_name {
                        let field_type = self.find(type_name);
                        let (dep_crate, _) = self.crate_from_proto_filename(field_type.proto_file.get_name());
                        if dep_crate != crate_name {
                            self.deps_map
                                .borrow_mut()
                                .get_mut(&crate_name)
                                .unwrap()
                                .insert(dep_crate);
                        }
                    }
                }
            }
        }
    }

    fn find(&self, typename: &str) -> &ProtoType<'a> {
        self.proto_types.get(typename).unwrap_or_else(|| {
            panic!(
                "Could not find type by proto name: {}, {:?}",
                typename,
                self.proto_types.keys().collect::<Vec<_>>()
            )
        })
    }

    fn _set_boxed_if_recursive(&self, visited: &mut HashSet<String>, looking_for: &str, pt: &ProtoType) -> bool {
        visited.insert(pt.proto_name());
        let mut any_field_boxed = false;
        for field in &pt.msg_typ().field {
            if field.get_type_() == FieldDescriptorProto_Type::TYPE_MESSAGE
                && field.get_label() != FieldDescriptorProto_Label::LABEL_REPEATED
            {
                let mut need_box = false;
                if !visited.contains(field.get_type_name()) {
                    need_box = self._set_boxed_if_recursive(visited, looking_for, self.find(field.get_type_name()));
                }
                if need_box || field.get_type_name() == looking_for {
                    //field.options.mut_extensions().set_box_it(true);
                    // TODO
                    any_field_boxed = true;
                }
            }
        }
        any_field_boxed
    }

    // This will recursively descend through a message and see if there are any
    // recursive nesting. In those cases, it will automatically box the fields.
    fn set_boxed_if_recursive(&self, pt: &ProtoType) {
        let mut visited = HashSet::new();
        self._set_boxed_if_recursive(&mut visited, &pt.proto_name(), pt);
    }

    fn get_lib_and_mod_rs(&self, mod_tree: ModTree, derive_serde: bool) -> Vec<(String, String)> {
        let mut result: Vec<(String, String)> = Vec::new();

        for (crate_name, deps) in self.deps_map.borrow().iter() {
            let mut librs = String::new();
            writeln!(&mut librs, "#[macro_use]").unwrap();
            writeln!(&mut librs, "extern crate lazy_static;").unwrap();
            if derive_serde {
                writeln!(&mut librs, "#[macro_use]").unwrap();
                writeln!(&mut librs, "extern crate serde;").unwrap();
            }
            writeln!(&mut librs, "").unwrap();

            fn mod_tree_dfs(mod_prefix_path: &str, sub_mod_tree: &ModTree) -> Vec<(String, String)> {
                let mut result: Vec<(String, String)> = Vec::new();

                if sub_mod_tree.0.is_empty() {
                    return result;
                }

                let filename = format!("{}/mod.rs", mod_prefix_path);
                let content = concat!("// @", "generated, do not edit\n\n").to_string()
                    + &sub_mod_tree
                        .0
                        .keys()
                        .map(|k| format!("pub mod {};\n", k))
                        .collect::<String>();
                result.push((filename, content));

                for (child_mod_name, child_mod_tree) in sub_mod_tree.0.iter() {
                    for res in mod_tree_dfs(&format!("{}/{}", mod_prefix_path, child_mod_name), child_mod_tree) {
                        result.push(res);
                    }
                }

                result
            }

            let crate_mod_tree = &mod_tree.0[crate_name];
            for (mod_name, child_mod_tree) in crate_mod_tree.0.iter() {
                writeln!(&mut librs, "pub mod {};", mod_name).unwrap();

                for res in mod_tree_dfs(&format!("{}/src/{}", crate_name, mod_name), child_mod_tree) {
                    result.push(res);
                }
            }

            let filename = format!("{}/src/lib.rs", crate_name);
            let content = format!("{}{}{}", RS_HEADER, LIB_RS_HEADER, librs);
            result.push((filename, content));
        }

        result
    }

    fn get_build_file(&self) -> Vec<(String, String)> {
        let mut results = vec![];
        for crate_ in self.deps_map.borrow().keys() {
            let build_file = BUILD_TEMPLATE.replace("{crate}", crate_);
            results.push((crate_.clone(), build_file));
        }
        results
    }
    fn get_spec_toml_file(&self, derive_serde: bool, include_sso: bool) -> Vec<(String, String)> {
        let mut results = Vec::new();

        for (crate_name, deps) in self.deps_map.borrow().iter() {
            let mut all_deps: HashSet<&str> = ["lazy_static", "pb-jelly"]
                .iter()
                .copied()
                .chain(deps.iter().map(|dep| dep.as_str()))
                .collect::<HashSet<_>>();

            all_deps.remove("std");

            let mut features: HashMap<&str, &str> = [
                ("serde", r#"features=["serde_derive"]"#),
                ("compact_str", r#"features=["bytes"]"#),
            ]
            .iter()
            .cloned()
            .collect();

            if derive_serde {
                all_deps.insert("serde");
            }

            if include_sso {
                all_deps.insert("compact_str");

                if derive_serde {
                    features.insert("compact_str", r#"features=["bytes", "serde"]"#);
                }
            }

            let deps_str = all_deps
                .iter()
                .map(|dep| format!("{dep} = {{{feat}}}", dep = dep, feat = features.get(dep).unwrap_or(&"")))
                .collect::<Vec<_>>()
                .join("\n");

            let targets = SPEC_TOML_TEMPLATE
                .replace("{crate}", crate_name)
                .replace("{deps}", &deps_str);

            results.push((crate_name.clone(), targets));
        }

        results
    }
    fn get_cargo_toml_file(&self, derive_serde: bool, include_sso: bool) -> Vec<(String, String)> {
        let mut result = Vec::new();

        for (crate_name, deps) in &*self.deps_map.borrow() {
            let mut all_deps: BTreeSet<&str> = deps.iter().map(|dep| dep.as_str()).collect();
            all_deps.insert("lazy_static");
            all_deps.insert("pb-jelly");
            if let Some(extra_crates) = self.extra_crates.get(crate_name) {
                all_deps.extend(extra_crates.iter().map(|dep| dep.as_str()));
            }
            all_deps.remove("std");

            let mut features: BTreeMap<&str, String> = BTreeMap::new();
            features.insert("serde", "features=[\"serde_derive\"]".to_string());
            features.insert("compact_str", "features=[\"bytes\"]".to_string());

            if derive_serde {
                all_deps.insert("serde");
            }
            if include_sso {
                all_deps.insert("compact_str");
                if derive_serde {
                    features.insert("compact_str", "features=[\"bytes\", \"serde\"]".to_string());
                }
            }

            let mut versions: HashMap<&str, String> = HashMap::new();
            versions.insert("lazy_static", "version = \"1.4.0\"".to_string());
            versions.insert("pb-jelly", "version = \"0.0.13\"".to_string());
            versions.insert("serde", "version = \"1.0\"".to_string());
            versions.insert("bytes", "version = \"1.0\"".to_string());
            versions.insert("compact_str", "version = \"0.5\"".to_string());

            let mut deps_lines = Vec::new();
            for dep in all_deps {
                let mut fields = Vec::new();
                if let Some(feature) = features.get(&dep) {
                    fields.push(feature.to_owned());
                }
                if let Some(version) = versions.get(&dep) {
                    fields.push(version.to_owned());
                } else {
                    fields.push(format!("path = \"../{}\"", dep));
                }
                deps_lines.push(format!("{} = {{{}}}", dep, fields.join(",")));
            }

            let targets = CARGO_TOML_TEMPLATE
                .replace("{crate}", crate_name)
                .replace("{deps}", &deps_lines.join("\n"));
            result.push((crate_name.clone(), targets));
        }

        result
    }

    fn crate_from_proto_filename(&self, proto_filename: &str) -> (String, Vec<String>) {
        let filename = proto_filename.replace(&self.prefix_to_clear, "").replace(".proto", "");

        let mod_parts_unsanitized = filename.split("/");
        let mod_parts: Vec<String> = mod_parts_unsanitized
            .map(|mod_part| {
                if RESERVED_KEYWORDS.contains(&mod_part) {
                    format!("{}_", mod_part)
                } else {
                    mod_part.to_string()
                }
            })
            .collect();

        if self.crate_per_dir {
            let crate_name = format!("proto_{}", mod_parts[..mod_parts.len() - 1].join("_"));
            return (crate_name, vec![mod_parts[mod_parts.len() - 1].clone()]);
        }

        let crate_name = format!("proto_{}", mod_parts[0]);
        (crate_name, mod_parts[1..].to_vec())
    }
}

const BUILD_TEMPLATE: &str = r#"
package(default_visibility = ["//visibility:public"])

rust_library(
    name = "{crate}",
    crate_type = "lib",
    edition = "2018",
)


rust_doc(
    name = "{crate}_doc",
    crate = ":{crate}",
    edition = "2018",
)
"#;

const SPEC_TOML_TEMPLATE: &str = concat!(
    "# @",
    r#"generated, do not edit
[package]
name = "{crate}"
edition = "2018"

[dependencies]
{deps}
"#
);

const CARGO_TOML_TEMPLATE: &str = concat!(
    "# @",
    r#"generated, do not edit
[package]
name = "{crate}"
version = "0.0.1"
edition = "2018"

[dependencies]
{deps}
"#
);

const RS_HEADER: &str = concat!("// @", "generated, do not edit\n");

const LIB_RS_HEADER: &str = r#"#![warn(rust_2018_idioms)]
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

"#;

fn generate_single_crate(
    ctx: &mut Context,
    file_prefix: &str,
    file_to_generate: &[&str],
    request: &plugin::CodeGeneratorRequest,
    response: &mut plugin::CodeGeneratorResponse,
) {
    let mut mod_tree = ModTree(BTreeMap::new());

    let mut processed_files: BTreeSet<String> = BTreeSet::new();
    let mut derive_serde = false;
    let mut include_sso = false;

    for &proto_file_name in file_to_generate.iter() {
        // Detect packages which collide with filenames. The rust codegen does not support those due
        // to the rust module structure.
        //
        // eg. edgestore/engine.proto and edgestore/engine/service.proto
        // engine would be both a file and container module
        let package_path = proto_file_name.rsplitn(2, '/').nth(0).unwrap();
        if processed_files.contains(package_path) {
            panic!(
                "Unable to process proto {}. It collides with package {}.",
                proto_file_name, package_path
            );
        }
        processed_files.insert(proto_file_name[..proto_file_name.len() - 6].to_owned()); // Strip the .proto

        let (crate_name, mod_parts) = ctx.crate_from_proto_filename(proto_file_name);
        let parent_mods = &mod_parts[..mod_parts.len() - 1];
        let mod_name = if mod_parts.is_empty() {
            &crate_name
        } else {
            &mod_parts[mod_parts.len() - 1]
        };

        let mut add_mod = |writer: &mut CodeWriter| {
            let rs_file_name = format!("{}/{}/src/{}.rs", file_prefix, crate_name, writer.mod_parts.join("/"));

            response.file.push(plugin::CodeGeneratorResponse_File {
                name: Some(rs_file_name),
                content: Some(format!("{}{}", RS_HEADER, writer.content)),
                ..Default::default()
            });

            let mut curr = mod_tree.0.entry(crate_name.clone()).or_default();
            for mod_name in writer.mod_parts {
                curr = curr.0.entry(mod_name.to_owned()).or_default();
            }
        };

        // Now generate code!
        let proto_file = request
            .proto_file
            .iter()
            .find(|f| f.get_name() == proto_file_name)
            .unwrap();
        let mod_parts = &[parent_mods, &[mod_name.clone()]].concat();
        let mut writer = CodeWriter::new(ctx, proto_file, &crate_name, mod_parts);
        if writer.derive_serde {
            derive_serde = true;
        }

        let WalkResult {
            enums,
            messages,
            extensions,
        } = walk(proto_file);

        for (path, enum_typ, scl) in &enums {
            writer.gen_enum(path, enum_typ, scl);
            writer.write("");
        }

        for (path, msg_typ, scl) in &messages {
            ctx.set_boxed_if_recursive(&ProtoType::new(
                ctx,
                proto_file,
                path.clone(),
                ProtoTypeDescriptor::Message(msg_typ),
            ));
            writer.ctx = &*ctx;
            writer.gen_msg(path, msg_typ, scl);
            writer.write("");
        }

        for (path, extension_field, scl) in &extensions {
            writer.gen_extension(path, extension_field, scl);
            writer.write("");
        }

        add_mod(&mut writer);

        // check if the writer ever used a small string optimization
        if writer.uses_sso {
            include_sso = true;
        }
    }
    // Note that output filenames must use "/" even on windows. It is part of the
    // protoc plugin protocol. The plugin speaks os-independent in "/". Thus, we
    // should not use std::path::Path::new() or std::path::PathBuf::push()

    for (filename, content) in ctx.get_lib_and_mod_rs(mod_tree, derive_serde) {
        response.file.push(plugin::CodeGeneratorResponse_File {
            name: Some(file_prefix.to_owned() + &filename),
            content: Some(content.to_owned()),
            ..Default::default()
        });
    }

    if request.get_parameter().contains(&"generate_build_files".to_owned()) {
        for (crate_, build_file) in ctx.get_build_file() {
            response.file.push(plugin::CodeGeneratorResponse_File {
                name: Some(file_prefix.to_owned() + &crate_ + "/BUILD.in-gen-proto~"),
                content: Some(build_file.to_owned()),
                ..Default::default()
            });
        }
    } else if request.get_parameter().contains(&"generate_spec_toml".to_owned()) {
        for (crate_, spec_toml_file) in ctx.get_spec_toml_file(derive_serde, include_sso) {
            response.file.push(plugin::CodeGeneratorResponse_File {
                name: Some(file_prefix.to_owned() + &crate_ + "/Spec.toml"),
                content: Some(spec_toml_file.to_owned()),
                ..Default::default()
            });
        }
    } else {
        // Generate good ol Cargo.toml files
        for (crate_, cargo_toml_file) in ctx.get_cargo_toml_file(derive_serde, include_sso) {
            response.file.push(plugin::CodeGeneratorResponse_File {
                name: Some(file_prefix.to_owned() + &crate_ + "/Cargo.toml"),
                content: Some(cargo_toml_file.to_owned()),
                ..Default::default()
            });
        }
    }
}
fn generate_code(request: &plugin::CodeGeneratorRequest, response: &mut plugin::CodeGeneratorResponse) {
    let to_generate: Vec<&str> = request.get_file_to_generate().iter().map(|s| s.as_str()).collect();

    let mut prefix_to_clear = String::new();
    if request.get_parameter().contains("prefix_to_clear") {
        let args: Vec<&str> = request.get_parameter().split(' ').collect();
        for arg in args {
            if arg.contains("prefix_to_clear") {
                prefix_to_clear = arg.split('=').nth(1).unwrap().to_string();
                break;
            }
        }
    }

    if request.get_parameter().contains("crate_per_dir") {
        let mut files_by_dir: IndexMap<String, Vec<&str>> = IndexMap::new();
        for file_path in to_generate {
            let (dir_path, file_name) = file_path.rsplit_once("/").unwrap_or(("", file_path));
            files_by_dir.entry(dir_path.to_string()).or_default().push(file_path);
        }

        let mut sorted_files_by_dir: Vec<_> = files_by_dir.into_iter().collect();
        sorted_files_by_dir.sort_by_key(|(dir_path, _)| dir_path.clone());

        for (dir_path, to_generate) in sorted_files_by_dir {
            let file_prefix = dir_path
                .replace(&prefix_to_clear, "")
                .split('/')
                .next()
                .unwrap()
                .to_string()
                + "/";
            let mut ctx = Context::new(true, prefix_to_clear.clone());
            for proto_file in request.get_proto_file() {
                ctx.feed(proto_file, &to_generate);
            }
            generate_single_crate(&mut ctx, &file_prefix, &to_generate, request, response);
        }
    } else {
        let mut ctx = Context::new(false, prefix_to_clear.clone());
        for proto_file in request.get_proto_file() {
            ctx.feed(proto_file, &to_generate);
        }
        generate_single_crate(&mut ctx, "", &to_generate, request, response);
    }
}

fn main() -> io::Result<()> {
    // Read request message from stdin
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data)?;

    // Parse request
    let request = plugin::CodeGeneratorRequest::deserialize_from_slice(&data)?;

    // Create response
    let mut response = plugin::CodeGeneratorResponse::default();
    response.supported_features = Some(plugin::CodeGeneratorResponse_Feature::FEATURE_PROTO3_OPTIONAL.value() as u64);

    // Generate code
    generate_code(&request, &mut response);

    // Serialise response message
    let output = response.serialize_to_vec();
    // Write to stdout
    io::stdout().write_all(&output)?;

    Ok(())
}
