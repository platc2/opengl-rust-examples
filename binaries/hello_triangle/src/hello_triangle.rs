use crate::gamma_window;
use alloc::rc::Rc;
use anyhow::Context;
use renderer::application::{App, Application, View};
use renderer::input::InputManager;
use renderer::resources::Resources;
use renderer::time::Time;
use renderer::Format::RG32F;
use renderer::{Buffer, BufferUsage, Mesh, MeshBuilder, RenderPass, Shader, ShaderKind};
use std::cell::RefCell;
use std::time::Instant;

/// # Errors
/// - Fail to initialize vertex buffer
fn initialize_vertices() -> anyhow::Result<Buffer> {
    let triangle = renderer::primitives::triangle();
    let vertex_data = triangle.as_f32_slice();
    Ok(Buffer::load_data(BufferUsage::Vertex, vertex_data)?)
}

pub struct HelloTriangle {
    render_pass: RenderPass,
    gamma_buffer: Buffer,
    mesh: Mesh,
    gamma: Rc<RefCell<f32>>,
    views: Vec<Box<dyn View>>,

    quit: bool,
}

impl App for HelloTriangle {
    fn new() -> anyhow::Result<Self> {
        let res = Resources::from_relative_exe_path(std::path::Path::new("assets"))?;

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

        let main_render_pass =
            RenderPass::new(&vertex_shader, &fragment_shader, &[&gamma_buffer], &[], &[])?;

        let vertex_buffer = Rc::new(initialize_vertices()?);
        let mesh = MeshBuilder::new()
            .vertex_buffer(vertex_buffer, 0, i32::try_from(size_of::<f32>() * 8)?)
            .attribute(0, RG32F, 0)
            .build();

        let gamma = Rc::new(RefCell::new(1.0f32));
        Ok(Self {
            render_pass: main_render_pass,
            gamma_buffer,
            mesh,
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

/*
            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);
*/

            self.mesh.bind();

/*
            gl::sys::DrawArrays(gl::sys::TRIANGLES, 0, 3);
*/
        }
    }

    fn views(&mut self) -> &mut [Box<dyn View>] {
        &mut self.views
    }

    fn quit(&self) -> bool {
        self.quit
    }
}
