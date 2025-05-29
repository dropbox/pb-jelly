#![warn(rust_2018_idioms)]

#[cfg(test)]
mod bench;
#[cfg(feature = "bench_rust_protobuf")]
mod gen;
#[cfg(test)]
mod pbtest;
#[cfg(test)]
mod verify_generated_files;
