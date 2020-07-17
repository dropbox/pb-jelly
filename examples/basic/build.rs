//! Example build.rs you can use to generate Rust bindings for your protos!

use std::process::Command;

fn main() {
    Command::new("./gen_protos.sh").output().expect("failed to run gen_proto!");
}
