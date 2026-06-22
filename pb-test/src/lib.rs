#![warn(rust_2018_idioms)]

#[cfg(feature = "bench_rust_protobuf")]
pub mod gen;
#[cfg(test)]
mod pbtest;
#[cfg(test)]
mod verify_generated_files;
