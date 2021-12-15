# `pb-jelly-gen`
###### It's working! It's working! - Anakin Skywalker
[![Crates.io](https://img.shields.io/crates/v/pb-jelly-gen)](https://crates.io/crates/pb-jelly-gen) [![Documentation](https://docs.rs/pb-jelly-gen/badge.svg)](https://docs.rs/pb-jelly-gen) [![Crates.io](https://img.shields.io/crates/l/pb-jelly-gen)](LICENSE)

This crate provides a tool to generate [`Rust`](https://www.rust-lang.org/) code from `proto2` or `proto3` files.

### How To Use

##### `python` + `protoc`
The core of this crate is a python script `codegen.py` that is provided to the protobuf compiler, `protoc` as a plugin.

You'll need the protobuf compiler which you can get by:
1. Running `brew install protobuf` or...
2. Download or build from source [`protobuf`](https://github.com/protocolbuffers/protobuf)

Once you've completed the above steps, you should include this crate as a build-dependency in your `Cargo.toml` and then call the API of this crate from a [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) files in the root of your repo.

##### `Cargo.toml`
```
[build-dependencies]
pb-jelly-gen = "0.0.11"
```

##### `build.rs`
```
use pb_jelly_gen::gen_protos;

fn main() -> std::io::Result<()> {
    // Replace `./protos` with a path to your proto files.
    gen_protos(vec!["./protos"])
}
```
