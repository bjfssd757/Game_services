fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::var("OUT_DIR")?;

    // tonic_build::compile_protos(
    //     "../proto/session.proto"
    // )?;
    tonic_build::configure()
        .out_dir("./generated")
        .compile_protos(
            &["../proto/session.proto"],
            &["../proto"],
        )?;

    Ok(())
}