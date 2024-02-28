extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

const BINDNGS_OUTPUT_FILE: &str = "bindings.rs";

type ApiVersion = (u8, u8);

fn main() {
    let out_dir = env::var("OUT_DIR")
        .expect("Couldn't find build directory from 'OUT_DIR' environment variable!");
    let mut file_gl = File::create(Path::new(&out_dir).join(BINDNGS_OUTPUT_FILE))
        .expect("Failed to create gl bindings file!");
    let (api, version) = get_api_and_version();
    Registry::new(api, version, Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file_gl)
        .expect("Failed to write gl bindings!");
}

fn get_api_and_version() -> (Api, ApiVersion) {
    if cfg!(feature = "GL45") {
        (Api::Gl, (4, 5))
    } else if cfg!(feature = "GL44") {
        (Api::Gl, (4, 4))
    } else if cfg!(feature = "GL43") {
        (Api::Gl, (4, 3))
    } else if cfg!(feature = "GL42") {
        (Api::Gl, (4, 2))
    } else if cfg!(feature = "GL41") {
        (Api::Gl, (4, 1))
    } else if cfg!(feature = "GL40") {
        (Api::Gl, (4, 0))
    } else if cfg!(feature = "GL33") {
        (Api::Gl, (3, 3))
    } else if cfg!(feature = "GLES31") {
        (Api::Gles2, (3, 1))
    } else if cfg!(feature = "GLES30") {
        (Api::Gles2, (3, 0))
    } else if cfg!(feature = "GLES20") {
        (Api::Gles2, (2, 0))
    } else {
        panic!("Could not determine API & Version to generate bindings for!");
    }
}
