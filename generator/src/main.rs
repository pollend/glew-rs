extern crate core;

mod command_parser;
mod const_parser;
mod context;
mod generator;
mod gl_command_generator;
mod gl_generator;

use generator::write_source_code;
use std::path::Path;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    if cwd.ends_with("generator") {
        write_source_code(Path::new("OpenGL-Registry"), "../rogl/src");
    } else {
        write_source_code(Path::new("generator/OpenGL-Registry"), "./rogl/src");
    }
}
