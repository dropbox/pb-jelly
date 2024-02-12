# `pb-jelly-gen`
###### It's working! It's working! - Anakin Skywalker
[![Crates.io](https://img.shields.io/crates/v/pb-jelly-gen)](https://crates.io/crates/pb-jelly-gen) [![Documentation](https://docs.rs/pb-jelly-gen/badge.svg)](https://docs.rs/pb-jelly-gen) [![Crates.io](https://img.shields.io/crates/l/pb-jelly-gen)](LICENSE)

This crate provides a tool to generate [`Rust`](https://www.rust-lang.org/) code from `.proto` files.

### How To Use

You'll need the protobuf compiler which you can get by:
1. Running `brew install protobuf` or...
2. Download or build from source [`protobuf`](https://github.com/protocolbuffers/protobuf)

#### As a plugin for protoc

A binary is included that can be passed directly to `protoc`:

```
% cargo build --bin protoc-gen-jellyrust
% protoc --plugin=protoc-gen-jellyrust=target/debug/protoc-gen-jellyrust --jellyrust_out=out foo/bar.proto...
```

#### As a library

Add this crate as a dependency in your `Cargo.toml` and then call `gen_protos`:

##### `Cargo.toml`
```
[dependencies]
pb-jelly-gen = "0.0.16"
```

##### `main.rs`
```
use pb_jelly_gen::gen_protos;

fn main() {
    // Replace `./protos` with a path to your proto files.
    gen_protos(vec!["./protos"]).unwrap()
}
```
