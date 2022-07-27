use compact_str::ToCompactString;
use pb_jelly::Message;
use proto_sso::city::City;

fn main() -> std::io::Result<()> {
    let msg = City {
        name: "New York City".to_compact_string(),
        latitude: 40.7127837,
        longitude: -74.0059413,
        population: "8405837".to_compact_string(),
        rank: "1".to_compact_string(),
        state: "New York".to_compact_string(),
    };

    // Serialize our message to bytes
    let bytes = msg.serialize_to_vec();
    println!("Bytes 0x{:X?}", bytes);

    // De-serialize our message back to a Rust struct
    let de_msg: City = Message::deserialize_from_slice(&bytes[..])?;

    // Pretty print!
    println!("Message: {:#?}", de_msg);

    // Assert our two messages are the same
    assert_eq!(msg, de_msg);

    Ok(())
}
