use pb::Message;
use proto_box_it::basic::{
    BoxedMessage,
    OuterMessage,
};

fn main() -> std::io::Result<()> {
    // Create our messages
    let box_msg = BoxedMessage {
        name: "Paper".to_owned(),
    };
    let msg = OuterMessage {
        msg: Some(Box::new(box_msg)),
        other: "Dropbox".to_owned(),
    };

    // Serialize our message
    let se_msg = msg.serialize_to_vec();

    // Deserialize our message
    let de_msg: OuterMessage = Message::deserialize_from_slice(&se_msg[..])?;

    // Grab our inner box
    let de_box_msg: Box<BoxedMessage> = de_msg.msg.unwrap();
    
    // Print our message!
    println!("{} {}", de_msg.other, de_box_msg.name);

    Ok(())
}