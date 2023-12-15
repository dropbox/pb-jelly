// @generated, do not edit
/// Generate this field in a Box as opposed to inline Option
pub const BOX_IT: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::FieldOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50000,
        ::pb_jelly::wire_format::Type::Varint,
        "box_it",
    );

pub const GRPC_SLICES: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::FieldOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50003,
        ::pb_jelly::wire_format::Type::Varint,
        "grpc_slices",
    );

/// Use a different Rust type which implements `pb::Message` to represent the field.
/// All paths must be fully qualified, as in `::my_crate::full::path::to::type`.
/// This only works with proto3.
pub const TYPE: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::FieldOptions, ::std::string::String> =
    ::pb_jelly::extensions::SingularExtension::new(
        50004,
        ::pb_jelly::wire_format::Type::LengthDelimited,
        "type",
    );

/// Generate this `bytes` field using a Lazy<bytes::Bytes> to enable zero-copy deserialization.
pub const ZERO_COPY: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::FieldOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50007,
        ::pb_jelly::wire_format::Type::Varint,
        "zero_copy",
    );

/// Generate this `string` field using a type that supports a small string optimization.
pub const SSO: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::FieldOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50009,
        ::pb_jelly::wire_format::Type::Varint,
        "sso",
    );

/// If false, make this field's Rust type non-Optional. If the field is
/// missing on the wire during deserialization, it will remain as
/// Default::default().

/// It doesn't make sense to specify this option for a repeating field, as a
/// missing field always deserializes as an empty Vec.
/// In proto3, this option is also ignored for primitive types, which are
/// always non-nullable

/// Beware that Default may not make sense for all message types. In
/// particular, fields using `OneofOptions.nullable=false` or
/// `EnumOptions.err_if_default_or_unknown=true` will simply default to their
/// first variant, and will _not_ trigger a deserialization error. That
/// behaviour may change in the future (TODO).
pub const NULLABLE_FIELD: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::FieldOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50008,
        ::pb_jelly::wire_format::Type::Varint,
        "nullable_field",
    );

/// Setting this to true on an enum means the generated enum won't even have a value for the
/// 0-value, and any message that would've parsed to having the value be 0 fail instead.

/// If an enum field doesn't appear in the wire format of proto3, the 0 value is assumed. So if
/// an enum field is added to a message in use between a client and server, and the client hasn't
/// been recompiled, then all received messages on the server side will get the 0 value. As such,
/// it's common in cases when there's not an obvious (and safe) default value to make the 0 value
/// an explicit unknown/invalid value. In those cases, they become cumbersome to use in Rust
/// because match statements will always require a branch for the unknown/invalid case, but it's
/// more desirable to just fail at parse time. If the client has updated *past* the server, it may
/// send a value that the server does not know how to handle. We *also* fail this at parse time.
pub const ERR_IF_DEFAULT_OR_UNKNOWN: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::EnumOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50002,
        ::pb_jelly::wire_format::Type::Varint,
        "err_if_default_or_unknown",
    );

/// Setting this to true means that an enum's variants are considered exhaustive.
/// A Rust `enum` will be generated, rather than a wrapper around `i32`. This
/// makes matching against the enum simpler as there is no need to match
/// unknown variants. Instead, deserialization will fail if an unknown
/// variant is found over the wire. That makes adding or removing variants
/// potentially unsafe when it comes to version skew.

/// This option differs from `err_if_default_or_unknown` because default
/// values are still allowed. The two options are incompatible, as
/// `err_if_default_or_unknown` is strictly stronger.
pub const CLOSED_ENUM: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::EnumOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50008,
        ::pb_jelly::wire_format::Type::Varint,
        "closed_enum",
    );

/// Setting this to true adds an extra field to the deserialized message, which includes
/// a serialized representation of unrecognized fields.
/// Eg.
/// MyMessage {
///   field: u32,
///   _unrecognized: Vec<u8>,
/// }
pub const PRESERVE_UNRECOGNIZED: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::MessageOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50006,
        ::pb_jelly::wire_format::Type::Varint,
        "preserve_unrecognized",
    );

/// If false, this oneof must have a field set. Parse error if no variant (or unrecognized
/// variant) is set.
pub const NULLABLE: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::OneofOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50001,
        ::pb_jelly::wire_format::Type::Varint,
        "nullable",
    );

pub const SERDE_DERIVE: ::pb_jelly::extensions::SingularExtension<::proto_google::protobuf::descriptor::FileOptions, bool> =
    ::pb_jelly::extensions::SingularExtension::new(
        50005,
        ::pb_jelly::wire_format::Type::Varint,
        "serde_derive",
    );

