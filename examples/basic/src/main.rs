use pb::Message;
use proto_rust::basic::basic::Basic;

fn main() {
    let msg = Basic {
        name: "Parker".to_owned(),
    };

    let bytes = msg.serialize_to_vec();
    let result: Result<Basic, std::io::Error> = Message::deserialize_from_slice(&bytes[..]);

    let deserde_msg = result.unwrap();

    assert_eq!(msg, deserde_msg);

    println!("Messages are equal!\n\nOriginal: {:#?}\nAfter: {:#?}", msg, deserde_msg);
}
