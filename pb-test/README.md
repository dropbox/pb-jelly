# `pb-test`
###### Testing 1, 2, 3... can you hear me?

This crate has integration tests to assert we properly serialize/deserialize proto messages between [`Rust`](https://www.rust-lang.org/) and [`Go`](https://golang.org/). Checked in under `/bin` are the Go serialized bytes
of our test messages, we use these binary files to assert proper ser/de without having to run Go.

Also in this crate are benchmarks around deserialization, utilizing zero-copy deserialization where we can. We benchmark our implementation of zero-copy versus 
non-zero-copy, we also benchmark against [`PROST!`](https://github.com/danburkert/prost) and [`rust-protobuf`](https://github.com/stepancheg/rust-protobuf).
