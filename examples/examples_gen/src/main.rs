use pb_jelly_gen::GenProtos;

fn main() -> std::io::Result<()> {
    GenProtos::builder()
        .out_path("../gen/rust/proto")
        .src_path("../protos")
        .include_path("../includes")
        .cleanup_out_path(true)
        .gen_protos()
        .expect("Failed to generate protos");

    Ok(())
}
