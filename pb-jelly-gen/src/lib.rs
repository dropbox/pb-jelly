//! `pb-jelly-gen` generates Rust bindings for `proto` files. It's intended to be used with [`pb-jelly`](https://github.com/dropbox/pb-jelly).
//!
//! ## Examples
//! Complete examples can be found in the [`examples`](https://github.com/dropbox/pb-jelly/tree/main/examples) crate,
//! or the [`pb-test`](https://github.com/dropbox/pb-jelly/tree/main/pb-test) crate of the [`pb-jelly`](https://github.com/dropbox/pb-jelly) workspace.
//!
//! ## In a nutshell 🥜
//! You can include `pb-jelly-gen` in your Cargo project, by including it as a `[build-dependency]` in your `Cargo.toml`
//! ```toml
//! [build-dependencies]
//! pb-jelly-gen = "0.0.16"
//! ```
//!
//! Then from a [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) script, use either the `GenProtos` builder struct,
//! or the `gen_protos` convience function to specify where your protos live, and where the generated code should be
//! put.
//! ```no_run
//! use pb_jelly_gen::GenProtos;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    GenProtos::builder()
//!        // output path for our generated code
//!        .out_path("./gen")
//!        // directory where our protos live
//!        .src_path("./protos")
//!        // delete and recreate the `out_path` directory every time
//!        .cleanup_out_path(true)
//!        .gen_protos()?;
//!
//!    Ok(())
//! }
//! ```

pub mod codegen;
/// Don't depend on this module, it's only public so that `protoc-gen-jellyrust` can use it
#[doc(hidden)]
#[rustfmt::skip]
pub mod protos;

#[cfg(feature = "generate")]
mod generate;
#[cfg(feature = "generate")]
pub use crate::generate::*;
