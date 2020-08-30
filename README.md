# `protobuf-rs`
<p align="right"><a href="https://dropbox.tech/">by <img src="https://www.dropbox.com/s/vwygq2i9gmx60bz/dropbox_small.png?raw=1" width="32" height="32"/></a></p>

`pb-rs` is a [protobuf](https://developers.google.com/protocol-buffers) code generation framework for the [Rust language](https://www.rust-lang.org/) developed at Dropbox.

### History

This implementation was initially written in 2016 to satisfy the need of shuffling large amount
of bytes in [Dropbox's Storage System (Magic Pocket)](https://dropbox.tech/infrastructure/inside-the-magic-pocket).
Previously, we were using [`rust-protobuf`](https://github.com/stepancheg/rust-protobuf) (and therefore generated APIs are exactly
the same to make migration easy) but serializing Rust structs to proto messages, and then serializing them again in
our RPC layer, meant multiple copies (and same thing in reverse on parsing stack). Taking control of this
implementation and integrating it in our RPC stack end-to-end helped avoid these extra copies.

Over the years, the implementation has grown and matured and is currently used in several parts of Dropbox, including
our [Sync Engine](https://dropbox.tech/infrastructure/rewriting-the-heart-of-our-sync-engine), and the aforementioned [Magic Pocket](https://dropbox.tech/infrastructure/inside-the-magic-pocket).

Other implementations exist in the Rust ecosystem (e.g. [`prost`](https://github.com/danburkert/prost) and [`rust-protobuf`](https://github.com/stepancheg/rust-protobuf)), we wanted to share ours as well.

<br />

## Features
- Functional "Rust-minded" proto extensions, e.g. `[(rust.box_it)=true]`
- Scalable - Generates separate crates per module, with option for crate-per-directory
  - Autogenerates `Cargo.toml`, or optionally `Spec.toml` / [bazel](https://bazel.build/) `BUILD` files
- Support for [`Serde`](https://serde.rs/)
- Zero-copy deserialization with [`Bytes`](https://docs.rs/bytes/0.5.6/bytes/) via a proto extension `[(rust.zero_copy)=true]`
- Automatically boxes messages if it finds a recursive message definition
- Retains comments on proto fields
- Supports `proto2` and `proto3`

<br />

### Extensions

|                Extension                |                                                 Description                                                | Type  | Example |
|:---------------------------------------:|:----------------------------------------------------------------------------------------------------------:|-------|:-------:|
| `(rust.zero_copy)=true`                 | Generates field type of `Lazy<bytes::Bytes>` for proto `bytes` fields to support zero-copy deserialization | Field | [`zero_copy`](https://github.com/dropbox/pb-rs/tree/zero_copy/examples/src) |
| `(rust.box_it)=true`                    | Generates a `Box<Message>` field type                                                                      | Field | [`box_it`](https://github.com/dropbox/pb-rs/tree/zero_copy/examples/src) |
| `(rust.type)="type"`                    | Generates a custom field type                                                                              | Field | [`custom_type`](https://github.com/dropbox/pb-rs/tree/zero_copy/examples/src) |
| `(rust.preserve_unrecognized)=true`     | Preserves unrecognized proto fields into an `_unrecognized` struct field                                   | Field | `TODO` |
| `(gogoproto.nullable)=false`            | Generates non-nullable fields types									       | Field | `TODO` |
| `(rust.nullable)=false`                 | Generates oneofs as non-nullable (fail on deserialization)                                                 | Oneof | [`non_optional`](https://github.com/dropbox/pb-rs/tree/zero_copy/examples/src) |
| `(rust.err_if_default_or_unknown)=true` | Generates enums as non-zeroable (fail on deserialization)                                                  | Enum  | [`non_optional`](https://github.com/dropbox/pb-rs/tree/zero_copy/examples/src) |
| `(rust.serde_derive)=true`              | Generates serde serializable/deserializable messages                                                       | File  | [`serde`](https://github.com/dropbox/pb-rs/tree/zero_copy/examples/src) |

<br />

## Using `protobuf-rs` in your project
Multiple crates, multiple languages, my oh my!

### Essential Crates
There are only two crates you'll need if you want to use this with you project `pb-rs` and `pb-gen`. <br />

##### `pb-rs` 
Contains all of the important traits and structs that power our generated code, e.g. `Message` and `Lazy`. Include this as a `dependency`, e.g.
```
[dependencies]
pb = "0.1"
```

##### `pb-gen` 
A framework for generating Rust structs and implementations for `proto2` and `proto3` files. Include this as a `build-dependency`, e.g.
```
[build-dependencies]
pb-gen = "0.1"
```

### Generating Rust Code
In order to generate Rust code from your proto definitions you'll need three things
1. `pb-gen`
2. `protoc` - The protobuf compiler, this can be built from source [`protobuf`](https://github.com/protocolbuffers/protobuf) or installed via `brew install protobuf`.
3. `python2` - The codegen plugin used with `protoc` is written in Python2. Before running it, you'll need to install some packages, a requirements.txt is pending [#18](https://github.com/dropbox/pb-rs/issues/18).

Take a look at the [`examples`](https://github.com/dropbox/pb-rs/tree/master/examples) crate to see how we leverage `pb-gen` and `build.rs` to get started using protobufs in Rust!

<br />

#### Non-essential Crates
- `pb-test` contains integration tests and benchmarks. You don't need to worry about this one unless you want to contribute to this repository!
- `pb-types` contains generated Rust types for [well known proto types](https://developers.google.com/protocol-buffers/docs/reference/csharp/namespace/google/protobuf/well-known-types) [TODO]: Might deprecate this?

<br />

### A Note On Scalability 📝
We mention "scalabilty" as a feature, what does that mean? We take an opinionated stance that every module should be a crate, as opposed to generating Rust files 1:1 with proto files. We take this stance because `rustc` is parallel *across* crates, but not yet totally parallel *within* a crate. When we had all of our generated Rust code in a single crate, it was often that single crate that took the longest to compile. The solution to these long compile times, was creating many crates!

<br />

# Contributing

First contributions are __greatly__ appreciated and highly encouraged. For legal reasons all outside 
contributors must agree to [Dropbox's CLA](https://opensource.dropbox.com/cla/). Thank you for
your understanding.


<br />

---

# Upcoming
Some of the features here require additional tooling to be useful, which are not yet public.
- Spec.toml is a stripped down templated Cargo.toml - which you can script convert into
    Cargo.toml in order to get consistent dependency versions in a multi-crate project.
    Currently, the script to convert Spec.toml -> Cargo.toml isn't yet available
- Autogenerated BUILD files require additional tooling to convert `BUILD.in-gen-proto~` to a BUILD file

Closed structs with public fields
- Adding fields to a proto file will lead to compiler errors. This can be a benefit in that it allows the
compiler to identify all callsites that may need to be visited. However, it can make updating protos with
many callsites a bit tedious. We opted to go this route to make it easier to add a new field and update
all callsites with assistance from the compiler.

Service Generation
- Generating stubs for gPRC clients and servers

# Running the `pbtest` unit tests

1. Clone Repo.
2. Install Dependencies / Testing Dependencies. These instructions are for OSX using the brew
   package manager. Use the appropriate package manager for your system.
    - protoc - part of Google's [protobuf tools](https://github.com/protocolbuffers/protobuf/)	
        - `brew install protobuf`
    - Install Go
        - `brew install go`
    - Install [gogoproto](https://github.com/gogo/protobuf)
        - `go get github.com/gogo/protobuf/proto`	
    - Install Python & dependencies
        - `brew install python3`
        - `pip install six`
        - `pip install protobuf`
    - Generate test protos
        - On OSX: you'll have to install coreutils for realpath `brew install coreutils`
        - `./gen_protos.sh`
3. **pb-rs** currently uses an experimental test framework that requires a nightly build of rust.
	-  `rustup default nightly`
4. `cargo test`



### TODO

- [ ] Document and add tests for `grpc_slices`
- [ ] Add `custom_type` example to pbtest
- [ ] Add travis-ci integration
- [ ] Mypy the codegen.py in CI
- [ ] rustfmt in CI
- [ ] Add service generation codegen as well
- [ ] Rename pbtest.proto to pbtest2.proto
- [ ] Create github issues for remaining todos
- [ ] Run mypy-protobuf for the tests - to better mypy codegen.py
- [ ] Autogenerate warn on rust 2018 idioms
- [ ] Add test files w/ multiple generated crates that depend on each other
- [ ] Figure out how to host documentation somewhere
- [ ] Bonus: Port over the test which serializes in go and deserializes/reserializes in rust
- [ ] Bonus: Python3
- [ ] Open source oxidize (or something of the sort)

## Contributors

### Dropboxers
- [@nipunn1313](https://github.com/nipunn1313)
- [@rajatgoel](https://github.com/rajatgoel)
- [@rbtying](https://github.com/rbtying)
- [@goffrie](https://github.com/goffrie)
- [@euroelessar](https://github.com/euroelessar)
- [@bradenaw](https://github.com/bradenaw)
- [@aaronp]
- [@jiayixu](https://github.com/jiayixu)
- [@dyv](https://github.com/dyv)
- [@joshuawarner32](https://github.com/joshuawarner32)
- [@peterlvilim](https://github.com/peterlvilim)
- [@ddeville](https://github.com/ddeville)
- [@isho](https://github.com/isho)
- [@ParkMyCar](https://github.com/ParkMyCar)

## Similar Projects
[`rust-protobuf`](https://github.com/stepancheg/rust-protobuf) - Rust implementation of Google protocol buffers <br />
[`prost`](https://github.com/danburkert/prost) - PROST! a Protocol Buffers implementation for the Rust Language <br />
[`quick-protobuf`](https://github.com/tafia/quick-protobuf) - A rust implementation of protobuf parser <br />
[`serde-protobuf`](https://github.com/dflemstr/serde-protobuf)
