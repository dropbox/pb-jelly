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

use std::convert::AsRef;
use std::error::Error;
use std::fs;
use std::iter::IntoIterator;
use std::path::{
    self,
    Path,
    PathBuf,
};
use std::process::Command;

use pb_jelly::Message;
use walkdir::WalkDir;

use crate::protos::google::protobuf::compiler::plugin::CodeGeneratorRequest;
use crate::protos::google::protobuf::descriptor::FileDescriptorSet;

pub mod codegen;
/// Don't depend on this module, it's only public so that `protoc-gen-jellyrust` can use it
#[doc(hidden)]
#[rustfmt::skip]
pub mod protos;

/// A "no frills" way to generate Rust bindings for your proto files. `src_paths` is a list of
/// paths to your `.proto` files, or the directories that contain them. Generated code it outputted
/// to `<current crate's manifest>/gen`.
pub fn gen_protos<P: AsRef<Path>>(src_paths: Vec<P>) -> Result<(), Box<dyn Error>> {
    GenProtos::builder().src_paths(src_paths).gen_protos()
}

/// A builder struct to configure the way your protos are generated, create one with `GenProtos::builder()`
#[must_use]
pub struct GenProtos {
    gen_path: PathBuf,
    src_paths: Vec<PathBuf>,
    include_paths: Vec<PathBuf>,
    include_extensions: bool,
    cleanup_out_path: bool,
}

impl std::default::Default for GenProtos {
    fn default() -> Self {
        let gen_path =
            get_cargo_manifest_path().expect("couldn't get `CARGO_MANIFEST_DIR` when building default GenProtos");
        let gen_path = gen_path.join(PathBuf::from("./gen"));

        let src_paths = vec![];
        let include_paths = vec![];
        let include_extensions = true;
        let cleanup_out_path = false;

        GenProtos {
            gen_path,
            src_paths,
            include_paths,
            include_extensions,
            cleanup_out_path,
        }
    }
}

// Public functions
impl GenProtos {
    /// Create a default builder
    pub fn builder() -> GenProtos {
        GenProtos::default()
    }

    /// Set the output path for the generated code. This should be relative to the current crate's
    /// manifest.
    ///
    /// Defaults to the `<current crate's manifest>/gen`
    pub fn out_path<P: AsRef<Path>>(mut self, path: P) -> GenProtos {
        let manifest_path = get_cargo_manifest_path().expect("out_path");
        self.gen_path = manifest_path.join(path);
        self
    }

    /// Set the output path for the generate code. This will be treated as an absolute path.
    pub fn abs_out_path<P: AsRef<Path>>(mut self, path: P) -> GenProtos {
        self.gen_path = path.as_ref().to_owned();
        self
    }

    /// Add a path to a `.proto` file, or a directory containing your proto files.
    pub fn src_path<P: AsRef<Path>>(mut self, path: P) -> GenProtos {
        self.src_paths.push(path.as_ref().to_owned());
        self
    }

    /// Add a list of paths to `.proto` files, or to directories containing your proto files.
    pub fn src_paths<P: AsRef<Path>, I: IntoIterator<Item = P>>(mut self, paths: I) -> GenProtos {
        self.src_paths.extend(paths.into_iter().map(|p| p.as_ref().to_owned()));
        self
    }

    /// A path to a protobuf file, or a directory containing protobuf files, that get included in
    /// the proto compilation. Rust bindings will *not* be generated for these files, but the proto
    /// compiler will look at included paths for proto dependencies.
    pub fn include_path<P: AsRef<Path>>(mut self, path: P) -> GenProtos {
        self.include_paths.push(path.as_ref().to_owned());
        self
    }

    /// Paths to a protobuf files, or directories containing protobuf files, that get included in
    /// the proto compilation. Rust bindings will *not* be generated for these files, but the proto
    /// compiler will look at included paths for proto dependencies.
    pub fn include_paths<P: AsRef<Path>, I: IntoIterator<Item = P>>(mut self, paths: I) -> GenProtos {
        self.include_paths
            .extend(paths.into_iter().map(|p| p.as_ref().to_owned()));
        self
    }

    /// Include `rust/extensions.proto` in the proto compilation.
    ///
    /// Defaults to true
    pub fn include_extensions(mut self, include: bool) -> GenProtos {
        self.include_extensions = include;
        self
    }

    /// If true, before proto compilation, will delete whatever exists at `out_path` and create a
    /// directory at that location.
    pub fn cleanup_out_path(mut self, cleanup: bool) -> GenProtos {
        self.cleanup_out_path = cleanup;
        self
    }

    /// Consumes the builder and generates Rust bindings to your proto files.
    pub fn gen_protos(self) -> Result<(), Box<dyn Error>> {
        // TODO: change expect()s to propagate errors.

        // Clean up root generated directory
        if self.cleanup_out_path && self.gen_path.exists() && self.gen_path.is_dir() {
            fs::remove_dir_all(&self.gen_path).expect("Failed to clean");
        }

        let temp_dir = tempfile::Builder::new()
            .prefix("codegen")
            .tempdir()
            .expect("Failed to create temp dir");

        // Construct protoc command line
        let mut protoc_cmd = Command::new("protoc");

        // Directories that contain protos
        for path in &self.src_paths {
            protoc_cmd.arg("-I");
            protoc_cmd.arg(path);
        }

        // If we want to include our `extensions.proto` file for Rust extentions
        if self.include_extensions {
            fs::create_dir_all(temp_dir.path().join("rust")).expect("failed to create rust/");
            fs::write(temp_dir.path().join("rust").join("extensions.proto"), EXTENSIONS_PROTO)
                .expect("failed to create rust/extensions.proto");
            protoc_cmd.arg("-I");
            protoc_cmd.arg(temp_dir.path());
        }

        // Include any protos from our include paths
        for path in &self.include_paths {
            protoc_cmd.arg("-I");
            protoc_cmd.arg(path);
        }

        // Ideally we'd just invoke protoc with our plugin,
        // but without artifact dependencies in Cargo it's hard to depend on a binary Rust target.
        // Instead we'll invoke the guts of the plugin manually.
        let file_descriptor_set_path = temp_dir.path().join("file_descriptor_set.pb");
        protoc_cmd.arg("-o").arg(&file_descriptor_set_path);
        protoc_cmd.arg("--include_imports");
        protoc_cmd.arg("--include_source_info");

        // Get paths of our Protos
        let proto_paths: Vec<String> = self
            .src_paths
            .iter()
            .flat_map(|path| {
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|file| file.path().extension().unwrap_or_default() == "proto")
                    .map(move |file| {
                        let relative_path = file
                            .path()
                            .strip_prefix(path)
                            .expect("Walked file didn't have root as a prefix");
                        // Convert all paths into Unix-style, relative paths
                        relative_path
                            .to_str()
                            .unwrap_or_else(|| panic!("File path is not UTF-8: {}", file.path().display()))
                            .replace(path::MAIN_SEPARATOR, "/")
                    })
            })
            .collect();

        // Set each proto file as an argument
        protoc_cmd.args(&proto_paths);

        let protoc_status = protoc_cmd.status().expect("something went wrong in running protoc");

        if !protoc_status.success() {
            return Err(format!("protoc exited with status {protoc_status}").into());
        }

        let file_descriptor_set = FileDescriptorSet::deserialize_from_slice(
            &fs::read(file_descriptor_set_path).expect("Failed to read protoc output"),
        )
        .expect("Failed to deserialize FileDescriptorSet");

        let plugin_input = CodeGeneratorRequest {
            file_to_generate: proto_paths,
            proto_file: file_descriptor_set.file,
            ..Default::default()
        };
        let out = codegen::generate_code(&plugin_input);
        if let Some(error) = out.error {
            panic!("Codegen error: {}", error);
        }
        for file in out.file {
            let path = self.gen_path.join(file.get_name());
            fs::create_dir_all(path.parent().expect("generated path should have parent"))
                .expect("Failed to create dir");
            fs::write(path, file.get_content()).expect("Failed to write output");
        }
        Ok(())
    }
}

const EXTENSIONS_PROTO: &str = include_str!("../proto/rust/extensions.proto");

/// Helper function to get the path of the current Cargo.toml
///
/// Get the environment value of `CARGO_MANIFEST_DIR` and converts it into a `PathBuf`
#[doc(hidden)]
fn get_cargo_manifest_path() -> std::io::Result<PathBuf> {
    let path_str = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| std::io::ErrorKind::NotFound)?;
    Ok(PathBuf::from(path_str))
}
