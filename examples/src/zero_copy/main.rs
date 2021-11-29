use bytes::Bytes;
use pb_jelly::{
    Lazy,
    Message,
};
use proto_zero_copy::basic::BytesMessage;
use std::io::Cursor;

fn main() -> std::io::Result<()> {
    // Create 1kb of Data
    let data = Lazy::new(Bytes::from(vec![42 as u8; 1 * 1024]));

    // Create our Proto Struct
    let mut proto = BytesMessage::default();
    proto.set_data(data);
    proto.set_name("Parker".to_owned());

    // Serialize our proto
    let ser_bytes: Vec<u8> = proto.serialize_to_vec();

    // Serialized proto gets theoretically sent across ☁️ The Internet ☁️

    // To achieve zero copy deserialization, we need to our serialized bytes to be in a container
    // with a reader that implements the `pb::PbBufferReader` trait, such as `Cursor<bytes::Bytes>`.
    //
    // Ideally your serialized bytes would already be in a `Bytes` struct, e.g. some network request you're about to
    // handle.
    let request = Bytes::from(ser_bytes);
    let mut reader = Cursor::new(request.clone());

    // Deserialize our proto
    let mut de_proto = BytesMessage::default();
    de_proto.deserialize(&mut reader)?;
    // Grab the bytes from the Lazy field
    let inner_bytes: Bytes = de_proto.data.unwrap().into_buffer();
    // Because of the coordination between the `Cursor<Bytes>` reader and the `Lazy<Bytes>` field,
    // this field contains a reference to the original data, rather than a copy!
    assert!(request.as_ptr_range().contains(&inner_bytes.as_ptr()));

    // Print our fields!
    println!("Name: {}", de_proto.name.unwrap());
    println!("{:#X}", &inner_bytes);

    Ok(())
}
