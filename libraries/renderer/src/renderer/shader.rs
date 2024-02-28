use thiserror::Error;

use crate::renderer::shader::Error::{ShaderCompilation, UnsupportedFileExtension};
use crate::resources::Resources;

mod gl {
    pub use gl::shader::*;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Shader type could not be guessed from file extension: {0}")]
    UnsupportedFileExtension(String),

    #[error("Resource error: {0}")]
    Resource(#[from] crate::resources::Error),

    #[error("UTF-8 Error: {0}")]
    Utf8Encoding(#[from] core::str::Utf8Error),

    #[error("Shader failed to compile: {0}")]
    ShaderCompilation(String),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Copy, Clone)]
pub enum Kind {
    Vertex,
    Fragment,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Compute,
}

impl Kind {
    #[must_use]
    pub const fn gl_type(self) -> gl::ShaderKind {
        match self {
            Self::Vertex => gl::ShaderKind::VERTEX_SHADER,
            Self::Fragment => gl::ShaderKind::FRAGMENT_SHADER,
            Self::Geometry => gl::ShaderKind::GEOMETRY_SHADER,
            Self::TessellationControl => gl::ShaderKind::TESS_CONTROL_SHADER,
            Self::TessellationEvaluation => gl::ShaderKind::TESS_EVALUATION_SHADER,
            Self::Compute => gl::ShaderKind::COMPUTE_SHADER
        }
    }
}

pub struct Shader {
    id: gl::ShaderId,
    kind: Kind,
}

impl Shader {
    /// # Errors
    /// - Shader compilation error
    pub fn from_res(res: &Resources, name: &str) -> Result<Self> {
        const POSSIBLE_EXT: [(&str, Kind); 2] =
            [(".vert", Kind::Vertex), (".frag", Kind::Fragment)];

        let shader_kind = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, kind)| kind)
            .ok_or_else(|| UnsupportedFileExtension(String::from(name)))?;
        let source = res.load_cstring(name)?;

        Self::from_source(source.to_str()?, shader_kind)
    }

    /// # Errors
    /// - Shader compilation error
    pub fn from_source(source: &str, kind: Kind) -> Result<Self> {
        let gl_type = kind.gl_type();
        let id = shader_from_source(source, gl_type)?;
        Ok(Self { id, kind })
    }

    #[must_use]
    pub const fn id(&self) -> gl::ShaderId { self.id }

    #[must_use]
    pub const fn kind(&self) -> Kind {
        self.kind
    }
}

fn shader_from_source(source: &str, kind: gl::ShaderKind) -> Result<gl::ShaderId> {
    let id = gl::create_shader(kind);

    gl::shader_source(id, source);
    gl::compile_shader(id);

    let compilation_successful = gl::shader_compile_status(id);
    let info_log = gl::shader_info_log(id);
    if compilation_successful {
        if let Some(info_log) = info_log { println!("Shader compiled successfully: {info_log}"); }
        Ok(id)
    } else {
        // I know that this function is not expensive!
        #[allow(clippy::or_fun_call)]
        Err(ShaderCompilation(info_log.unwrap_or(String::from("Unknown error"))))
    }
}

impl Drop for Shader {
    fn drop(&mut self) { gl::delete_shader(&mut self.id) }
}
