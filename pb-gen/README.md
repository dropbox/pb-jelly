# `pb-gen`
###### It's working! It's working! - Anakin Skywalker

This crate provides a tool to generate [`Rust`](https://www.rust-lang.org/) code from `proto2` or `proto3` files. You should include this crate as a build-dependency in your `Cargo.toml` and then call the API of this crate from a [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) files in the root of your repo.

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
