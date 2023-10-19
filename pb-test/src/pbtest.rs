use std::fs::metadata;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;

use bytes::Bytes;
use pb_jelly::extensions::Extensible;
use pb_jelly::reflection::FieldMut;
use pb_jelly::wire_format::Type;
use pb_jelly::{
    ClosedProtoEnum,
    Label,
    Message,
    OpenProtoEnum,
    Reflection,
};
use pretty_assertions::{
    assert_eq,
    assert_ne,
};
use proto_pbtest::extensions;
use proto_pbtest::pbtest2::*;
use proto_pbtest::pbtest3::*;

fn get_go_proto_bytes(name: &str) -> ::std::vec::Vec<u8> {
    let filename = format!("bin/{}.bin", name);
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn succeeds<S: AsRef<[u8]> + Clone, M: Message + PartialEq>(s: S, expected: M) {
    let parsed = M::deserialize_from_slice(s.as_ref()).unwrap();

    assert_eq!(expected, parsed);

    let serialized = {
        let mut v = ::std::vec::Vec::new();
        expected.serialize(&mut Cursor::new(&mut v)).unwrap();
        v
    };

    assert_eq!(&s.as_ref()[..], &serialized[..]);
    assert_eq!(s.as_ref().len(), expected.compute_size());
}

#[test]
fn empty_string() {
    // non nullable oneof is expected to be set to default
    succeeds(b"\xb2\x03\x00\xb2\x04\x00", TestMessage::default());
}

#[test]
fn one_field_only() {
    let mut expected = TestMessage::default();
    expected.set_optional_int32(1234);
    succeeds(b"\x08\xd2\x09\xb2\x03\x00\xb2\x04\x00", expected);
}

#[test]
fn test_construct() {
    assert_eq!(ForeignMessage { c: Some(55) }.has_c(), true);
    assert_eq!(ForeignMessage { c: Some(55) }.get_c(), 55);
    assert_eq!(ForeignMessage { c: None }.has_c(), false);
}

#[test]
fn repeated_varint_fields() {
    let mut expected = TestMessage::default();
    expected.mut_repeated_uint32().extend(vec![1, 2, 3, 1, 2, 3]);

    // Write first 3 fields in packed format and other 3 in old repeated format
    let serialized = {
        let mut v = ::std::vec::Vec::new();

        // repeated part
        pb_jelly::wire_format::write(33, Type::Varint, &mut v).unwrap();
        pb_jelly::varint::write(1, &mut v).unwrap();

        // packed part
        pb_jelly::wire_format::write(33, Type::LengthDelimited, &mut v).unwrap();
        pb_jelly::varint::write(4, &mut v).unwrap();

        pb_jelly::varint::write(2, &mut v).unwrap();
        pb_jelly::varint::write(3, &mut v).unwrap();
        pb_jelly::varint::write(1, &mut v).unwrap();
        pb_jelly::varint::write(2, &mut v).unwrap();

        // repeated part
        pb_jelly::wire_format::write(33, Type::Varint, &mut v).unwrap();
        pb_jelly::varint::write(3, &mut v).unwrap();

        // set non-nullable oneof to a so it parses
        pb_jelly::wire_format::write(70, Type::LengthDelimited, &mut v).unwrap();
        pb_jelly::varint::write(0, &mut v).unwrap();

        v
    };

    let parsed = TestMessage::deserialize_from_slice(&serialized).unwrap();

    assert_eq!(parsed, expected);
}

#[test]
fn all_fields() {
    // Generate this test data with `bazel run //go/src/dropbox/pbtest:pbtest`
    let buf = get_go_proto_bytes("pbtest");
    assert_eq!(buf.len(), 464);

    let mut expected = TestMessage::default();
    expected.set_optional_int32(-1234);
    expected.set_optional_int64(-4_294_967_298);
    expected.set_optional_uint32(2345);
    expected.set_optional_uint64(4_294_967_299);
    expected.set_optional_fixed64(1_234_567_890);
    expected.set_optional_fixed32(12);
    expected.set_optional_sfixed64(500);
    expected.set_optional_sfixed32(22);
    expected.set_optional_double(0.2225);
    expected.set_optional_float(0.2225);
    expected.set_optional_bool(true);
    expected.set_optional_string("abc".to_owned());
    expected.set_optional_bytes(b"def".to_vec());
    expected.set_optional_foreign_message(ForeignMessage::default());
    expected.set_optional_nested_enum(TestMessage_NestedEnum::NEG);
    expected.set_optional_foreign_enum(ForeignEnum::FOREIGN_BAZ);
    expected.mut_repeated_int32().extend(vec![1, 2, 3, 4, 5]);
    expected
        .mut_repeated_int64()
        .extend(vec![1283, -129_837, 1_239_871_298, -123_871_287]);
    expected.mut_repeated_uint32().extend(vec![213, 123, 3646, 34]);
    expected.mut_repeated_uint64().extend(vec![123, 14, 123, 34324, 2]);
    expected
        .mut_repeated_fixed64()
        .extend(vec![pb_jelly::Fixed64(std::u64::MIN), pb_jelly::Fixed64(std::u64::MAX)]);
    expected
        .mut_repeated_fixed32()
        .extend(vec![pb_jelly::Fixed32(std::u32::MIN), pb_jelly::Fixed32(std::u32::MAX)]);
    expected.mut_repeated_sfixed64().extend(vec![
        pb_jelly::Sfixed64(std::i64::MIN),
        pb_jelly::Sfixed64(std::i64::MAX),
    ]);
    expected.mut_repeated_sfixed32().extend(vec![
        pb_jelly::Sfixed32(std::i32::MIN),
        pb_jelly::Sfixed32(std::i32::MAX),
    ]);
    expected.mut_repeated_double().extend(vec![0.1, 0.2, 0.3, 0.4, 0.5]);
    expected.mut_repeated_float().extend(vec![1.1, 2.2, 3.3, 4.4, 5.5]);
    expected
        .mut_repeated_bool()
        .extend(vec![false, true, false, true, true]);
    expected
        .mut_repeated_string()
        .extend(vec!["str1".to_owned(), "str2".to_owned(), "str3 str4".to_owned()]);
    expected
        .mut_repeated_bytes()
        .extend(vec![b"byte1".to_vec(), b"byte2".to_vec()]);
    expected
        .mut_repeated_foreign_message()
        .extend(vec![ForeignMessage::default()]);
    expected
        .mut_repeated_nested_enum()
        .extend(vec![TestMessage_NestedEnum::NEG]);
    expected
        .mut_repeated_foreign_enum()
        .extend(vec![ForeignEnum::FOREIGN_BAZ]);
    expected.optional_foreign_message_boxed = Some(Box::new(ForeignMessage { c: Some(-1234) }));
    expected.optional_foreign_message_nonnullable = ForeignMessage { c: Some(-1234) };
    expected.oneof_int = Some(TestMessage_OneofInt::Int1(34));
    expected.oneof_foreign = Some(TestMessage_OneofForeign::Foreign2(ForeignMessage { c: Some(-1234) }));
    expected.oneof_zero = Some(TestMessage_OneofZero::Int3(0)); // Caller set the int to 0 explicitly
    expected.oneof_null = None as ::std::option::Option<TestMessage_OneofNull>; // Caller set the message to None
    expected.oneof_unset = None as ::std::option::Option<TestMessage_OneofUnset>; // Caller never set the field
    expected.oneof_primitives = Some(TestMessage_OneofPrimitives::Bool6(false));
    // Ensure that a, b don't have variants, but c does
    expected.oneof_empty_field = TestMessage_OneofEmptyField::C(33);
    expected.oneof_empty_field = TestMessage_OneofEmptyField::B;
    expected.oneof_empty_field = TestMessage_OneofEmptyField::A;
    expected.r#type = Some(false);
    expected.r#mod = Some(TestMessage_Mod::Unsafe(3));
    expected.r#mod = Some(TestMessage_Mod::Loop(3));

    succeeds(&buf[..], expected)
}

#[test]
fn all_fields_default3() {
    let default = TestMessage3::default();
    assert_eq!(default.optional_int32, 0);
    assert_eq!(default.optional_int64, 0);
    assert_eq!(default.optional_uint32, 0);
    assert_eq!(default.optional_uint64, 0);
    assert_eq!(default.optional_fixed64, pb_jelly::Fixed64(0));
    assert_eq!(default.optional_double, 0.0);
    assert_eq!(default.optional_bool, false);
    assert_eq!(default.optional_string, "");
    assert_eq!(default.optional_bytes, b"");
    assert_eq!(default.optional_foreign_message, None);
    assert_eq!(default.optional_nested_enum, TestMessage3_NestedEnum3::FOO);
    assert_eq!(default.optional_foreign_enum, ForeignEnum3::FOREIGN3_FOO);
    assert_eq!(default.proto2_msg, None);
    assert_eq!(
        default.optional_foreign_message_boxed,
        None as ::std::option::Option<Box<ForeignMessage3>>
    );
    assert_eq!(default.optional_foreign_message_nonnullable, ForeignMessage3::default());
    assert_eq!(default.oneof_int, None as ::std::option::Option<TestMessage3_OneofInt>);
    assert_eq!(
        default.oneof_foreign,
        None as ::std::option::Option<TestMessage3_OneofForeign>
    );
    assert_eq!(
        default.oneof_zero,
        None as ::std::option::Option<TestMessage3_OneofZero>
    );
    assert_eq!(
        default.oneof_null,
        None as ::std::option::Option<TestMessage3_OneofNull>
    );
    assert_eq!(
        default.oneof_unset,
        None as ::std::option::Option<TestMessage3_OneofUnset>
    );
    assert_eq!(
        default.oneof_primitives,
        None as ::std::option::Option<TestMessage3_OneofPrimitives>
    );
    assert_eq!(default.oneof_empty_field, TestMessage3_OneofEmptyField::A);
    assert_eq!(default.nested, TestMessage3_NestedMessage::default());
    assert_eq!(
        default.nested_nullable,
        None as ::std::option::Option<TestMessage3_NestedMessage>
    );
    assert_eq!(
        default.nested_repeated,
        ::std::vec::Vec::<TestMessage3_NestedMessage>::new()
    );

    let buf = get_go_proto_bytes("pbtest3");
    assert_eq!(buf.len(), 193);

    let p2_msg = ForeignMessage { c: Some(77) };

    let mut expected = TestMessage3::default();
    expected.optional_int32 = -1;
    expected.optional_uint32 = 7;
    expected.optional_fixed64 = pb_jelly::Fixed64(33);
    expected.optional_fixed32 = pb_jelly::Fixed32(12);
    expected.optional_sfixed64 = pb_jelly::Sfixed64(500);
    expected.optional_sfixed32 = pb_jelly::Sfixed32(22);
    expected.optional_float = 1.3;
    expected.optional_bool = false;
    expected.proto2_msg = Some(p2_msg);
    expected.proto2_msg_empty = Some(ForeignMessage::default());
    expected.repeated_fixed64 = vec![pb_jelly::Fixed64(std::u64::MIN), pb_jelly::Fixed64(std::u64::MAX)];
    expected.repeated_fixed32 = vec![pb_jelly::Fixed32(std::u32::MIN), pb_jelly::Fixed32(std::u32::MAX)];
    expected.repeated_sfixed64 = vec![pb_jelly::Sfixed64(std::i64::MIN), pb_jelly::Sfixed64(std::i64::MAX)];
    expected.repeated_sfixed32 = vec![pb_jelly::Sfixed32(std::i32::MIN), pb_jelly::Sfixed32(std::i32::MAX)];
    expected.optional_foreign_message_boxed = Some(Box::new(ForeignMessage3 { c: 78 }));
    expected.optional_foreign_message_nonnullable = ForeignMessage3 { c: 78 };
    expected.oneof_int = Some(TestMessage3_OneofInt::Int1(34));
    expected.oneof_foreign = Some(TestMessage3_OneofForeign::Foreign2(ForeignMessage3 { c: 79 }));
    expected.oneof_zero = Some(TestMessage3_OneofZero::Int3(0)); // Caller set the int to 0 explicitly
    expected.oneof_null = None as ::std::option::Option<TestMessage3_OneofNull>; // Caller set the message to None
    expected.oneof_unset = None as ::std::option::Option<TestMessage3_OneofUnset>; // Caller never set the field
    expected.oneof_primitives = Some(TestMessage3_OneofPrimitives::Bool6(false));
    // Ensure that a, b don't have variants, but c does
    expected.oneof_empty_field = TestMessage3_OneofEmptyField::C(33);
    expected.oneof_empty_field = TestMessage3_OneofEmptyField::B;
    expected.oneof_empty_field = TestMessage3_OneofEmptyField::A;
    expected.nested = TestMessage3_NestedMessage {
        nested_oneof: TestMessage3_NestedMessage_NestedOneof::F(TestMessage3_NestedMessage_File {
            blocklist: vec![],
            size: 0,
        }),
    };
    expected.nested_nullable = Some(TestMessage3_NestedMessage {
        nested_oneof: TestMessage3_NestedMessage_NestedOneof::F(TestMessage3_NestedMessage_File {
            blocklist: vec![],
            size: 0,
        }),
    });
    expected.nested_repeated = vec![];
    expected.fixed_length = [0, 1, 2, 3];
    expected.fixed_length_repeated = vec![[0, 1, 2, 3], [4, 5, 6, 7]];
    expected.zero_or_fixed_length = None;
    expected.zero_or_fixed_length_repeated = vec![None, Some([0, 1, 2, 3]), None, Some([4, 5, 6, 7])];

    succeeds(&buf[..], expected);
}

#[test]
fn all_fields_reflection3() {
    let p2_msg = ForeignMessage { c: Some(77) };

    let mut expected = TestMessage3::default();
    expected.optional_int32 = -1;
    expected.optional_uint32 = 7;
    expected.optional_fixed64 = pb_jelly::Fixed64(33);
    expected.optional_fixed32 = pb_jelly::Fixed32(12);
    expected.optional_sfixed64 = pb_jelly::Sfixed64(500);
    expected.optional_sfixed32 = pb_jelly::Sfixed32(22);
    expected.optional_float = 1.3;
    expected.optional_bool = false;
    expected.proto2_msg = Some(p2_msg);
    expected.proto2_msg_empty = Some(ForeignMessage::default());
    expected.repeated_fixed64 = vec![pb_jelly::Fixed64(std::u64::MIN), pb_jelly::Fixed64(std::u64::MAX)];
    expected.repeated_fixed32 = vec![pb_jelly::Fixed32(std::u32::MIN), pb_jelly::Fixed32(std::u32::MAX)];
    expected.repeated_sfixed64 = vec![pb_jelly::Sfixed64(std::i64::MIN), pb_jelly::Sfixed64(std::i64::MAX)];
    expected.repeated_sfixed32 = vec![pb_jelly::Sfixed32(std::i32::MIN), pb_jelly::Sfixed32(std::i32::MAX)];
    expected.optional_foreign_message_boxed = Some(Box::new(ForeignMessage3 { c: 78 }));
    expected.optional_foreign_message_nonnullable = ForeignMessage3 { c: 78 };
    expected.oneof_int = Some(TestMessage3_OneofInt::Int1(34));
    expected.oneof_foreign = Some(TestMessage3_OneofForeign::Foreign2(ForeignMessage3 { c: 79 }));
    expected.oneof_zero = Some(TestMessage3_OneofZero::Int3(0)); // Caller set the int to 0 explicitly
    expected.oneof_null = None as ::std::option::Option<TestMessage3_OneofNull>; // Caller set the message to None
    expected.oneof_unset = None as ::std::option::Option<TestMessage3_OneofUnset>; // Caller never set the field
    expected.oneof_primitives = Some(TestMessage3_OneofPrimitives::Bool6(false));
    // Ensure that a, b don't have variants, but c does
    expected.oneof_empty_field = TestMessage3_OneofEmptyField::C(33);
    expected.oneof_empty_field = TestMessage3_OneofEmptyField::B;
    expected.oneof_empty_field = TestMessage3_OneofEmptyField::A;
    expected.nested = TestMessage3_NestedMessage {
        nested_oneof: TestMessage3_NestedMessage_NestedOneof::F(TestMessage3_NestedMessage_File {
            blocklist: vec![],
            size: 0,
        }),
    };
    expected.nested_nullable = Some(TestMessage3_NestedMessage {
        nested_oneof: TestMessage3_NestedMessage_NestedOneof::F(TestMessage3_NestedMessage_File {
            blocklist: vec![],
            size: 0,
        }),
    });
    expected.nested_repeated = vec![];
    expected.fixed_length = [0, 1, 2, 3];
    expected.fixed_length_repeated = vec![[0, 1, 2, 3], [4, 5, 6, 7]];
    expected.zero_or_fixed_length = None;
    expected.zero_or_fixed_length_repeated = vec![None, Some([0, 1, 2, 3]), None, Some([4, 5, 6, 7])];

    let mut serialized: ::std::collections::HashMap<u32, ::std::vec::Vec<u8>> = ::std::collections::HashMap::new();
    let oneofs = expected.descriptor().unwrap().oneofs;
    for f in expected.descriptor().unwrap().fields {
        if f.label == Label::Repeated {
            // Skip, repeated fields aren't currently supported.
            continue;
        }

        if let Some(oneof_index) = f.oneof_index {
            if expected.which_one_of(oneofs[oneof_index as usize].name) != Some(f.name) {
                // Not set to this oneof variant, skip this field.
                continue;
            }
        }

        match expected.get_field_mut(f.name) {
            FieldMut::Value(field) => {
                serialized.insert(f.number, field.erased_serialize());
            },
            _ => (),
        };
    }

    let mut deserialized = TestMessage3::default();
    // Repeated fields aren't currently supported, manually set them to the correct values.
    deserialized.repeated_fixed64 = expected.repeated_fixed64.clone();
    deserialized.repeated_fixed32 = expected.repeated_fixed32.clone();
    deserialized.repeated_sfixed64 = expected.repeated_sfixed64.clone();
    deserialized.repeated_sfixed32 = expected.repeated_sfixed32.clone();
    deserialized.nested_repeated = expected.nested_repeated.clone();
    deserialized.fixed_length_repeated = expected.fixed_length_repeated.clone();
    deserialized.zero_or_fixed_length_repeated = expected.zero_or_fixed_length_repeated.clone();
    for f in deserialized.descriptor().unwrap().fields {
        if f.label == Label::Repeated {
            // Skip, repeated fields aren't currently supported.
            continue;
        }

        if f.oneof_index.is_some() {
            if !serialized.contains_key(&f.number) {
                // Not set to this oneof variant, skip this field.
                continue;
            }
        }

        match deserialized.get_field_mut(f.name) {
            FieldMut::Value(field) => {
                field.erased_deserialize(serialized.get(&f.number).unwrap()).unwrap();
            },
            _ => (),
        };
    }

    assert_eq!(deserialized, expected);
}

#[test]
fn missing_non_nullable_oneof_fails_parse() {
    let buf = get_go_proto_bytes("pbtest3_missing_oneof");
    assert_eq!(buf.len(), 3);

    assert!(TestMessage3NonNullableOneof::deserialize_from_slice(buf.as_ref()).is_err());
}

#[test]
fn default_non_nullable_oneof() {
    let buf = get_go_proto_bytes("pbtest3_default_oneof");
    assert_eq!(buf.len(), 5);

    let expected = TestMessage3NonNullableOneof {
        non_nullable_oneof: TestMessage3NonNullableOneof_NonNullableOneof::A(0),
        other_field: 12345,
    };

    succeeds(&buf[..], expected);
}

#[test]
fn err_if_default_fails_parse() {
    let buf = get_go_proto_bytes("pbtest3_err_if_default");
    // Default values get elided in the wire format.
    assert_eq!(buf.len(), 0);

    assert!(TestMessage3ErrIfDefaultEnum::deserialize_from_slice(buf.as_ref()).is_err());
}

#[test]
fn repeated_err_if_default_fails_parse() {
    let buf = get_go_proto_bytes("pbtest3_repeated_err_if_default");
    assert_eq!(buf.len(), 4);

    assert!(TestMessage3RepeatedErrIfDefaultEnum::deserialize_from_slice(buf.as_ref()).is_err());
}

/// Test deserialize a v2 enum with v1 enum result in an error
/// Because it isn't expect a enum value that isn't defined in v1
#[test]
fn read_v1_enum_with_v2_compatibility() {
    // Test protobuf2
    {
        let buf2 = get_go_proto_bytes("pbtest2_version2enum");
        assert_ne!(buf2.len(), 0);
        let parsed1 = Version1Enum::deserialize_from_slice(buf2.as_ref()).unwrap();
        assert!(!parsed1.enum_field.unwrap().is_known());
    }
    // Test protobuf3
    {
        let buf2 = get_go_proto_bytes("pbtest3_version2enum");
        assert_ne!(buf2.len(), 0);
        let parsed1 = Version31Enum::deserialize_from_slice(buf2.as_ref()).unwrap();
        assert!(!parsed1.enum_field.is_known());
    }
}

/// Test deserialize a v1 enum with v2 enum is ok
#[test]
fn read_v2_enum_with_v1_compatibility() {
    // Test protobuf2
    {
        let buf1 = get_go_proto_bytes("pbtest2_version1enum");
        assert_ne!(buf1.len(), 0);
        let parsed2 = Version2Enum::deserialize_from_slice(buf1.as_ref()).unwrap();
        assert_eq!(parsed2.has_enum_field(), true);
        assert_eq!(parsed2.get_enum_field(), Version2Enum_TestEnum::ENUM0);
    }
    // Test protobuf3
    {
        let buf1 = get_go_proto_bytes("pbtest3_version1enum");
        let parsed2 = Version32Enum::deserialize_from_slice(buf1.as_ref()).unwrap();
        assert_eq!(parsed2.enum_field, Version32Enum_TestEnum::ENUM0);
    }
}

/// Test deserialize a v2 oneof with v1 oneof is ok
#[test]
fn read_v1_one_ofs_with_v2_compatibility() {
    // Test protobuf2 with rust null enabled
    {
        let buf2 = get_go_proto_bytes("pbtest2_version2oneof");
        assert_ne!(buf2.len(), 0);
        let parsed1 = Version1OneOf::deserialize_from_slice(buf2.as_ref()).unwrap();
        assert_eq!(parsed1.test_oneof, None);
    }
    // Test protobuf2 with rust null disabled. It should error on deserialize because of unexpected
    // field
    {
        let buf2 = get_go_proto_bytes("pbtest2_version2oneof_none_null");
        assert_ne!(buf2.len(), 0);
        let parsed1 = Version1OneOfNoneNullable::deserialize_from_slice(buf2.as_ref());
        assert!(parsed1.is_err());
    }
    // Test protobuf3 with rust null enabled. It should error on deserialize because of unexpected
    // field
    {
        let buf2 = get_go_proto_bytes("pbtest3_version2oneof");
        assert_ne!(buf2.len(), 0);
        let parsed1 = Version31OneOf::deserialize_from_slice(buf2.as_ref()).unwrap();
        assert_eq!(parsed1.test_oneof, None);
    }
    // Test protobuf3 with rust null disabled
    {
        let buf2 = get_go_proto_bytes("pbtest3_version2oneof_none_null");
        assert_ne!(buf2.len(), 0);
        let parsed1 = Version31OneOfNoneNullable::deserialize_from_slice(buf2.as_ref());
        assert!(parsed1.is_err());
    }
}

/// Test deserialize a v1 oneof with v2 oneof is ok
#[test]
fn read_v2_one_ofs_with_v1_compatibility() {
    // Test protobuf2 with rust null enabled
    {
        let buf1 = get_go_proto_bytes("pbtest2_version1oneof");
        assert_ne!(buf1.len(), 0);
        let parsed2 = Version2OneOf::deserialize_from_slice(buf1.as_ref()).unwrap();
        assert_eq!(
            parsed2.test_oneof,
            Some(Version2OneOf_TestOneof::StringOneOf(::std::string::String::from("abc")))
        );
    }
    // Test protobuf2 with rust null disabled
    {
        let buf1 = get_go_proto_bytes("pbtest2_version1oneof_none_null");
        assert_ne!(buf1.len(), 0);
        let parsed2 = Version2OneOfNoneNullable::deserialize_from_slice(buf1.as_ref()).unwrap();
        assert_eq!(
            parsed2.test_oneof,
            Version2OneOfNoneNullable_TestOneof::StringOneOf(::std::string::String::from("abc"))
        );
    }
    // Test protobuf3 with rust null enabled
    {
        let buf1 = get_go_proto_bytes("pbtest3_version1oneof");
        assert_ne!(buf1.len(), 0);
        let parsed2 = Version32OneOf::deserialize_from_slice(buf1.as_ref()).unwrap();
        assert_eq!(
            parsed2.test_oneof,
            Some(Version32OneOf_TestOneof::StringOneOf(::std::string::String::from(
                "abc"
            )))
        );
    }
    // Test protobuf3 with rust null disabled
    {
        let buf1 = get_go_proto_bytes("pbtest3_version1oneof_none_null");
        assert_ne!(buf1.len(), 0);
        let parsed2 = Version32OneOfNoneNullable::deserialize_from_slice(buf1.as_ref()).unwrap();
        assert_eq!(
            parsed2.test_oneof,
            Version32OneOfNoneNullable_TestOneof::StringOneOf(::std::string::String::from("abc"))
        );
    }
}

/// Test deserialize a v2 with v1 and vice versa is ok with proto2
#[test]
fn read_pb2_v1_v2_compatibility() {
    // Read v2 data as v1
    let buf2 = get_go_proto_bytes("pbtest_version2");
    assert_ne!(buf2.len(), 0);
    let parsed1 = Version1::deserialize_from_slice(buf2.as_ref()).unwrap();
    assert_eq!(parsed1.has_required_string(), true);
    assert_eq!(parsed1.get_required_string(), "abc");

    // Read v1 data as v2
    let buf1 = get_go_proto_bytes("pbtest_version1");
    assert_ne!(buf1.len(), 0);
    let parsed2 = Version2::deserialize_from_slice(buf1.as_ref()).unwrap();
    assert_eq!(parsed2.has_required_string(), true);
    assert_eq!(parsed2.get_required_string(), "abc");
    assert_eq!(parsed2.has_optional_int32(), false);
    assert_eq!(parsed2.has_optional_int64(), false);
    assert_eq!(parsed2.has_optional_uint32(), false);
    assert_eq!(parsed2.has_optional_uint64(), false);
    assert_eq!(parsed2.has_optional_fixed64(), false);
    assert_eq!(parsed2.has_optional_fixed32(), false);
    assert_eq!(parsed2.has_optional_sfixed64(), false);
    assert_eq!(parsed2.has_optional_sfixed32(), false);
    assert_eq!(parsed2.has_optional_double(), false);
    assert_eq!(parsed2.has_optional_bool(), false);
    assert_eq!(parsed2.has_optional_string(), false);
    assert_eq!(parsed2.has_optional_bytes(), false);
    assert_eq!(parsed2.has_optional_float(), false);
}

/// Test deserialize a v2 with v1 and vice versa is ok with proto3
#[test]
fn read_pb3_v1_v2_compatibility() {
    // Read v2 data as v1
    let buf2 = get_go_proto_bytes("pbtest3_version2");
    assert_ne!(buf2.len(), 0);
    let parsed1 = Version31::deserialize_from_slice(buf2.as_ref()).unwrap();
    assert_eq!(parsed1.optional_string1.as_str(), "abc");

    // Read v1 data as v2
    let buf1 = get_go_proto_bytes("pbtest3_version1");
    assert_ne!(buf1.len(), 0);
    let parsed2 = Version32::deserialize_from_slice(buf1.as_ref()).unwrap();
    assert_eq!(parsed2.optional_string1.as_str(), "abc");
    assert_eq!(parsed2.optional_int32, 0);
    assert_eq!(parsed2.optional_int64, 0);
    assert_eq!(parsed2.optional_uint32, 0);
    assert_eq!(parsed2.optional_uint64, 0);
    assert_eq!(parsed2.optional_fixed64, pb_jelly::Fixed64(0));
    assert_eq!(parsed2.optional_fixed32, pb_jelly::Fixed32(0));
    assert_eq!(parsed2.optional_sfixed64, pb_jelly::Sfixed64(0));
    assert_eq!(parsed2.optional_sfixed32, pb_jelly::Sfixed32(0));
    assert!(parsed2.optional_double.abs() < 0.00001);
    assert_eq!(parsed2.optional_bool, false);
    assert_eq!(parsed2.optional_string.as_str(), "");
    assert_eq!(parsed2.optional_bytes, ::std::vec::Vec::<u8>::new());
    assert!(parsed2.optional_float.abs() < 0.000001);
}

#[test]
fn err_if_default_non_default_succeeds() {
    let buf = get_go_proto_bytes("pbtest3_err_if_default_non_default");
    assert_eq!(buf.len(), 2);

    let expected = TestMessage3ErrIfDefaultEnum {
        field: TestMessage3ErrIfDefaultEnum_ErrIfDefaultEnum::THE_OTHER_ONE,
    };

    // Ensure no variant is produced for UNKNOWN_INVALID_VALUE.
    match expected.field {
        TestMessage3ErrIfDefaultEnum_ErrIfDefaultEnum::THE_OTHER_ONE => {},
    }

    succeeds(&buf[..], expected);
}

#[test]
fn repeated_err_if_default_non_default_succeeds() {
    let buf = get_go_proto_bytes("pbtest3_repeated_err_if_default_non_default");
    assert_eq!(buf.len(), 4);

    let expected = TestMessage3RepeatedErrIfDefaultEnum {
        field: vec![
            TestMessage3ErrIfDefaultEnum_ErrIfDefaultEnum::THE_OTHER_ONE,
            TestMessage3ErrIfDefaultEnum_ErrIfDefaultEnum::THE_OTHER_ONE,
        ],
    };

    succeeds(&buf[..], expected);
}

#[test]
fn closed_enum() {
    succeeds(
        &[],
        TestMessage3ClosedEnum {
            value: TestMessage3ClosedEnum_ClosedEnum::DEFAULT,
        },
    );
    succeeds(
        &TestMessage3ClosedEnum2 {
            value: TestMessage3ClosedEnum2_ClosedEnum::DEFAULT,
        }
        .serialize_to_vec(),
        TestMessage3ClosedEnum {
            value: TestMessage3ClosedEnum_ClosedEnum::DEFAULT,
        },
    );
    succeeds(
        &TestMessage3ClosedEnum2 {
            value: TestMessage3ClosedEnum2_ClosedEnum::ONE,
        }
        .serialize_to_vec(),
        TestMessage3ClosedEnum {
            value: TestMessage3ClosedEnum_ClosedEnum::ONE,
        },
    );
    TestMessage3ClosedEnum::deserialize_from_slice(
        &TestMessage3ClosedEnum2 {
            value: TestMessage3ClosedEnum2_ClosedEnum::TWO,
        }
        .serialize_to_vec(),
    )
    .expect_err("should fail");

    // Check that the codegen is actually generating a closed enum
    match TestMessage3ClosedEnum2_ClosedEnum::DEFAULT {
        TestMessage3ClosedEnum2_ClosedEnum::DEFAULT
        | TestMessage3ClosedEnum2_ClosedEnum::ONE
        | TestMessage3ClosedEnum2_ClosedEnum::TWO => (),
    }
}

//TODO: Add blob crate so we can bring this test back.
/*
#[test]
fn wrong_wireformat() {
    // (tag, expected_wireformat)
    let correct_wireformat = &[(1, Type::Varint), (8, Type::Fixed64), (14, Type::LengthDelimited)];

    for wf in &[Type::Varint, Type::Fixed64, Type::LengthDelimited] {
        for &(tag, expected) in correct_wireformat {
            let mut serialized: BlobReaderImpl = {
                let mut v = ::std::vec::Vec::new();
                pb_jelly::wire_format::write(tag, *wf, &mut v).unwrap();
                Blob::from_vec(v).into()
            };

            let mut v = TestMessage::default();
            let r = v.deserialize(&mut serialized);
            assert!(r.is_err());

            let msg = r.unwrap_err().to_string();
            if expected != *wf {
                assert!(msg.contains("expected wire_format"));
            } else {
                assert!(msg.contains("unexpected EOF") || msg.contains("failed to fill whole buffer"));
            }
        }
    }
}
*/

#[test]
fn test_enum_str_methods() {
    let nested_enum = TestMessage_NestedEnum::default();
    assert_eq!(nested_enum.name().unwrap(), "FOO");
    let errifdefault = TestMessage3ErrIfDefaultEnum_ErrIfDefaultEnum::THE_OTHER_ONE;
    assert_eq!(errifdefault.name(), "THE_OTHER_ONE");
}

#[test]
fn test_preserve_unrecognized() {
    // Serialize new version deserialize as old version
    let msg = TestPreserveUnrecognized2 {
        a_string1: "hello".to_owned(),
        a_int32: 3,
        a_int64: 4,
        a_uint32: 5,
        a_uint64: 6,
        a_fixed64: pb_jelly::Fixed64(7),
        a_fixed32: pb_jelly::Fixed32(8),
        a_sfixed64: pb_jelly::Sfixed64(9),
        a_sfixed32: pb_jelly::Sfixed32(10),
        a_double: 3.4,
        a_bool: true,
        a_string: "world".to_owned(),
        a_bytes: b"some bytes".to_vec(),
        a_float: 6.8,
        _unrecognized: vec![],
    };
    let serialized = msg.serialize_to_vec();
    let deserialized = TestPreserveUnrecognized1::deserialize_from_slice(&serialized).unwrap();
    assert_eq!(deserialized.string1, "hello".to_owned());
    assert_eq!(deserialized._unrecognized.len(), 71);

    // Reserialize old version and redeserialize as new version!
    let reserialized = deserialized.serialize_to_vec();
    assert_eq!(reserialized, serialized);
    let redeserialized = TestPreserveUnrecognized2::deserialize_from_slice(&reserialized).unwrap();
    assert_eq!(msg, redeserialized);

    let deserialized_empty = TestPreserveUnrecognizedEmpty::deserialize_from_slice(&serialized).unwrap();
    assert_eq!(deserialized_empty.serialize_to_vec(), serialized);
}

#[test]
fn test_preserve_unrecognized_sort_order() {
    // Ensure we deserialize _unrecognized in a deterministic order
    let mut msg = TestPreserveUnrecognized2::default();
    msg.a_int32 = 3;
    msg.a_int64 = 4;
    let serialized = msg.serialize_to_vec();
    assert_eq!(serialized, b"\x10\x03\x18\x04".to_vec());
    let serialized_backward = vec![serialized[2], serialized[3], serialized[0], serialized[1]];

    // Regardless of field order, it should deserialize the same into the original struct
    assert_eq!(
        TestPreserveUnrecognized2::deserialize_from_slice(&serialized).unwrap(),
        msg
    );
    assert_eq!(
        TestPreserveUnrecognized2::deserialize_from_slice(&serialized_backward).unwrap(),
        msg
    );

    // Regardless of field order, it should deserialize in deterministic sorted order into
    // _unrecognized
    assert_eq!(
        TestPreserveUnrecognized1::deserialize_from_slice(&serialized).unwrap(),
        TestPreserveUnrecognized1::deserialize_from_slice(&serialized_backward).unwrap(),
    );
}

#[test]
fn test_strings_sso3() {
    let buf = get_go_proto_bytes("pbtest3_version2");
    assert_ne!(buf.len(), 0);

    let msg = Version31SSO::deserialize_from_slice(buf.as_ref()).unwrap();
    assert_eq!(msg.optional_string1.as_str(), "abc");
}

fn check_roundtrip<M: Message>(proto: &M) {
    assert_eq!(
        M::deserialize_from_slice(&proto.serialize_to_vec()).expect("failed to deserialize"),
        *proto
    );
}

fn unwrap_field_mut<'a>(p: &'a mut impl Reflection, field_name: &str) -> &'a mut dyn Reflection {
    match p.get_field_mut(field_name) {
        FieldMut::Value(r) => r,
        _ => panic!("field {} was empty", field_name),
    }
}

#[test]
fn test_proto3_optional() {
    // Check that all-None roundtrips
    check_roundtrip(&TestProto3Optional::default());
    // Check that all default value roundtrips (which is different from None)
    check_roundtrip(&TestProto3Optional {
        a_message: Some(ForeignMessage3 { c: 0 }),
        a_int32: Some(0),
        a_int64: Some(0),
        a_uint32: Some(0),
        a_uint64: Some(0),
        a_fixed64: Some(pb_jelly::Fixed64(0)),
        a_fixed32: Some(pb_jelly::Fixed32(0)),
        a_sfixed64: Some(pb_jelly::Sfixed64(0)),
        a_sfixed32: Some(pb_jelly::Sfixed32(0)),
        a_double: Some(0.0),
        a_bool: Some(false),
        a_string: Some("".to_owned()),
        a_bytes: Some(vec![]),
        a_float: Some(0.0),
        real_oneof_1: Some(TestProto3Optional_RealOneof1::RealOneof11("".to_string())),
        real_oneof_2: TestProto3Optional_RealOneof2::RealOneof21("".to_string()),
    });
    let mut proto = TestProto3Optional {
        a_message: Some(ForeignMessage3 { c: 123 }),
        a_int32: Some(123),
        a_int64: Some(123),
        a_uint32: Some(123),
        a_uint64: Some(123),
        a_fixed64: Some(pb_jelly::Fixed64(123)),
        a_fixed32: Some(pb_jelly::Fixed32(123)),
        a_sfixed64: Some(pb_jelly::Sfixed64(123)),
        a_sfixed32: Some(pb_jelly::Sfixed32(123)),
        a_double: Some(12.3),
        a_bool: Some(true),
        a_string: Some("123".to_owned()),
        a_bytes: Some(vec![1, 2, 3]),
        a_float: Some(12.3),
        real_oneof_1: Some(TestProto3Optional_RealOneof1::RealOneof12("123".to_string())),
        real_oneof_2: TestProto3Optional_RealOneof2::RealOneof22("123".to_string()),
    };
    check_roundtrip(&proto);
    let descriptor = proto.descriptor().unwrap();
    assert_eq!(descriptor.fields.len(), 18);
    for field in descriptor.fields {
        // only real oneofs have oneof_index set
        assert_eq!(field.oneof_index.is_some(), field.name.starts_with("real_oneof"));
    }
    // only real oneofs are listed
    assert_eq!(descriptor.oneofs.len(), 2);
    assert_eq!(descriptor.oneofs[0].name, "real_oneof_1");
    assert_eq!(descriptor.oneofs[1].name, "real_oneof_2");

    // basic test for reflection
    unwrap_field_mut(&mut proto, "a_int32")
        .erased_deserialize(&456i32.serialize_to_vec())
        .unwrap();
    assert_eq!(proto.a_int32, Some(456));
}

#[test]
fn test_proto3_zerocopy_read() {
    let data = Bytes::from_static(b"\x0a\x08zerocopy\x12\x09zerocopy2");

    let mut proto = TestProto3Zerocopy::default();
    proto.deserialize(&mut Cursor::new(data.clone())).unwrap();
    let data1 = proto.data1.clone().into_buffer();
    let data2 = proto.data2.clone().into_buffer();
    assert_eq!(data1, &b"zerocopy"[..]);
    assert_eq!(data2, &b"zerocopy2"[..]);
    // The deserialized buffers should point into `data`.
    data.slice_ref(data1.as_ref());
    data.slice_ref(data2.as_ref());

    assert_eq!(proto.compute_grpc_slices_size(), "zerocopy".len() + "zerocopy2".len());
    assert_eq!(
        TestProto3ContainsZerocopy { inner: Some(proto) }.compute_grpc_slices_size(),
        "zerocopy".len() + "zerocopy2".len()
    );
}

// Test that boxing works properly for oneof fields.
#[test]
fn test_recursive_oneof() {
    let message = RecursiveOneof {
        oneof_field: Some(RecursiveOneof_OneofField::BoxedEmpty(std::default::Default::default())),
    };
    check_roundtrip(&message);
    let message = RecursiveOneof {
        oneof_field: Some(RecursiveOneof_OneofField::Field(Box::new(message))),
    };
    check_roundtrip(&message);
    let message = RecursiveOneof {
        oneof_field: Some(RecursiveOneof_OneofField::Field(Box::new(message))),
    };
    check_roundtrip(&message);
}

#[test]
fn test_mutual_recursion() {
    check_roundtrip(&MutuallyRecursiveA {
        // Both MutuallyRecursiveA::inner and MutuallyRecursiveB::inner are boxed
        inner: Some(Box::new(MutuallyRecursiveB {
            inner: Some(Box::new(MutuallyRecursiveA::default())),
        })),
    });
    check_roundtrip(&MutuallyRecursiveWithRepeatedA {
        inner: vec![
            MutuallyRecursiveWithRepeatedB {
                // MutuallyRecursiveWithRepeatedB::inner is *not* boxed
                inner: Some(MutuallyRecursiveWithRepeatedA::default()),
            },
            MutuallyRecursiveWithRepeatedB {
                inner: Some(MutuallyRecursiveWithRepeatedA::default()),
            },
        ],
    });
    check_roundtrip(&MutuallyRecursiveWithBoxedA {
        inner: Some(Box::new(MutuallyRecursiveWithBoxedB {
            // MutuallyRecursiveWithBoxedB::inner is *not* boxed
            inner: Some(MutuallyRecursiveWithBoxedA::default()),
        })),
    });
}

#[test]
fn test_extensions() {
    check_roundtrip(extensions::FakeMsg::default());
    check_roundtrip(extensions::FakeMsg {
        base_field: Some(39),
        singular_primitive: Some(123),
        singular_message: Some(ForeignMessage3 { c: 321 }),
        repeated_primitive: vec![456, 789],
        repeated_message: vec![ForeignMessage3 { c: 654 }, ForeignMessage3 { c: 987 }],
    });

    // Check that serializing a FakeMsg and deserializing into Msg preserves the extension fields,
    // and that those fields can be read using `get_extension()`.
    fn check_roundtrip(orig: extensions::FakeMsg) {
        let m = extensions::Msg::deserialize_from_slice(&orig.serialize_to_vec()).unwrap();
        assert_eq!(m.base_field, orig.base_field);
        assert_eq!(
            m.get_extension(extensions::SINGULAR_PRIMITIVE).unwrap(),
            orig.singular_primitive
        );
        assert_eq!(
            m.get_extension(extensions::SINGULAR_MESSAGE).unwrap(),
            orig.singular_message,
        );
        assert_eq!(
            m.get_extension(extensions::REPEATED_PRIMITIVE).unwrap(),
            orig.repeated_primitive,
        );
        assert_eq!(
            m.get_extension(extensions::REPEATED_MESSAGE).unwrap(),
            orig.repeated_message
        );
    }
}
