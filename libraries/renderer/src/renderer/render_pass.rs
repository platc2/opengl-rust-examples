use thiserror::Error;

use crate::renderer::labelled::Labelled;
use crate::renderer::render_pass::Error::IncompleteFramebuffer;
use crate::renderer::{Buffer, Program, Shader, Texture};
use gl::sys::types::{GLenum, GLintptr, GLsizeiptr, GLuint};

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
    program: Program,
    uniform_buffers: Vec<(GLuint, GLsizeiptr)>,
    textures: Vec<GLuint>,
    frame_buffer: GLuint,
}

impl Labelled for RenderPass {
    fn set_label(&mut self, label: &str) {
        let _label = format!("{label}::framebuffer");
        if self.frame_buffer > 0 {
/*
            unsafe {
                gl::sys::ObjectLabel(
                    gl::sys::FRAMEBUFFER,
                    self.frame_buffer,
                    label.len() as _,
                    label.as_ptr().cast(),
                );
            }
*/
        }
    }

    fn identifier(&self) -> GLenum {
        unimplemented!()
    }

    fn name(&self) -> GLuint {
        unimplemented!()
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
        uniform_buffers: &[&Buffer],
        textures: &[&Texture],
        attachments: &[&Texture],
    ) -> Result<Self> {
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
/*
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
                gl::sys::RenderbufferStorage(
                    gl::sys::RENDERBUFFER,
                    gl::sys::DEPTH24_STENCIL8,
                    1024,
                    1024,
                );
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

            unsafe {
                gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, 0);
            }
*/
        }

        Ok(Self {
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
        uniform_buffers: &[&Buffer],
        textures: &[&Texture],
        attachments: &[&Texture],
    ) -> Result<Self> {
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
/*
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
                gl::sys::RenderbufferStorage(
                    gl::sys::RENDERBUFFER,
                    gl::sys::DEPTH24_STENCIL8,
                    1024,
                    1024,
                );
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

            unsafe {
                gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, 0);
            }
*/
        }

        Ok(Self {
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
        uniform_buffers: &[&Buffer],
        textures: &[&Texture],
        attachments: &[&Texture],
    ) -> Result<Self> {
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
/*
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
                gl::sys::RenderbufferStorage(
                    gl::sys::RENDERBUFFER,
                    gl::sys::DEPTH24_STENCIL8,
                    1024,
                    1024,
                );
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

            unsafe {
                gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, 0);
            }
*/
        }

        Ok(Self {
            program,
            uniform_buffers,
            textures,
            frame_buffer,
        })
    }

    pub fn display(&self) {
        let buffer = if self.frame_buffer > 0 {
            self.frame_buffer
        } else {
            0
        };
/*
        unsafe { gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, buffer) };
*/

        self.program.set_used();

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
/*
            unsafe {
                gl::sys::BindBufferRange(
                    gl::sys::UNIFORM_BUFFER,
                    index,
                    *handle,
                    0 as GLintptr,
                    *size,
                );
            }
*/
        }

        for (texture_slot, texture_handle) in self
            .textures
            .iter()
            .enumerate()
            .map(|(index, handle)| (index_to_texture_slot(index), handle))
        {
/*
            unsafe {
                gl::sys::ActiveTexture(texture_slot);
                gl::sys::BindTexture(gl::sys::TEXTURE_2D, *texture_handle);
            }
*/
        }

        if self.frame_buffer > 0 {
            //            unsafe { gl::sys::BindFramebuffer(gl::sys::DRAW_FRAMEBUFFER, 0); }
        }
    }
}

fn index_to_texture_slot(index: usize) -> GLenum {
    GLenum::try_from(gl::sys::TEXTURE0 as usize + index).expect("Texture index too large")
}

fn index_to_color_attachment_slot(index: usize) -> GLenum {
    GLenum::try_from(gl::sys::COLOR_ATTACHMENT0 as usize + index)
        .expect("Attachment index too large")
}
