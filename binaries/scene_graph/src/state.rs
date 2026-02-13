use std::cell::Ref;
use crate::polyhedron::{Polyhedron, Triangle};
use anyhow::Result;
use imgui::Ui;
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::resources::Resources;
use renderer::time::Time;
use renderer::{
    gl, Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute,
    VertexAttributeFormat, VertexBinding,
};
use std::time::Instant;

use crate::scene_graph::*;
use gl::sys::*;
use gl_bindings::sys::types::GLsizei;

pub struct State {
    quit: bool,
    res: Resources,

    cube: Polyhedron,
    cube_buffer: Buffer,
    render_pass: RenderPass,

    scene_graph: SceneGraph,

    show_scene_graph_window: bool,
    show_component_overview: bool,
}

impl State {
    pub fn new(res: Resources) -> Result<Self> {
        let cube = Polyhedron::cube();
        let triangles = cube.triangles();

        let cube_buffer = initialize_cube_buffer(triangles)?;
        let vertex_bindings = &[VertexBinding::new(
            0,
            VertexAttribute::new(VertexAttributeFormat::RGB32F, 0),
        )];

        let render_pass = RenderPass::new(
            &Shader::from_source(
                &res.load_string("/shaders/shader.vert")?,
                ShaderKind::Vertex,
            )?,
            &Shader::from_source(
                &res.load_string("/shaders/shader.frag")?,
                ShaderKind::Fragment,
            )?,
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

            scene_graph: SceneGraph::new(Node::new("Root")),

            show_scene_graph_window: false,
            show_component_overview: false,
        })
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: Ref<dyn InputManager>) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        unsafe {
            gl::sys::Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);

            self.render_pass.display();

            gl::sys::BindVertexBuffer(
                0,
                self.cube_buffer.handle(),
                0,
                GLsizei::try_from(size_of::<f32>() * 3).unwrap(),
            );
            gl::sys::DrawArrays(TRIANGLES, 0, 3 * self.cube.triangles().len() as i32);
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu("File", || {
                ui.separator();
                if ui.menu_item("Quit") {
                    self.quit = true;
                }
            });

            ui.menu("View", || {
                ui.menu_toggle("Scene Graph", &mut self.show_scene_graph_window);
                ui.menu_toggle("Component Overview", &mut self.show_component_overview);
            });
        });

        if self.show_scene_graph_window {
            ui.window("Scene Graph")
                .opened(&mut self.show_scene_graph_window)
                .build(|| {});

            ui.window("Component Overview")
                .opened(&mut self.show_component_overview)
                .build(|| {});
        }
    }

    fn quit(&self) -> bool {
        self.quit
    }
}

fn initialize_cube_buffer(triangles: &[Triangle]) -> Result<Buffer> {
    let data = triangles
        .iter()
        .flat_map(|(a, b, c)| vec![a, b, c])
        .flat_map(|v| vec![v.x, v.y, v.z])
        .collect::<Vec<_>>();

    let mut buffer = Buffer::allocate(BufferUsage::Vertex, size_of::<f32>() * data.len())?;
    let ptr = buffer.map();
    ptr.copy_from_slice(data.as_slice());
    buffer.unmap();

    Ok(buffer)
}

trait MenuOps {
    fn menu_toggle<S: AsRef<str>>(&self, name: S, toggle: &mut bool) -> bool;
}

impl MenuOps for Ui {
    fn menu_toggle<S: AsRef<str>>(&self, name: S, toggle: &mut bool) -> bool {
        if self.menu_item_config(name).selected(*toggle).build() {
            *toggle = !*toggle;
        }

        *toggle
    }
}
