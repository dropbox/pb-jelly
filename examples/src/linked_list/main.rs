use pb_jelly::Message;
use proto_linked_list::node::Node;

fn main() -> std::io::Result<()> {
    // Creat our head node
    let mut head = Node {
        content: "Hello".to_owned(),
        next: None,
    };
    // Create a second node
    let next_node = Node {
        content: "World".to_owned(),
        next: None,
    };
    // Set next_node to be head.next
    head.next.replace(Box::new(next_node));

    // Serialize to bytes
    let bytes = head.serialize_to_vec();

    // Deserialize
    let list: Node = Message::deserialize_from_slice(&bytes[..])?;

    // Pretty print!
    println!("{:#?}", list);

    Ok(())
}