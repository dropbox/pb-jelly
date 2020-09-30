use pb_jelly::Message;
use proto_serde::basic::NewYorkCity;
use serde_json;

fn main() -> std::io::Result<()> {
    // Some data formatted in JSON
    let json_str = r#" 
        {
            "num_windows": 42
        }"#;

    // Parse the data from our JSON formatted string, into our struct
    let msg_json: NewYorkCity = serde_json::from_str(json_str).unwrap();

    // Create the struct via Rust
    let msg = NewYorkCity { num_windows: 42 };

    // Serialize both messages to bytes
    let msg_json_bytes = msg_json.serialize_to_vec();
    let msg_bytes = msg.serialize_to_vec();

    // Print both serialized bytes formatted as hex
    println!("JSON Data Bytes: {:X?}", msg_json_bytes);
    println!("Rust Struct Bytes: {:X?}", msg_bytes);

    Ok(())
}
