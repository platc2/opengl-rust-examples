use gl::types::{GLenum, GLint, GLintptr, GLsizeiptr, GLuint};

use crate::Buffer;
use crate::renderer::{Program, Shader, Texture, VertexAttribute};
use crate::renderer::vertex_attribute::Format;

pub struct RenderPass {
    vertex_array_object: GLuint,
    program: Program,
    uniform_buffers: Vec<(GLuint, GLsizeiptr)>,
    textures: Vec<GLuint>,
}

pub struct VertexBinding {
    binding_index: GLuint,
    vertex_attribute: VertexAttribute,
}

impl VertexBinding {
    pub fn new(binding_index: u8, vertex_attribute: VertexAttribute) -> Self {
        Self { binding_index: GLuint::from(binding_index), vertex_attribute }
    }
}

impl RenderPass {
    /// # Errors
    /// - Invalid shaders
    ///   - Compile errors
    ///   - Link errors
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader, vertex_bindings: &[VertexBinding], uniform_buffers: &[&Buffer], textures: &[Texture]) -> Result<Self, String> {
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
            vertex_shader,
            fragment_shader
        ])?;

        let uniform_buffers = uniform_buffers.iter()
            .map(|buffer| (buffer.handle(), GLsizeiptr::try_from(buffer.size()).expect("Uniform buffer too large!")))
            .collect();

        let textures = textures.iter()
            .map(Texture::handle)
            .collect();

        Ok(Self { vertex_array_object, program, uniform_buffers, textures })
    }

    pub fn display(&self) {
        self.program.set_used();

        unsafe {
            gl::BindVertexArray(self.vertex_array_object);
            for (index, (handle, size)) in self.uniform_buffers.iter().enumerate() {
                gl::BindBufferRange(gl::UNIFORM_BUFFER,
                                    GLuint::try_from(index)
                                        .expect("Too many uniform buffers!"),
                                    *handle,
                                    0 as GLintptr,
                                    *size);
            }

            for (texture_slot, texture_handle) in self.textures.iter()
                .enumerate()
                .map(|(index, handle)| (index_to_texture_slot(index), handle)) {
                gl::ActiveTexture(texture_slot);
                gl::BindTexture(gl::TEXTURE_2D, *texture_handle);
            }
        }
    }
}

fn index_to_texture_slot(index: usize) -> GLenum {
    GLenum::try_from(gl::TEXTURE0 as usize + index)
        .expect("Texture index too large")
}

const fn convert_format(format: Format) -> (GLint, GLenum) {
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
