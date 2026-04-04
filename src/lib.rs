// astro-aski library — FFI adapters + generated aski code.
#[path = "../rust-ffi/ffi.rs"]
pub mod ffi;
#[path = "../rust-ffi/render.rs"]
pub mod render;

#[allow(dead_code, unused_variables, unreachable_patterns, unused_imports)]
pub mod chart {
    use crate::ffi;
    use crate::render;
    include!(concat!(env!("OUT_DIR"), "/chart_generated.rs"));
}
