use std::{
    convert::AsRef,
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};
use include_dir::{include_dir, Dir};
use tempdir::TempDir;
use walkdir::WalkDir;

const CODEGEN: Dir = include_dir!("codegen");


pub fn gen_protos<P: AsRef<Path>>(src_paths: Vec<P>) -> std::io::Result<()> {
    let builder = GenProtosBuilder::new(src_paths);
    builder.gen_protos()
}

struct GenProtosBuilder {
    gen_path: PathBuf,
    src_paths: Vec<PathBuf>,
    include_extensions: bool,
}

impl std::default::Default for GenProtosBuilder {
    fn default() -> Self {
        let gen_path = PathBuf::from("./gen"); 
        let src_paths = vec![];
        let include_extensions = true;

        GenProtosBuilder {
            gen_path,
            src_paths,
            include_extensions,
        }
    }
}

impl GenProtosBuilder {
    fn new<P: AsRef<Path>>(src_paths: Vec<P>) -> GenProtosBuilder {
        let mut builder = GenProtosBuilder::default();
        builder.src_paths = src_paths
            .into_iter()
            .map(|p| p.as_ref().to_owned())
            .collect();

        builder
    }

    fn gen_protos(self) -> std::io::Result<()> {
        let paths = ArgPaths::generate(&self);
        dbg!(&paths);

        // Clean up root generated directory
        if paths.root_gen_path.exists() && paths.root_gen_path.is_dir() {
            fs::remove_dir_all(&paths.root_gen_path)?;
        }
        fs::create_dir(&paths.root_gen_path)?;

        dbg!(paths.rust_gen_dir());
        dbg!(paths.rust_out());

        // Create language specific generate directories
        fs::create_dir(paths.rust_gen_dir())?;
        fs::create_dir(paths.rust_out())?;

        // Re-create essential files
        let temp_dir = self.create_temp_files()?;
        // Generate Rust protos
        self.gen_rust_protos(temp_dir)?;

        Ok(())
    }

    fn gen_rust_protos(&self, temp_dir: TempDir) -> std::io::Result<()> {
        let paths = ArgPaths::generate(&self);
        let codegen_path = temp_dir.path().join("codegen.py");

        let mut rust_cmd = Command::new("protoc");
        
        // Directories that contain protos
        for path in self.src_paths.iter() {
            rust_cmd.arg("-I");
            rust_cmd.arg(path);
        }

        if self.include_extensions {
            let ext_path = temp_dir.path();
            rust_cmd.arg("-I");
            rust_cmd.arg(ext_path);
        }

        // Set the rust plugin
        rust_cmd.arg(format!("--plugin=protoc-gen-rust={}", codegen_path.to_str().unwrap()));

        // Set the Rust out path
        rust_cmd.arg(format!(
            "--rust_out={}",
            paths.rust_out().canonicalize().unwrap().to_str().unwrap()
        ));

        // Get paths of our Protos
        let proto_paths = self.src_paths
            .iter()
            .map(|path| {
                WalkDir::new(path)
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|file| file.path().extension().unwrap_or_default() == "proto")
                    .map(|file| file.into_path())
            })
            .flatten();

        // Set our protos
        for path in proto_paths {
            dbg!(&path);
            rust_cmd.arg(path);
        }

        let output = rust_cmd.output()?;
        dbg!(output);

        Ok(())
    }

    /// We bundle all non-Rust, but necessary files into a static CODEGEN blob. When we run the codegen script,
    /// we recreate these in a temp directory `/tmp/codegen` that is cleaned up after.
    fn create_temp_files(&self) -> std::io::Result<TempDir> {
        let temp_dir = TempDir::new("codegen")?;

        fn create_temp_files_helper(dir: &Dir, temp_dir: &TempDir) -> std::io::Result<()> {
            for file in dir.files() {
                let blob_path = file.path();
                let abs_path = temp_dir.path().join(blob_path);
                dbg!(&abs_path);

                let mut abs_file = fs::OpenOptions::new().write(true).create_new(true).open(&abs_path)?;
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
                dbg!(&abs_path);
                fs::create_dir(&abs_path)?;

                create_temp_files_helper(dir, temp_dir)?;
            }

            Ok(())
        }
        create_temp_files_helper(&CODEGEN, &temp_dir)?;

        Ok(temp_dir)
    }
}

#[derive(Debug)]
struct ArgPaths {
    root: PathBuf,
    root_gen_path: PathBuf,
}

impl ArgPaths {
    fn generate(builder: &GenProtosBuilder) -> ArgPaths {
        // TODO: Figuring out these paths should be better
        let abs_target = std::env::current_dir().unwrap();
        let root = PathBuf::from(abs_target);
        let root_gen_path = root.join(&builder.gen_path);

        ArgPaths {
            root,
            root_gen_path,
        }
    }

    fn rust_gen_dir(&self) -> PathBuf {
        self.root_gen_path.join("rust")
    }

    fn rust_out(&self) -> PathBuf {
        self.rust_gen_dir().join("proto")
    }
}
