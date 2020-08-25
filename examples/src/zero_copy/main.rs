use bytes::Bytes;
use pb::{
    Blob,
    Lazy,
    Message,
};
use proto_zero_copy::basic::BytesMessage;

fn main() -> std::io::Result<()> {
    // Create 1kb of Data
    let data = Lazy::new(Bytes::from(vec![42 as u8; 1 * 1024]));

    // Create our Proto Struct
    let mut proto = BytesMessage::default();
    proto.set_data(data);
    proto.set_name("Parker".to_owned());

    // Serialize our proto
    let ser_bytes: Vec<u8> = proto.serialize_to_vec();


    // Serialized proto gets "sent" across ☁️ The Internet ☁️


    // To achieve zero copy deserialization, we need to our serialized bytes to be in a container
    // that implements the `pb::BlobReader` trait, `bytes::Bytes` implements this trait.
    //
    // Ideally your serialized bytes would already be in a `Bytes` struct, e.g. some network request you're about to
    // handle.
    let request = Bytes::from(ser_bytes);
    let mut reader = request.into_reader();

    // Deserialize our proto
    let mut de_proto = BytesMessage::default();
    de_proto.deserialize(&mut reader)?;
    // Grab the bytes from the Lazy field
    let inner_bytes: Bytes = de_proto.data.unwrap().into_blob();

    // Print our fields!
    println!("Name: {}", de_proto.name.unwrap());
    println!("{:#X}", &inner_bytes);

    Ok(())
}
