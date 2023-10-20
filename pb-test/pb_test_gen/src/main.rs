use pb_jelly_gen::GenProtos;
use std::{
    env,
    fs,
};

#[cfg(feature = "bench_prost")]
use prost_build;

#[cfg(feature = "bench_rust_protobuf")]
use protoc_rust::Customize;

fn main() -> std::io::Result<()> {
    // Tell Cargo only re-run our build script if something in protos changes
    // println!("cargo:rerun-if-changed=protos");

    // Generate protobuf-rust bindings
    GenProtos::builder()
        .out_path("../gen/pb-jelly")
        .src_path("../proto/packages")
        .src_path("../proto/includes")
        .gen_protos();

    // compile the protos we use for bench marking, if we want to benchmark against PROST!
    if cfg!(feature = "bench_prost") {
        let mut crate_path = env!("CARGO_MANIFEST_DIR").to_owned();
        crate_path.push_str("/../gen/prost");
        fs::create_dir_all(&crate_path)?;
        env::set_var("OUT_DIR", crate_path);

        #[cfg(feature = "bench_prost")]
        prost_build::compile_protos(
            &["../proto/packages/pbtest/bench.proto"],
            &["../proto/packages", "../proto/includes"],
        )
        .unwrap();
    }

    // compile the protos we use for bench marking, if we want to benchmark against rust_protobuf
    if cfg!(feature = "bench_rust_protobuf") {
        let mut crate_path = env!("CARGO_MANIFEST_DIR").to_owned();
        crate_path.push_str("/../gen/rust_protobuf");
        fs::create_dir_all(&crate_path)?;

        #[cfg(feature = "bench_rust_protobuf")]
        protoc_rust::Codegen::new()
            .out_dir("../gen/rust_protobuf")
            .inputs(&["../proto/packages/pbtest/bench.proto"])
            .include("../proto/includes")
            .include("../proto/packages")
            .customize(Customize {
                carllerche_bytes_for_bytes: Some(true),
                ..Default::default()
            })
            .run()
            .expect("failed to generate rust_protobuf protos!");
    }

    Ok(())
}
