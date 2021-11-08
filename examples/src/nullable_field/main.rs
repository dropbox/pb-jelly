use pb_jelly::Message;
use proto_nullable_field::basic::{
    Inner,
    NonNullableMessage,
    PlainMessage,
};

/// Shows comparison between a plain message (regular fields)
/// and non-nullable fields via field option rust.nullable
///
/// With [(rust.nullable)=false], pb-jelly will generate
/// structs fields MessageType rather than Option<MessageType>
///
/// From protos/nullable_field/basic.proto
/// message NonNullableMessage {
///     Inner field1_nonnullable = 1 [(rust.nullable_field)=false];
///     Inner field2 = 2 [(rust.nullable_field)=true];
///     Inner field3 = 3;  // Messages default to nullable
/// }
///
/// This is beneficial when you know that the message field
/// semantically must be set - as you get simpler generated code.
///
/// However, this loses expressiveness, as
/// None and MessageType::default(), though they can be represented
/// distinguishably on the wire, will be interpreted equivalently
/// - as shown in the example.
fn main() {
    // When all fields are set (non null), they operate equivalently.
    let plain1 = PlainMessage {
        field1: Some(Inner { x: 1 }),
        field2: Some(Inner { x: 1 }),
        field3: Some(Inner { x: 1 }),
    };
    let nn1 = NonNullableMessage {
        // [(rust.nullable_field)=false]
        field1_nonnullable: Inner { x: 1 },
        // [(rust.nullable_field)=true]
        field2: Some(Inner { x: 1 }),
        field3: Some(Inner { x: 1 }),
    };
    assert_eq!(plain1.serialize_to_vec(), nn1.serialize_to_vec());

    // When fields are None, there is difference in operation
    let plain2 = PlainMessage {
        field1: None,
        field2: None,
        field3: None,
    };
    let plain2_serialized = plain2.serialize_to_vec();
    let nn2 = NonNullableMessage {
        field1_nonnullable: Inner { x: 0 },
        field2: None,
        field3: None,
    };
    let nn2_serialized = nn2.serialize_to_vec();

    // Serialization is not equivalent - default value and nil are distinguishable
    assert_eq!(plain2_serialized, vec![]);
    assert_eq!(nn2_serialized, vec![10, 0]);

    // Both wire formats deserialize equivalently for NonNullable message
    // despite being different on the wire
    assert_eq!(
        NonNullableMessage::deserialize_from_slice(&plain2_serialized).unwrap(),
        NonNullableMessage::deserialize_from_slice(&nn2_serialized).unwrap(),
    );

    // However, they deserialize differently for Plain message - field1 being
    // None vs Some(Inner {x: 0})
    assert_ne!(
        PlainMessage::deserialize_from_slice(&plain2_serialized).unwrap(),
        PlainMessage::deserialize_from_slice(&nn2_serialized).unwrap(),
    );
}
