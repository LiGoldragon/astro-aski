use std::io::Write;

use aski::codegen::{CodegenConfig, generate_rust_from_db_with_config};
use aski::db::{create_db, insert_ast};
use aski::parser::parse_source;

fn main() {
    println!("cargo:rerun-if-changed=aski/chart.aski");

    let manifest = env!("CARGO_MANIFEST_DIR");
    let source = std::fs::read_to_string(format!("{manifest}/aski/chart.aski"))
        .expect("failed to read chart.aski");

    // Parse
    let items = match parse_source(&source) {
        Ok(items) => items,
        Err(e) => panic!("failed to parse chart.aski:\n{e}"),
    };
    eprintln!("Parsed {} items from chart.aski", items.len());

    // Insert into CozoDB
    let db = create_db().expect("failed to create db");
    insert_ast(&db, &items).expect("failed to insert AST");

    // Generate Rust (no rkyv, no main — just types and methods)
    let config = CodegenConfig { rkyv: false };
    let rust_code = generate_rust_from_db_with_config(&db, &config)
        .expect("failed to generate Rust");

    // Write to OUT_DIR
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = format!("{out_dir}/chart_generated.rs");
    let mut f = std::fs::File::create(&out_path).expect("failed to create output file");
    f.write_all(rust_code.as_bytes()).expect("failed to write");

    eprintln!("Generated {out_path}");
}
