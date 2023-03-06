use gl::types::{GLenum, GLint, GLintptr, GLsizeiptr, GLuint};
use thiserror::Error;

use crate::renderer::render_pass::Error::IncompleteFramebuffer;
use crate::renderer::vertex_attribute::Format;
use crate::renderer::{Buffer, Program, Shader, Texture, VertexAttribute};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Too many vertex bindings!")]
    TooManyVertexBindings,

    #[error("Program failed to link: {0}")]
    ProgramLink(#[from] crate::renderer::program::Error),

    #[error("Framebuffer incomplete")]
    IncompleteFramebuffer,
}
type Result<T> = std::result::Result<T, Error>;

pub struct RenderPass {
    vertex_array_object: GLuint,
    program: Program,
    uniform_buffers: Vec<(GLuint, GLsizeiptr)>,
    textures: Vec<GLuint>,
    frame_buffer: GLuint,
}

pub struct VertexBinding {
    binding_index: GLuint,
    vertex_attribute: VertexAttribute,
}

impl VertexBinding {
    #[must_use]
    pub fn new(binding_index: u8, vertex_attribute: VertexAttribute) -> Self {
        Self {
            binding_index: GLuint::from(binding_index),
            vertex_attribute,
        }
    }
}

impl RenderPass {
    /// # Errors
    /// - Invalid shaders
    ///   - Compile errors
    ///   - Link errors
    /// - Too many vertex bindings
    pub fn new(
        vertex_shader: &Shader,
        fragment_shader: &Shader,
        vertex_bindings: &[VertexBinding],
        uniform_buffers: &[&Buffer],
        textures: &[&Texture],
        attachments: &[&Texture],
    ) -> Result<Self> {
        let mut vertex_array_object: GLuint = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut vertex_array_object);
        }

        for (
            index,
            VertexBinding {
                binding_index,
                vertex_attribute,
            },
        ) in vertex_bindings.iter().enumerate()
        {
            let index = GLuint::try_from(index).map_err(|_| Error::TooManyVertexBindings)?;
            let (format_size, format_type) = convert_format(vertex_attribute.format());
            unsafe {
                gl::EnableVertexArrayAttrib(vertex_array_object, index);
                gl::VertexArrayAttribFormat(
                    vertex_array_object,
                    index,
                    format_size,
                    format_type,
                    gl::FALSE,
                    GLuint::from(vertex_attribute.offset()),
                );
                gl::VertexArrayAttribBinding(vertex_array_object, index, *binding_index);
            }
        }

        let program = Program::from_shaders(&[vertex_shader, fragment_shader])?;

        // Buffer object already checks for valid size
        #[allow(clippy::cast_possible_wrap)]
        let uniform_buffers = uniform_buffers
            .iter()
            .map(|&buffer| (buffer.handle(), buffer.size() as GLsizeiptr))
            .collect();

        let textures = textures.iter().map(|texture| texture.handle()).collect();

        let mut frame_buffer: GLuint = 0;
        if !attachments.is_empty() {
            unsafe {
                gl::CreateFramebuffers(1, &mut frame_buffer);
                gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, frame_buffer);

                for (index, attachment) in attachments.iter().enumerate() {
                    gl::BindTexture(gl::TEXTURE_2D, attachment.handle());
                    gl::FramebufferTexture2D(
                        gl::DRAW_FRAMEBUFFER,
                        index_to_color_attachment_slot(index),
                        gl::TEXTURE_2D,
                        attachment.handle(),
                        0,
                    );
                }

                /*
                                                let mut render_buffer: GLuint = 0;
                                                gl::CreateRenderbuffers(1, &mut render_buffer);
                                                gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer);
                                                gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, 1024, 1024);
                                                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, gl::RENDERBUFFER, render_buffer);
                */
            }

            match unsafe { gl::CheckFramebufferStatus(gl::DRAW_FRAMEBUFFER) } {
                gl::FRAMEBUFFER_COMPLETE => (),
                _ => return Err(IncompleteFramebuffer),
            }
        }

        Ok(Self {
            vertex_array_object,
            program,
            uniform_buffers,
            textures,
            frame_buffer,
        })
    }

    pub fn display(&self) {
        unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.frame_buffer) };

        self.program.set_used();
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);
        }

        for (index, (handle, size)) in
            self.uniform_buffers
                .iter()
                .enumerate()
                .map(|(index, tuple)| {
                    (
                        GLuint::try_from(index).expect("Too many uniform buffers"),
                        tuple,
                    )
                })
        {
            unsafe {
                gl::BindBufferRange(gl::UNIFORM_BUFFER, index, *handle, 0 as GLintptr, *size);
            }
        }

        for (texture_slot, texture_handle) in self
            .textures
            .iter()
            .enumerate()
            .map(|(index, handle)| (index_to_texture_slot(index), handle))
        {
            unsafe {
                gl::ActiveTexture(texture_slot);
                gl::BindTexture(gl::TEXTURE_2D, *texture_handle);
            }
        }
    }
}

fn index_to_texture_slot(index: usize) -> GLenum {
    GLenum::try_from(gl::TEXTURE0 as usize + index).expect("Texture index too large")
}

fn index_to_color_attachment_slot(index: usize) -> GLenum {
    GLenum::try_from(gl::COLOR_ATTACHMENT0 as usize + index).expect("Attachment index too large")
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
