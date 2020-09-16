use pb_jelly::Message;
use proto_custom_type::custom::{UserId, UserRequest};

fn main() -> std::io::Result<()> {
    let req = UserRequest {
        user_id: Some(UserId {
            id: [0, 1, 2, 3, 4, 5, 6, 7]
        })
    };
    let bytes = req.serialize_to_vec();
    let req_de: UserRequest = Message::deserialize_from_slice(&bytes[..])?;
    
    // Our custom type!
    let user_id: [u8; 8] = req_de.user_id.unwrap().id;

    println!("Our 8 byte UserId: {:X?}", user_id);

    Ok(())
}