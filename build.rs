use std::io::Write;

use aski_rs::codegen::CodegenConfig;
use aski_rs::compiler::compile_directory;

fn main() {
    println!("cargo:rerun-if-changed=source/chart.aski");
    println!("cargo:rerun-if-changed=source/ephemeris.aski");
    println!("cargo:rerun-if-changed=source/render.aski");
    println!("cargo:rerun-if-changed=source/main.aski");

    let config = CodegenConfig { rkyv: false };
    let rust_code = compile_directory(
        &["source/chart.aski", "source/ephemeris.aski", "source/render.aski", "source/main.aski"],
        &config,
    )
    .expect("failed to compile aski files");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = format!("{out_dir}/chart_generated.rs");
    let mut f = std::fs::File::create(&out_path).expect("failed to create output file");
    f.write_all(rust_code.as_bytes()).expect("failed to write");

    eprintln!("Generated {out_path}");
}
