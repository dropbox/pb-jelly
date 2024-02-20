// This file is generated by rust-protobuf 3.3.0. Do not edit
// .proto file is parsed by protoc 25.2
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `rust/extensions.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

/// Extension fields
pub mod exts {

    pub const box_it: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50000, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const grpc_slices: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50003, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const blob: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50010, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const type_: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, ::std::string::String> = ::protobuf::ext::ExtFieldOptional::new(50004, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_STRING);

    pub const zero_copy: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50007, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const sso: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50009, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const nullable_field: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FieldOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50008, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const err_if_default_or_unknown: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50002, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const closed_enum: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::EnumOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50008, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const preserve_unrecognized: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::MessageOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50006, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const nullable: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::OneofOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50001, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);

    pub const serde_derive: ::protobuf::ext::ExtFieldOptional<::protobuf::descriptor::FileOptions, bool> = ::protobuf::ext::ExtFieldOptional::new(50005, ::protobuf::descriptor::field_descriptor_proto::Type::TYPE_BOOL);
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15rust/extensions.proto\x12\x04rust\x1a\x20google/protobuf/descripto\
    r.proto:6\n\x06box_it\x18\xd0\x86\x03\x20\x01(\x08\x12\x1d.google.protob\
    uf.FieldOptionsR\x05boxIt:@\n\x0bgrpc_slices\x18\xd3\x86\x03\x20\x01(\
    \x08\x12\x1d.google.protobuf.FieldOptionsR\ngrpcSlices:3\n\x04blob\x18\
    \xda\x86\x03\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x04blob:\
    3\n\x04type\x18\xd4\x86\x03\x20\x01(\t\x12\x1d.google.protobuf.FieldOpti\
    onsR\x04type:<\n\tzero_copy\x18\xd7\x86\x03\x20\x01(\x08\x12\x1d.google.\
    protobuf.FieldOptionsR\x08zeroCopy:1\n\x03sso\x18\xd9\x86\x03\x20\x01(\
    \x08\x12\x1d.google.protobuf.FieldOptionsR\x03sso:L\n\x0enullable_field\
    \x18\xd8\x86\x03\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptions:\x04t\
    rueR\rnullableField:X\n\x19err_if_default_or_unknown\x18\xd2\x86\x03\x20\
    \x01(\x08\x12\x1c.google.protobuf.EnumOptionsR\x15errIfDefaultOrUnknown:\
    ?\n\x0bclosed_enum\x18\xd8\x86\x03\x20\x01(\x08\x12\x1c.google.protobuf.\
    EnumOptionsR\nclosedEnum:V\n\x15preserve_unrecognized\x18\xd6\x86\x03\
    \x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x14preserveUnrecog\
    nized:A\n\x08nullable\x18\xd1\x86\x03\x20\x01(\x08\x12\x1d.google.protob\
    uf.OneofOptions:\x04trueR\x08nullable:H\n\x0cserde_derive\x18\xd5\x86\
    \x03\x20\x01(\x08\x12\x1c.google.protobuf.FileOptions:\x05falseR\x0bserd\
    eDerive\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(::protobuf::descriptor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
