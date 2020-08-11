use pb_gen::gen_protos;

fn main() -> std::io::Result<()> {
    // Tell Cargo only re-run our build script if something in protos changes
    // println!("cargo:rerun-if-changed=protos");

    gen_protos(vec!["./protos"])
}
