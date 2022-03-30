extern crate core;

mod command_parser;
mod const_parser;
mod generator;
mod context;
mod gl_generator;

use generator::write_source_code;
use std::path::Path;

use std::os::raw;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    if cwd.ends_with("generator") {
        write_source_code(Path::new("OpenGL-Registry"), "../rogl/src");
    } else {
        write_source_code(Path::new("generator/OpenGL-Registry"), "./rogl/src");
    }
}
