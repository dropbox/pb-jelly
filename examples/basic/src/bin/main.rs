use pb::Message;
use proto_user::user::HelloUser;

fn main() -> std::io::Result<()> {
    let msg = HelloUser {
        name: "Parker".to_string()
    };

    let se_msg = msg.serialize_to_vec();
    println!("Bytes 0x{:X?}", se_msg);

    let de_res: std::io::Result<HelloUser> = Message::deserialize_from_slice(&se_msg[..]);
    let de_msg = de_res?;
    println!("Message: {:#?}", de_msg);
    
    assert_eq!(msg, de_msg);

    Ok(())
}
