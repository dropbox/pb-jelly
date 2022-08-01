use proto_serde::basic::NewYorkCity;
use serde_json;

fn main() -> std::io::Result<()> {
    // Some data formatted in JSON
    let json_str = r#"
        {
            "num_windows": 42,
            "neighborhood": "West Village"
        }"#;

    // Parse the data from our JSON formatted string, into our struct
    let msg: NewYorkCity = serde_json::from_str(json_str).unwrap();

    assert_eq!(msg.num_windows, 42);
    assert_eq!(msg.neighborhood, "West Village");

    println!("{:#?}", msg);

    Ok(())
}
