use std::io::Write;

use aski_rs::codegen::CodegenConfig;
use aski_rs::compiler::compile_directory;

fn main() {
    println!("cargo:rerun-if-changed=aski/chart.aski");
    println!("cargo:rerun-if-changed=aski/ephemeris.aski");

    let config = CodegenConfig { rkyv: false };
    let rust_code = compile_directory(
        &["aski/chart.aski", "aski/ephemeris.aski"],
        &config,
    )
    .expect("failed to compile aski files");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = format!("{out_dir}/chart_generated.rs");
    let mut f = std::fs::File::create(&out_path).expect("failed to create output file");
    f.write_all(rust_code.as_bytes()).expect("failed to write");

    eprintln!("Generated {out_path}");
}
