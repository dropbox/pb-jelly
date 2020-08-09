use std::{
    convert::AsRef,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

pub fn gen_protos<P: AsRef<Path>>(src_paths: Vec<P>) -> std::io::Result<()> {
    let builder = GenProtosBuilder::new(src_paths);
    builder.gen_protos()
}

struct GenProtosBuilder {
    gen_path: PathBuf,
    go_path: PathBuf,
    src_paths: Vec<PathBuf>,
}

impl GenProtosBuilder {
    fn new<P: AsRef<Path>>(src_paths: Vec<P>) -> GenProtosBuilder {
        let home_path = env!("HOME");

        let gen_path = PathBuf::from("./gen");
        let go_path = PathBuf::from(home_path).join(Path::new("go"));
        let src_paths = src_paths
            .into_iter()
            .map(|p| p.as_ref().to_owned())
            .collect();

        GenProtosBuilder {
            gen_path,
            go_path,
            src_paths,
        }
    }

    fn gen_protos(self) -> std::io::Result<()> {
        let paths = ArgPaths::generate(&self);
        dbg!(&paths);

        // Clean up root generated directory
        if paths.root_gen_path.exists() && paths.root_gen_path.is_dir() {
            fs::remove_dir_all(&paths.root_gen_path)?;
        }
        fs::create_dir(&paths.root_gen_path)?;

        // Create language specific generate directories
        // fs::create_dir(paths.python_gen_dir())?;
        // fs::create_dir(paths.python_out())?;
        fs::create_dir(paths.rust_gen_dir())?;
        fs::create_dir(paths.rust_out())?;

        // Generate Python protos, codegen uses them
        // self.gen_python_protos()?;
        // Generate Rust protos
        self.gen_rust_protos()?;

        Ok(())
    }

    fn gen_python_protos(&self) -> std::io::Result<()> {
        let paths = ArgPaths::generate(&self);

        let mut python_cmd = Command::new("protoc");

        // Setup environment
        python_cmd.env("PYTHONPATH", paths.python_gen_dir());

        // Directories that contain protos
        let proto_src_dirs = vec![paths.root_proto(), paths.go_proto_src()];
        for path in proto_src_dirs {
            python_cmd.arg("-I");
            python_cmd.arg(path);
        }

        // Set Python out path
        python_cmd.arg(format!(
            "--python_out={}",
            paths.python_out().canonicalize().unwrap().to_str().unwrap()
        ));

        // Paths of proto files
        let proto_files = vec![
            paths.rust_extensions().canonicalize().unwrap(),
            paths.gogo_proto_src().canonicalize().unwrap(),
        ];
        for path in proto_files {
            python_cmd.arg(path);
        }

        // Generate Python protos
        python_cmd.spawn()?.wait()?;

        // Generate __init__.py
        self.generate_py_init()
    }

    fn generate_py_init(&self) -> std::io::Result<()> {
        let paths = ArgPaths::generate(&self);

        for entry in WalkDir::new(paths.python_gen_dir()) {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if metadata.is_dir() {
                fs::File::create(entry.path().join("__init__.py"))?;
            }
        }

        Ok(())
    }

    fn gen_rust_protos(&self) -> std::io::Result<()> {
        let paths = ArgPaths::generate(&self);

        let mut protoc_root_dir = paths.protoc_path.clone();
        protoc_root_dir.pop();
        protoc_root_dir.pop();
        let google_proto_dir = protoc_root_dir.join("include/google/protobuf");

        let entries = fs::read_dir(&google_proto_dir)?;
        
        let proto_paths: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .filter(|file| file.path().extension().unwrap_or_default() == "proto")
            .map(|file| file.path())
            .collect();

        let mut rust_cmd = Command::new("protoc");
        
        // Directories that contain protos
        let proto_src_dirs = vec![paths.root_proto(), paths.go_proto_src()];
        for path in proto_src_dirs {
            rust_cmd.arg("-I");
            rust_cmd.arg(path);
        }

        // Set the rust plugin
        rust_cmd.arg(format!("--plugin=protoc-gen-rust={}", paths.codegen_script().to_str().unwrap()));

        // Ser the Rust out path
        rust_cmd.arg(format!(
            "--rust_out={}",
            paths.rust_out().canonicalize().unwrap().to_str().unwrap()
        ));

        for path in proto_paths {
            rust_cmd.arg(path);
        }

        rust_cmd.spawn()?.wait().map(|_| ())
    }
}

#[derive(Debug)]
struct ArgPaths {
    go_path: PathBuf,
    protoc_path: PathBuf,
    root: PathBuf,
    root_gen_path: PathBuf,
}

impl ArgPaths {
    fn generate(builder: &GenProtosBuilder) -> ArgPaths {
        let abs_cargo = env!("CARGO_MANIFEST_DIR");
        let root = PathBuf::from(abs_cargo);

        let root_gen_path = root.join(&builder.gen_path);
        let go_path = builder.go_path.clone();

        // BLAH HACKY
        let mut protoc_path_raw = String::from_utf8(Command::new("which").arg("protoc").output().unwrap().stdout).unwrap();
        protoc_path_raw.pop();
        let mut real_protoc_path_raw = String::from_utf8(Command::new("realpath").arg(protoc_path_raw).output().unwrap().stdout).unwrap();
        real_protoc_path_raw.pop();

        let protoc_path = PathBuf::from(real_protoc_path_raw);

        ArgPaths {
            go_path,
            protoc_path,
            root,
            root_gen_path,
        }
    }

    fn python_gen_dir(&self) -> PathBuf {
        self.root_gen_path.join("python")
    }

    fn rust_gen_dir(&self) -> PathBuf {
        self.root_gen_path.join("rust")
    }

    fn go_proto_src(&self) -> PathBuf {
        self.go_path.join("src")
    }

    fn gogo_proto_src(&self) -> PathBuf {
        self.go_proto_src()
            .join("github.com/gogo/protobuf/gogoproto/gogo.proto")
    }

    fn rust_extensions(&self) -> PathBuf {
        self.root.join("proto/rust/extensions.proto")
    }

    fn python_out(&self) -> PathBuf {
        self.python_gen_dir().join("proto")
    }

    fn rust_out(&self) -> PathBuf {
        self.rust_gen_dir().join("proto")
    }

    fn root_proto(&self) -> PathBuf {
        self.root.join("proto")
    }

    fn codegen_script(&self) -> PathBuf {
        self.root.join("codegen/codegen.py")
    }
}
