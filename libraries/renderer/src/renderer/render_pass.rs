use thiserror::Error;

use gl::sys::types::{GLenum, GLint, GLintptr, GLsizeiptr, GLuint};

use crate::renderer::{Buffer, Program, Shader, Texture, VertexAttribute};
use crate::renderer::render_pass::Error::IncompleteFramebuffer;
use crate::renderer::vertex_attribute::Format;

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
    ///
    /// # Panics
    /// - Framebuffer not setup
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
            gl::sys::CreateVertexArrays(1, &mut vertex_array_object);
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
                gl::sys::EnableVertexArrayAttrib(vertex_array_object, index);
                gl::sys::VertexArrayAttribFormat(
                    vertex_array_object,
                    index,
                    format_size,
                    format_type,
                    gl::sys::FALSE,
                    GLuint::from(vertex_attribute.offset()),
                );
                gl::sys::VertexArrayAttribBinding(vertex_array_object, index, *binding_index);
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
                gl::sys::CreateFramebuffers(1, &mut frame_buffer);
                gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, frame_buffer);

                for (index, attachment) in attachments.iter().enumerate() {
                    gl::sys::BindTexture(gl::sys::TEXTURE_2D, attachment.handle());
                    gl::sys::FramebufferTexture2D(
                        gl::sys::DRAW_FRAMEBUFFER,
                        index_to_color_attachment_slot(index),
                        gl::sys::TEXTURE_2D,
                        attachment.handle(),
                        0,
                    );
                }

                let mut render_buffer: GLuint = 0;
                gl::sys::CreateRenderbuffers(1, &mut render_buffer);
                gl::sys::BindRenderbuffer(gl::sys::RENDERBUFFER, render_buffer);
                gl::sys::RenderbufferStorage(gl::sys::RENDERBUFFER, gl::sys::DEPTH24_STENCIL8, 1024, 1024);
                gl::sys::FramebufferRenderbuffer(
                    gl::sys::FRAMEBUFFER,
                    gl::sys::DEPTH_STENCIL_ATTACHMENT,
                    gl::sys::RENDERBUFFER,
                    render_buffer,
                );
            }

            match unsafe { gl::sys::CheckFramebufferStatus(gl::sys::DRAW_FRAMEBUFFER) } {
                gl::sys::FRAMEBUFFER_COMPLETE => (),
                e => panic!("{:x} INCOMPLETE!", e),
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

    pub fn new_geom(
        vertex_shader: &Shader,
        fragment_shader: &Shader,
        geometry_shader: &Shader,
        vertex_bindings: &[VertexBinding],
        uniform_buffers: &[&Buffer],
        textures: &[&Texture],
        attachments: &[&Texture],
    ) -> Result<Self> {
        let mut vertex_array_object: GLuint = 0;
        unsafe {
            gl::sys::CreateVertexArrays(1, &mut vertex_array_object);
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
                gl::sys::EnableVertexArrayAttrib(vertex_array_object, index);
                gl::sys::VertexArrayAttribFormat(
                    vertex_array_object,
                    index,
                    format_size,
                    format_type,
                    gl::sys::FALSE,
                    GLuint::from(vertex_attribute.offset()),
                );
                gl::sys::VertexArrayAttribBinding(vertex_array_object, index, *binding_index);
            }
        }

        let program = Program::from_shaders(&[vertex_shader, fragment_shader, geometry_shader])?;

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
                gl::sys::CreateFramebuffers(1, &mut frame_buffer);
                gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, frame_buffer);

                for (index, attachment) in attachments.iter().enumerate() {
                    gl::sys::BindTexture(gl::sys::TEXTURE_2D, attachment.handle());
                    gl::sys::FramebufferTexture2D(
                        gl::sys::DRAW_FRAMEBUFFER,
                        index_to_color_attachment_slot(index),
                        gl::sys::TEXTURE_2D,
                        attachment.handle(),
                        0,
                    );
                }

                let mut render_buffer: GLuint = 0;
                gl::sys::CreateRenderbuffers(1, &mut render_buffer);
                gl::sys::BindRenderbuffer(gl::sys::RENDERBUFFER, render_buffer);
                gl::sys::RenderbufferStorage(gl::sys::RENDERBUFFER, gl::sys::DEPTH24_STENCIL8, 1024, 1024);
                gl::sys::FramebufferRenderbuffer(
                    gl::sys::FRAMEBUFFER,
                    gl::sys::DEPTH_STENCIL_ATTACHMENT,
                    gl::sys::RENDERBUFFER,
                    render_buffer,
                );
            }

            match unsafe { gl::sys::CheckFramebufferStatus(gl::sys::DRAW_FRAMEBUFFER) } {
                gl::sys::FRAMEBUFFER_COMPLETE => (),
                e => panic!("{:x} INCOMPLETE!", e),
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
    /// # Errors
    /// - Invalid shaders
    ///   - Compile errors
    ///   - Link errors
    ///
    /// # Panics
    /// - Framebuffer not setup
    // TODO platc2 09.03.2023 - Temporary fix
    #[allow(clippy::too_many_arguments)]
    pub fn new_tess(
        vertex_shader: &Shader,
        fragment_shader: &Shader,
        tessellation_control_shader: &Shader,
        tessellation_evaluation_shader: &Shader,
        vertex_bindings: &[VertexBinding],
        uniform_buffers: &[&Buffer],
        textures: &[&Texture],
        attachments: &[&Texture],
    ) -> Result<Self> {
        let mut vertex_array_object: GLuint = 0;
        unsafe {
            gl::sys::CreateVertexArrays(1, &mut vertex_array_object);
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
                gl::sys::EnableVertexArrayAttrib(vertex_array_object, index);
                gl::sys::VertexArrayAttribFormat(
                    vertex_array_object,
                    index,
                    format_size,
                    format_type,
                    gl::sys::FALSE,
                    GLuint::from(vertex_attribute.offset()),
                );
                gl::sys::VertexArrayAttribBinding(vertex_array_object, index, *binding_index);
            }
        }

        let program = Program::from_shaders(&[
            vertex_shader,
            fragment_shader,
            tessellation_control_shader,
            tessellation_evaluation_shader,
        ])?;

        let uniform_buffers = uniform_buffers
            .iter()
            .map(|buffer| {
                (
                    buffer.handle(),
                    GLsizeiptr::try_from(buffer.size()).unwrap(),
                )
            })
            .collect();

        let textures = textures.iter().map(|texture| texture.handle()).collect();

        let mut frame_buffer: GLuint = 0;
        if !attachments.is_empty() {
            unsafe {
                gl::sys::CreateFramebuffers(1, &mut frame_buffer);
                gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, frame_buffer);

                for (index, attachment) in attachments.iter().enumerate() {
                    gl::sys::BindTexture(gl::sys::TEXTURE_2D, attachment.handle());
                    gl::sys::FramebufferTexture2D(
                        gl::sys::DRAW_FRAMEBUFFER,
                        index_to_color_attachment_slot(index),
                        gl::sys::TEXTURE_2D,
                        attachment.handle(),
                        0,
                    );
                }
                let mut render_buffer: GLuint = 0;
                gl::sys::CreateRenderbuffers(1, &mut render_buffer);
                gl::sys::BindRenderbuffer(gl::sys::RENDERBUFFER, render_buffer);
                gl::sys::RenderbufferStorage(gl::sys::RENDERBUFFER, gl::sys::DEPTH24_STENCIL8, 1024, 1024);
                gl::sys::FramebufferRenderbuffer(
                    gl::sys::FRAMEBUFFER,
                    gl::sys::DEPTH_STENCIL_ATTACHMENT,
                    gl::sys::RENDERBUFFER,
                    render_buffer,
                );
            }

            match unsafe { gl::sys::CheckFramebufferStatus(gl::sys::DRAW_FRAMEBUFFER) } {
                gl::sys::FRAMEBUFFER_COMPLETE => (),
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
        unsafe { gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, self.frame_buffer) };

        self.program.set_used();
        unsafe {
            gl::sys::BindVertexArray(self.vertex_array_object);
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
                gl::sys::BindBufferRange(gl::sys::UNIFORM_BUFFER, index, *handle, 0 as GLintptr, *size);
            }
        }

        for (texture_slot, texture_handle) in self
            .textures
            .iter()
            .enumerate()
            .map(|(index, handle)| (index_to_texture_slot(index), handle))
        {
            unsafe {
                gl::sys::ActiveTexture(texture_slot);
                gl::sys::BindTexture(gl::sys::TEXTURE_2D, *texture_handle);
            }
        }
    }
}

fn index_to_texture_slot(index: usize) -> GLenum {
    GLenum::try_from(gl::sys::TEXTURE0 as usize + index).expect("Texture index too large")
}

fn index_to_color_attachment_slot(index: usize) -> GLenum {
    GLenum::try_from(gl::sys::COLOR_ATTACHMENT0 as usize + index).expect("Attachment index too large")
}

const fn convert_format(format: Format) -> (GLint, GLenum) {
    match format {
        Format::R32F => (1, gl::sys::FLOAT),
        Format::RG32F => (2, gl::sys::FLOAT),
        Format::RGB32F => (3, gl::sys::FLOAT),
        Format::RGBA32F => (4, gl::sys::FLOAT),
        Format::R8 => (1, gl::sys::BYTE),
        Format::RG8 => (2, gl::sys::BYTE),
        Format::RGB8 => (3, gl::sys::BYTE),
        Format::RGBA8 => (4, gl::sys::BYTE),
    }
}
