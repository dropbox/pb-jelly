//! `pb_gen` generates Rust bindings for `proto2` and `proto3` files. It's intended to be used with [`pb_rs`](https://github.com/dropbox/pb-rs).
//! 
//! ## Examples
//! Complete examples can be found in the [`examples`](https://github.com/dropbox/pb-rs/tree/master/examples) crate, 
//! or the [`pb-test`](https://github.com/dropbox/pb-rs/tree/master/pb-test) crate of the [`protobuf_rs`](https://github.com/dropbox/pb-rs) workspace.
//! 
//! ## In a nutshell 🥜
//! You can include `pb_gen` in your Cargo project, by including it as a `[build-dependency]` in your `Cargo.toml`
//! ```toml
//! [build-dependencies]
//! pb-gen = "0.1"
//! ```
//! 
//! Then from a [`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html) script, use either the `GenProtos` builder struct, 
//! or the `gen_protos` convience function to specify where your protos live, and where the generated code should be put.
//! ```no_run
//! use pb_jelly_gen::GenProtos;
//! 
//! fn main() -> std::io::Result<()> {
//!    GenProtos::builder()
//!        // output path for our generated code
//!        .out_path("./gen")
//!        // directory where our protos live
//!        .src_path("./protos")
//!        // delete and recreate the `out_path` directory every time
//!        .cleanup_out_path(true)
//!        .gen_protos();
//!
//!    Ok(())
//! }
//! ```

use include_dir::{include_dir, Dir};
use std::{
    convert::AsRef,
    fs,
    io::Write,
    iter::IntoIterator,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Output},
    str::from_utf8,
};
use tempdir::TempDir;
use walkdir::WalkDir;

// We statically include the `/codegen` directory as a way to include our codegen.py and
// extensions.proto files in the Rust library. At execution time, this directory gets
// recreated a temp location, so `protoc` can access the files.
const CODEGEN: Dir = include_dir!("codegen");

/// A "no frills" way to generate Rust bindings for your proto files. `src_paths` is a list of
/// paths to your `.proto` files, or the directories that contain them. Generated code it outputted
/// to `<current crate's manifest>/gen`.
pub fn gen_protos<P: AsRef<Path>>(src_paths: Vec<P>) {
    GenProtos::builder().src_paths(src_paths).gen_protos()
}

/// A builder struct to configure the way your protos are generated, create one with `GenProtos::builder()`
pub struct GenProtos {
    gen_path: PathBuf,
    src_paths: Vec<PathBuf>,
    include_paths: Vec<PathBuf>,
    include_extensions: bool,
    cleanup_out_path: bool,
}

impl std::default::Default for GenProtos {
    fn default() -> Self {
        let gen_path = get_cargo_manifest_path()
            .expect("couldn't get `CARGO_MANIFEST_DIR` when building default GenProtos");
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
        self.src_paths
            .extend(paths.into_iter().map(|p| p.as_ref().to_owned()));
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
    pub fn include_paths<P: AsRef<Path>, I: IntoIterator<Item = P>>(
        mut self,
        paths: I,
    ) -> GenProtos {
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
    pub fn gen_protos(self) {
        let output = self.gen_protos_helper();
        
        if !output.status.success() {
            dbg!(output.status.code());
            eprintln!("stdout={}", from_utf8(&output.stdout).unwrap_or("cant decode stdout"));
            eprintln!("stderr={}", from_utf8(&output.stderr).unwrap_or("cant decode stderr"));
            panic!("Failed to generate Rust bindings to proto files!")
        }

        dbg!("Protos Generated Successfully");
    }
}

// Private functions
impl GenProtos {
    fn gen_protos_helper(self) -> Output {
        // Clean up root generated directory
        if self.cleanup_out_path && self.gen_path.exists() && self.gen_path.is_dir() {
            dbg!("Cleaning up existing gen path", &self.gen_path);
            fs::remove_dir_all(&self.gen_path).expect("Failed to clean");
        }

        // Re-create essential files
        if !self.gen_path.exists() {
            dbg!("Creating gen path", &self.gen_path);
            fs::create_dir_all(&self.gen_path).expect("Failed to create dir");
        }
        let temp_dir = self.create_temp_files().expect("Failed to package codegen script");
        // Generate Rust protos
        self.gen_rust_protos(temp_dir)
    }

    fn gen_rust_protos(&self, temp_dir: TempDir) -> Output {
        // Temp path to the codegen script
        let codegen_path = temp_dir.path().join("codegen.py");

        let mut protoc_cmd = Command::new("protoc");

        // Directories that contain protos
        dbg!("Include paths");
        for path in self.src_paths.iter() {
            protoc_cmd.arg("-I");
            protoc_cmd.arg(path);
            dbg!(path);
        }

        // If we want to include our `extensions.proto` file for Rust extentions
        if self.include_extensions {
            let ext_path = temp_dir.path();
            protoc_cmd.arg("-I");
            protoc_cmd.arg(ext_path);
            dbg!(ext_path);
        }

        // Include any protos from our include paths
        for path in self.include_paths.iter() {
            protoc_cmd.arg("-I");
            protoc_cmd.arg(path);
            dbg!(path);
        }

        // Set the rust plugin
        protoc_cmd.arg(format!(
            "--plugin=protoc-gen-rust={}",
            codegen_path.to_str().unwrap()
        ));

        // Set the Rust out path
        protoc_cmd.arg(format!(
            "--rust_out={}",
            self.gen_path.to_str().unwrap()
        ));

        // Get paths of our Protos
        let proto_paths = self
            .src_paths
            .iter()
            .map(|path| {
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|file| file.path().extension().unwrap_or_default() == "proto")
                    .map(|file| file.into_path())
            })
            .flatten();

        // Set each proto file as an argument
        dbg!("Proto paths");
        for path in proto_paths {
            dbg!(&path);
            protoc_cmd.arg(path);
        }

        dbg!(&protoc_cmd);
        protoc_cmd.output()
            .expect("something went wrong in running protoc to generate Rust bindings 🤮")
    }

    /// We bundle all non-Rust, but necessary files into a static CODEGEN blob. When we run the codegen script,
    /// we recreate these in a temp directory `/tmp/codegen` that is cleaned up after.
    fn create_temp_files(&self) -> std::io::Result<TempDir> {
        let temp_dir = TempDir::new("codegen")?;

        fn create_temp_files_helper(dir: &Dir, temp_dir: &TempDir) -> std::io::Result<()> {
            for file in dir.files() {
                let blob_path = file.path();
                let abs_path = temp_dir.path().join(blob_path);

                let mut abs_file = fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&abs_path)?;
                abs_file.write_all(file.contents())?;

                let mut permissions = abs_file.metadata()?.permissions();
                permissions.set_mode(0o777);
                drop(abs_file);

                // Set permissions of the file so it is executable
                fs::set_permissions(&abs_path, permissions)?;
            }

            for dir in dir.dirs() {
                let blob_path = dir.path();
                let abs_path = temp_dir.path().join(blob_path);
                fs::create_dir(&abs_path)?;

                create_temp_files_helper(dir, temp_dir)?;
            }

            Ok(())
        }
        create_temp_files_helper(&CODEGEN, &temp_dir)?;

        Ok(temp_dir)
    }
}

/// Helper function to get the path of the current Cargo.toml
///
/// Get the environment value of `CARGO_MANIFEST_DIR` and converts it into a `PathBuf`
#[doc(hidden)]
fn get_cargo_manifest_path() -> std::io::Result<PathBuf> {
    let path_str = std::env::var("CARGO_MANIFEST_DIR").map_err(|_| std::io::ErrorKind::NotFound)?;
    Ok(PathBuf::from(path_str))
}
