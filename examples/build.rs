use pb_gen::GenProtos;

fn main() -> std::io::Result<()> {
    // Tell Cargo only re-run our build script if something in protos changes
    // println!("cargo:rerun-if-changed=protos");

    GenProtos::builder()
        .out_path("./gen/rust/proto")
        .src_path("./protos")
        .cleanup_out_path(true)
        .gen_protos();

    Ok(())
}
