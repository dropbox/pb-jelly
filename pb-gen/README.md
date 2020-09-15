# `pb-gen`
###### It's working! It's working! - Anakin Skywalker

This crate provides a tool to generate [`Rust`](https://www.rust-lang.org/) code from `proto2` or `proto3` files. 

### How To Use

##### `python` + `protoc`
The core of this crate is a python2 script `codegen.py` that is provided to the protobuf compiler, `protoc` as a plugin.
There are a few python depdendencies which can be installed by running one of two commands:
1. Running `pip install six protobuf typing` or...
2. Running `pip install -r requirements.txt` from the root of this crate, if you have it cloned.

You'll also need the protobuf compiler which you can get by:
1. Running `brew install protobuf` or...
2. Build from source [`protobuf`](https://github.com/protocolbuffers/protobuf)

Once you've completed the above steps, you should include this crate as a build-dependency in your `Cargo.toml` and then call the API of this crate from a [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) files in the root of your repo.

##### `Cargo.toml`
```
[build-dependencies]
pb-gen = "0.1"
```

##### `build.rs`
```
use pb_gen::gen_protos;

fn main() -> std::io::Result<()> {
    // Replace `./protos` with a path to your proto files.
    gen_protos(vec!["./protos"])
}
```
