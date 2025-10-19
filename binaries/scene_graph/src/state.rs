use crate::polyhedron::{Polyhedron, Triangle};
use anyhow::Result;
use imgui::Ui;
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::resources::Resources;
use renderer::time::Time;
use renderer::{gl, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexAttributeFormat, VertexBinding};
use std::time::Instant;

use gl::sys::*;
use gl_bindings::sys::types::GLsizei;

pub struct State {
    quit: bool,
    res: Resources,

    cube: Polyhedron,
    cube_buffer: Buffer,
    render_pass: RenderPass,
}

impl State {
    pub fn new(res: Resources) -> Result<Self> {
        let cube = Polyhedron::cube();
        let triangles = cube.triangles();

        let cube_buffer = initialize_cube_buffer(&triangles)?;
        let vertex_bindings = &[
            VertexBinding::new(0, VertexAttribute::new(VertexAttributeFormat::RGB32F, 0))
        ];

        let render_pass = RenderPass::new(
            &Shader::from_source(&*res.load_string("/shaders/shader.vert")?, ShaderKind::Vertex)?,
            &Shader::from_source(&*res.load_string("/shaders/shader.frag")?, ShaderKind::Fragment)?,
            vertex_bindings,
            &[],
            &[],
            &[],
        )?;

        Ok(Self {
            quit: false,
            res,

            cube,
            cube_buffer,
            render_pass,
        })
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) { self.quit = true; }

        unsafe {
            gl::sys::Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);

            self.render_pass.display();

            gl::sys::BindVertexBuffer(0, self.cube_buffer.handle(), 0,
                                      GLsizei::try_from(size_of::<f32>() * 3).unwrap());
            gl::sys::DrawArrays(TRIANGLES, 0, 3 * self.cube.triangles().len() as i32);
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu("File", || {
                ui.separator();
                if ui.menu_item("Quit") { self.quit = true; }
            });

            ui.menu("View", || {});
        });
    }

    fn quit(&self) -> bool { self.quit }
}

fn initialize_cube_buffer(triangles: &[Triangle]) -> Result<Buffer> {
    let data = triangles.into_iter()
        .flat_map(|(a, b, c)| vec![a, b, c])
        .flat_map(|v| vec![v.x, v.y, v.z])
        .collect::<Vec<_>>();

    let mut buffer = Buffer::allocate(BufferUsage::Vertex, size_of::<f32>() * data.len())?;
    let ptr = buffer.map();
    ptr.copy_from_slice(data.as_slice());
    buffer.unmap();

    Ok(buffer)
}
