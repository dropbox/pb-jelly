# `pb-jelly`
<p align="right"><a href="https://dropbox.tech/">by <img src="https://www.dropbox.com/s/vwygq2i9gmx60bz/dropbox_small.png?raw=1" width="32" height="32"/></a></p>


`pb-jelly` is a [protobuf](https://developers.google.com/protocol-buffers) code generation framework for the [Rust language](https://www.rust-lang.org/) developed at Dropbox.


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

[![Crates.io](https://img.shields.io/crates/v/pb-jelly)](https://crates.io/crates/pb-jelly) [![Documentation](https://docs.rs/pb-jelly/badge.svg)](https://docs.rs/pb-jelly) [![Crates.io](https://img.shields.io/crates/l/pb-jelly)](LICENSE) [![Build Status](https://travis-ci.org/dropbox/pb-jelly.svg?branch=master)](https://travis-ci.org/dropbox/pb-jelly)


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
| `(rust.zero_copy)=true`                 | Generates field type of `Lazy<bytes::Bytes>` for proto `bytes` fields to support zero-copy deserialization | Field | [`zero_copy`](https://github.com/dropbox/pb-jelly/tree/master/examples/src) |
| `(rust.box_it)=true`                    | Generates a `Box<Message>` field type                                                                      | Field | [`box_it`](https://github.com/dropbox/pb-jelly/tree/master/examples/src) |
| `(rust.type)="type"`                    | Generates a custom field type                                                                              | Field | [`custom_type`](https://github.com/dropbox/pb-jelly/tree/master/examples/src) |
| `(rust.preserve_unrecognized)=true`     | Preserves unrecognized proto fields into an `_unrecognized` struct field                                   | Field | `TODO` |
| `(gogoproto.nullable)=false`            | Generates non-nullable fields types									                                       | Field | `TODO` |
| `(rust.nullable)=false`                 | Generates oneofs as non-nullable (fail on deserialization)                                                 | Oneof | [`non_optional`](https://github.com/dropbox/pb-jelly/tree/master/examples/src) |
| `(rust.err_if_default_or_unknown)=true` | Generates enums as non-zeroable (fail on deserialization)                                                  | Enum  | [`non_optional`](https://github.com/dropbox/pb-jelly/tree/master/examples/src) |
| `(rust.closed_enum)=true`               | Generates only a "closed" enum which will fail deserialization for unknown values, but is easier to work with in Rust | Enum | `TODO` |
| `(rust.serde_derive)=true`              | Generates serde serializable/deserializable messages                                                       | File  | [`serde`](https://github.com/dropbox/pb-jelly/tree/master/examples/src) |

<br />

## Using `pb-jelly` in your project
Multiple crates, multiple languages, my oh my!

### Essential Crates
There are only two crates you'll need if you want to use this with you project `pb-jelly` and `pb-jelly-gen`. <br />

##### `pb-jelly`
Contains all of the important traits and structs that power our generated code, e.g. `Message` and `Lazy`. Include this as a `dependency`, e.g.
```
[dependencies]
pb-jelly = "0.0.7"
```

##### `pb-jelly-gen`

A framework for generating Rust structs and implementations for `proto2` and `proto3` files.
In order to use pb-jelly, you need to add the pb-jelly-gen/codegen/codegen.py as a plugin to your protoc invocation.

We added some code here to handle the protoc invocation if you choose to use it.
You'll need to add a generation crate (see `examples_gen` for an example)
Include `pb-jelly-gen` as a dependency of your generation crate, and `cargo run` to invoke protoc for you.
```
[dependencies]
pb-jelly-gen = "0.0.7"
```

Eventually, we hope to eliminate the need for a generation crate, and simply have generation occur
inside a build.rs with `pb-jelly-gen` as a build dependency. However https://github.com/rust-lang/cargo/issues/8709
must be resolved first.

Note that you can always invoke protoc on your own (for example if you are already doing so to generate for multiple languages)
with `--rust_out=codegen.py` as a plugin for rust.

### Generating Rust Code
In order to generate Rust code from your proto definitions you'll need three things
1. `protoc` - The protobuf compiler, this can be built from source [`protobuf`](https://github.com/protocolbuffers/protobuf) or installed via `brew install protobuf`.
2. `python` - The codegen plugin used with `protoc` is written in Python (compatible with both py2 and py3). Before running it, you'll need to install some packages, via `python -m pip install -r pb-jelly-gen/requirements.txt`
3. `pb-jelly-gen` [optional - you may always invoke protoc on your own]

Take a look at the [`examples`](https://github.com/dropbox/pb-jelly/tree/master/examples/src) crate to see how we leverage `pb-jelly-gen` and `build.rs` to get started using protobufs in Rust!

<br />

#### Non-essential Crates
- `pb-test` contains integration tests and benchmarks. You don't need to worry about this one unless you want to contribute to this repository!
- `examples` contains some examples to help you get started

<br />

### A Note On Scalability 📝
We mention "scalabilty" as a feature, what does that mean? We take an opinionated stance that every module should be a crate, as opposed to generating Rust files 1:1 with proto files. We take this stance because `rustc` is parallel *across* crates, but not yet totally parallel *within* a crate. When we had all of our generated Rust code in a single crate, it was often that single crate that took the longest to compile. The solution to these long compile times, was creating many crates!

<br />

### The Name

pb-jelly is a shoutout to the jellyfish known for its [highly efficient locomotion](https://en.wikipedia.org/wiki/Jellyfish).
This library is capable of highly efficient locomotion of deserialized data. Also a shoutout to ability of the jellyfish
to have [substantial increases in population](https://en.wikipedia.org/wiki/Jellyfish_bloom). This library handles generating
a very large number of proto modules with complex dependencies, by generating to multiple crates.

We also like [the popular sandwich](https://en.wikipedia.org/wiki/Peanut_butter_and_jelly_sandwich).

# Contributing

First, contributions are __greatly__ appreciated and highly encouraged. For legal reasons all outside
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
        - [if necessary] `brew install python3`
        - `python3 -m pip install -r pb-jelly-gen/requirements.txt`
3. **pb-jelly** currently uses an experimental test framework that requires a nightly build of rust.
	-  `rustup default nightly`
4. `cd pb-test`
5. `( cd pb_test_gen ; cargo run ) ; cargo test`


## Contributors

### Dropboxers [incl former]
- [@nipunn1313](https://github.com/nipunn1313)
- [@rajatgoel](https://github.com/rajatgoel)
- [@ParkMyCar](https://github.com/ParkMyCar)
- [@rbtying](https://github.com/rbtying)
- [@goffrie](https://github.com/goffrie)
- [@euroelessar](https://github.com/euroelessar)
- [@bradenaw](https://github.com/bradenaw)
- [@glaysche2](https://github.com/glaysche2)
- [@jiayixu](https://github.com/jiayixu)
- [@dyv](https://github.com/dyv)
- [@joshuawarner32](https://github.com/joshuawarner32)
- [@peterlvilim](https://github.com/peterlvilim)
- [@ddeville](https://github.com/ddeville)
- [@isho](https://github.com/isho)
- [@benjaminp](https://github.com/benjaminp)
### Non-Dropbox
- [@RSMuthu](https://github.com/RSMuthu)

## Similar Projects
[`rust-protobuf`](https://github.com/stepancheg/rust-protobuf) - Rust implementation of Google protocol buffers <br />
[`prost`](https://github.com/danburkert/prost) - PROST! a Protocol Buffers implementation for the Rust Language <br />
[`quick-protobuf`](https://github.com/tafia/quick-protobuf) - A rust implementation of protobuf parser <br />
[`serde-protobuf`](https://github.com/dflemstr/serde-protobuf)
