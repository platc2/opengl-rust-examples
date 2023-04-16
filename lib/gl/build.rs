extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();

    let extensions = ["GL_ARB_compute_shader", "GL_ARB_shading_language_420pack"];

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, extensions)
        .write_bindings(GlobalGenerator, &mut file_gl)
        .unwrap();
}
