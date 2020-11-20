use pb_jelly::Message;
use proto_box_it::basic::{
    BoxedMessage,
    OuterMessage,
};

fn main() -> std::io::Result<()> {
    // Create our messages
    let maybe_msg = BoxedMessage {
        name: "Paper".to_owned(),
    };
    let msg = BoxedMessage {
        name: "Passwords".to_owned(),
    };
    let msg = OuterMessage {
        optional_msg: Some(Box::new(maybe_msg)),
        msg: Box::new(msg),
        other: "Dropbox".to_owned(),
    };

    // Serialize our message
    let se_msg = msg.serialize_to_vec();

    // Deserialize our message
    let de_msg: OuterMessage = Message::deserialize_from_slice(&se_msg[..])?;

    // Grab our inner box
    let de_box_msg: Box<BoxedMessage> = de_msg.optional_msg.unwrap();

    // Print our message!
    println!("{} {}", de_msg.other, de_box_msg.name);

    Ok(())
}
