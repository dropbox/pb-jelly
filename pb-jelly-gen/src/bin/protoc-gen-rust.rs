use std::io::{
    self,
    Read,
    Write,
};

use pb_jelly::Message;
use pb_jelly_gen::codegen::generate_code;
use pb_jelly_gen::protos::google::protobuf::compiler::plugin;

fn main() -> io::Result<()> {
    // Read request message from stdin
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data)?;

    // Parse request
    let request = plugin::CodeGeneratorRequest::deserialize_from_slice(&data)?;

    // Generate code
    let response = generate_code(&request);

    // Serialise response message
    let output = response.serialize_to_vec();
    // Write to stdout
    io::stdout().write_all(&output)?;

    Ok(())
}
