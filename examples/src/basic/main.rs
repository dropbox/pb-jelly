use pb::Message;
use proto_user::user::HelloUser;

fn main() -> std::io::Result<()> {
    // Say hello!
    let msg = HelloUser {
        name: "Parker".to_string(),
    };

    // Serialize our message to bytes
    let se_msg = msg.serialize_to_vec();
    println!("Bytes 0x{:X?}", se_msg);

    // De-serialize our message back to a Rust struct
    let de_msg: HelloUser = Message::deserialize_from_slice(&se_msg[..])?;

    // Pretty print!
    println!("Message: {:#?}", de_msg);

    // Assert our two messages are the same
    assert_eq!(msg, de_msg);

    Ok(())
}
