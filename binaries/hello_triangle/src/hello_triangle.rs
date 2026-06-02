use alloc::rc::Rc;
use anyhow::Context;
use std::cell::RefCell;
use std::time::Instant;

use crate::gamma_window;
use gl::sys::types::{GLintptr, GLsizei};
use renderer::application::{App, Application, View};
use renderer::input::InputManager;
use renderer::resources::Resources;
use renderer::time::Time;
use renderer::{
    Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding,
};

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> anyhow::Result<Buffer> {
    let vertices = vec![
        -0.5f32, -0.5f32, 1f32, 0f32, 0f32, 0.5f32, -0.5f32, 0f32, 1f32, 0f32, 0f32, 0.5f32, 0f32,
        0f32, 1f32,
    ];
    let mut vertex_buffer =
        Buffer::allocate(BufferUsage::Vertex, size_of::<f32>() * vertices.len())?;
    let ptr = vertex_buffer.map::<f32>();
    ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();
    Ok(vertex_buffer)
}

pub struct HelloTriangle {
    render_pass: RenderPass,
    gamma_buffer: Buffer,
    vertex_buffer: Buffer,
    gamma: Rc<RefCell<f32>>,
    views: Vec<Box<dyn View>>,

    quit: bool,
}

impl App for HelloTriangle {
    fn new() -> anyhow::Result<Self> {
        let res = Resources::from_relative_exe_path(std::path::Path::new("assets"))?;

        let vertex_buffer = initialize_vertices()?;

        let vertex_shader = res
            .load_string("/shaders/triangle.vert")
            .map_err(Into::into)
            .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
            .context("Failed to initialize vertex shader")?;
        let fragment_shader = res
            .load_string("/shaders/triangle.frag")
            .map_err(Into::into)
            .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
            .context("Failed to initialize fragment shader")?;

        let gamma_buffer = Buffer::allocate(BufferUsage::Uniform, size_of::<f32>())?;

        let vertex_bindings = [
            VertexBinding::new(
                0,
                VertexAttribute::new(renderer::VertexAttributeFormat::RG32F, 0),
            ),
            VertexBinding::new(
                1,
                VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
            ),
        ];

        let main_render_pass = RenderPass::new(
            &vertex_shader,
            &fragment_shader,
            &vertex_bindings,
            &[&gamma_buffer],
            &[],
            &[],
        )?;
        let gamma = Rc::new(RefCell::new(1.0f32));
        Ok(Self {
            render_pass: main_render_pass,
            gamma_buffer,
            vertex_buffer,
            gamma: gamma.clone(),
            views: vec![Box::new(gamma_window::GammaWindow::new(gamma))],

            quit: false,
        })
    }
}

impl Application for HelloTriangle {
    fn tick(&mut self, _: &Time<Instant>, _: &dyn InputManager) {
        self.render_pass.display();

        unsafe {
            let gamma_ptr = self.gamma_buffer.map::<f32>();
            gamma_ptr.copy_from_slice(&[*self.gamma.borrow()]);
            self.gamma_buffer.unmap();

            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);
            gl::sys::BindVertexBuffer(
                0,
                self.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(size_of::<f32>() * 5).unwrap(),
            );
            gl::sys::BindVertexBuffer(
                1,
                self.vertex_buffer.handle(),
                GLintptr::try_from(size_of::<f32>() * 2).unwrap(),
                GLsizei::try_from(size_of::<f32>() * 5).unwrap(),
            );
            gl::sys::DrawArrays(gl::sys::TRIANGLES, 0, 3);
        }
    }

    fn views(&mut self) -> &mut [Box<dyn View>] {
        &mut self.views
    }

    fn quit(&self) -> bool {
        self.quit
    }
}
