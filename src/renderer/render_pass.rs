use gl::types::{GLenum, GLint, GLuint};

use crate::renderer::{Program, Shader, VertexAttribute};
use crate::renderer::shader::Kind;
use crate::renderer::vertex_attribute::Format;

pub struct RenderPass {
    vertex_array_object: GLuint,
    program: Program,
}

pub struct VertexBinding {
    binding_index: GLuint,
    vertex_attribute: VertexAttribute,
}

impl VertexBinding {
    pub fn new(binding_index: u8, vertex_attribute: VertexAttribute) -> Self {
        VertexBinding { binding_index: GLuint::from(binding_index), vertex_attribute }
    }
}

impl RenderPass {
    /// # Errors
    /// - Invalid shaders
    ///   - Compile errors
    ///   - Link errors
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str, vertex_bindings: &[VertexBinding]) -> Result<Self, String> {
        let mut vertex_array_object: GLuint = 0;
        unsafe { gl::CreateVertexArrays(1, &mut vertex_array_object); }

        for (index, VertexBinding { binding_index, vertex_attribute }) in vertex_bindings.iter().enumerate() {
            let index = GLuint::try_from(index)
                .expect("Too many vertex bindings!");
            let (format_size, format_type) = convert_format(vertex_attribute.format());
            unsafe {
                gl::EnableVertexArrayAttrib(vertex_array_object, index);
                gl::VertexArrayAttribFormat(vertex_array_object, index, format_size, format_type,
                                            gl::FALSE, GLuint::from(vertex_attribute.offset()));
                gl::VertexArrayAttribBinding(vertex_array_object, index, *binding_index);
            }
        }

        let program = Program::from_shaders(&[
            Shader::from_source(vertex_shader_source, Kind::Vertex)?,
            Shader::from_source(fragment_shader_source, Kind::Fragment)?
        ])?;

        Ok(Self { vertex_array_object, program })
    }

    pub fn display(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);
            self.program.set_used();
        }
    }
}

fn convert_format(format: Format) -> (GLint, GLenum) {
    match format {
        Format::R32F => (1, gl::FLOAT),
        Format::RG32F => (2, gl::FLOAT),
        Format::RGB32F => (3, gl::FLOAT),
        Format::RGBA32F => (4, gl::FLOAT),
        Format::R8 => (1, gl::BYTE),
        Format::RG8 => (2, gl::BYTE),
        Format::RGB8 => (3, gl::BYTE),
        Format::RGBA8 => (4, gl::BYTE),
    }
}
