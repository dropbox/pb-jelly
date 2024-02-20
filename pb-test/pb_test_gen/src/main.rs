use std::{
    env,
    fs,
};

use pb_jelly_gen::GenProtos;
#[cfg(feature = "bench_prost")]
use prost_build;
#[cfg(feature = "bench_rust_protobuf")]
use protobuf_codegen::Customize;

fn main() -> std::io::Result<()> {
    // Tell Cargo only re-run our build script if something in protos changes
    // println!("cargo:rerun-if-changed=protos");

    // Generate protobuf-rust bindings
    GenProtos::builder()
        .out_path("../gen/pb-jelly")
        .src_path("../proto/packages")
        .gen_protos()
        .expect("Failed to generate protos");

    // compile the protos we use for bench marking, if we want to benchmark against PROST!
    if cfg!(feature = "bench_prost") {
        let mut crate_path = env!("CARGO_MANIFEST_DIR").to_owned();
        crate_path.push_str("/../gen/prost");
        fs::create_dir_all(&crate_path)?;
        env::set_var("OUT_DIR", crate_path);

        #[cfg(feature = "bench_prost")]
        prost_build::compile_protos(
            &["../proto/packages/pbtest/bench.proto"],
            &["../proto/packages", "../../pb-jelly-gen/proto"],
        )
        .unwrap();
    }

    // compile the protos we use for bench marking, if we want to benchmark against rust_protobuf
    if cfg!(feature = "bench_rust_protobuf") {
        let mut crate_path = env!("CARGO_MANIFEST_DIR").to_owned();
        crate_path.push_str("/../src/gen/rust_protobuf");
        fs::create_dir_all(&crate_path)?;

        #[cfg(feature = "bench_rust_protobuf")]
        protobuf_codegen::Codegen::new()
            .out_dir("../src/gen/rust_protobuf")
            .inputs(&[
                "../proto/packages/pbtest/bench.proto",
                "../../pb-jelly-gen/proto/rust/extensions.proto",
            ])
            .include("../../pb-jelly-gen/proto")
            .include("../proto/packages")
            .customize(Customize::default().tokio_bytes(true))
            .run()
            .expect("failed to generate rust_protobuf protos!");
    }

    Ok(())
}
