#![feature(test)]

extern crate pretty_assertions;
extern crate test;

#[cfg(test)]
mod bench;
#[cfg(feature = "bench_rust_protobuf")]
mod gen;
#[cfg(test)]
mod pbtest;
