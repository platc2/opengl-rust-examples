#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]

use std::collections::VecDeque;
use std::path::Path;

use anyhow::{Context, Result};
use gl::types::{GLintptr, GLsizei};
use nalgebra_glm as glm;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use hello_triangle_rust::{imgui_wrapper, renderer};
use hello_triangle_rust::key_codes::KeyCodes;
use hello_triangle_rust::mouse_buttons::MouseButtons;
use hello_triangle_rust::renderer::{Buffer, BufferUsage, RenderPass, Shader, ShaderKind, VertexAttribute, VertexBinding};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

use crate::camera::Camera;
use crate::frustum::{Frustum, Plane};
use crate::matrix_uniform::MatrixUniform;
use crate::movable::Movable;
use crate::planet::Planet;
use crate::time::Time;

mod matrix_uniform;
mod camera;
mod planet;
mod icosahedron;
mod polyhedron;
mod transform;
mod time;
mod movable;
mod frustum;

pub fn main() -> Result<()> {
    let mut time = Time::<std::time::Instant>::new();

    let window_dimension = WindowDimension::default();
    // Initialize render-context
    let context = RendererContext::init(
        "Planet Renderer",
        &window_dimension,
        &OpenGLVersion::default(),
    )?;

    let res = Resources::from_relative_exe_path(Path::new("assets/planet_renderer"))?;

    let mut matrix_uniform_buffer =
        Buffer::allocate(BufferUsage::Uniform, std::mem::size_of::<MatrixUniform>())?;
    let mut matrix_uniforms = MatrixUniform::default();

    let mut fov: f32 = 60f32;
    let mut near: f32 = 0.01;
    let mut far: f32 = 100.;
    let mut camera = camera::PerspectiveCamera::new(900. / 700., fov.to_radians(), near, far);
    let mut camera2 = camera::PerspectiveCamera::new(900. / 700., 60f32.to_radians(), 0.01, 500.);

    let mut planet_mesh = Planet::new()
        .context("Failed to initialize planet mesh")?;

    let planet_vertex_shader = res
        .load_string("/shaders/planet.vert")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Vertex))
        .context("Failed to initialize terrain vertex shader")?;
    let planet_fragment_shader = res
        .load_string("/shaders/planet.frag")
        .map_err(Into::into)
        .and_then(|source| Shader::from_source(&source, ShaderKind::Fragment))
        .context("Failed to initialize terrain fragment shader")?;

    let vertex_bindings = [VertexBinding::new(
        0,
        VertexAttribute::new(renderer::VertexAttributeFormat::RGB32F, 0),
    )];

    let main_render_pass = RenderPass::new(
        &planet_vertex_shader,
        &planet_fragment_shader,
        &vertex_bindings,
        &[&matrix_uniform_buffer],
        &[],
        &[],
    )?;

    let mut mouse_buttons = MouseButtons::default();
    let mut key_codes = KeyCodes::default();
    let mut mouse_pos = (0, 0);

    let mut event_pump = context
        .sdl()
        .event_pump()
        .expect("Failed to get event pump");

    let mut chars: Vec<char> = Vec::new();

    let mut imgui_context = imgui_wrapper::Imgui::init();
    let mut wireframe = false;
    let mut old_level = 0;
    let mut max_level = 0;
    let mut freeze_camera = false;
    let mut last_freeze_state = false;
    let mut camera_pos = *camera.transform_mut().position();
    let mut camera_forward = *camera.transform_mut().forward();
    let mut camera_transform = *camera.transform().transform();

    let mut frustum_vbx = Buffer::allocate(BufferUsage::Vertex, std::mem::size_of::<glm::Vec3>() * 8)?;
    let mut frustum_idx = Buffer::allocate(BufferUsage::Index, std::mem::size_of::<u16>() * 24)?;

    let fps_values = 1000;
    let mut fps = VecDeque::new();
    'main: loop {
        camera.update();
        camera2.update();
        time.update();
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::keyboard::Keycode;
            match event {
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = (
                        // This is ok - Mouse coordinates shouldn't reach numbers which overflow 16bit
                        i16::try_from(x).unwrap_or(0),
                        i16::try_from(y).unwrap_or(0),
                    );
                }
                Event::MouseButtonDown { mouse_btn, .. } => mouse_buttons[mouse_btn] = true,
                Event::MouseButtonUp { mouse_btn, .. } => mouse_buttons[mouse_btn] = false,
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main Ok(()),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    key_codes[keycode] = true;

                    let keycode = keycode as u32;
                    if (32..512).contains(&keycode) {
                        chars.push(char::from_u32(keycode).unwrap());
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => key_codes[keycode] = false,
                _ => {}
            }
        }

        if last_freeze_state != freeze_camera {
            last_freeze_state = freeze_camera;
        }

        if !freeze_camera {
            camera_pos = *camera.transform_mut().position();
            camera_forward = *camera.transform_mut().forward();
            camera_transform = *camera.transform().transform();
        }

        let mut moved = false;
        let speed = time.duration().as_secs_f32();
        if key_codes[Keycode::W] {
            camera.move_forward(speed);
            camera2.move_forward(speed);
            moved = true;
        }
        if key_codes[Keycode::S] {
            camera.move_backward(speed);
            camera2.move_backward(speed);
            moved = true;
        }
        if key_codes[Keycode::D] {
            camera.move_right(-speed);
            camera2.move_right(-speed);
            moved = true;
        }
        if key_codes[Keycode::A] {
            camera.move_left(-speed);
            camera2.move_left(-speed);
            moved = true;
        }
        if key_codes[Keycode::Space] {
            camera.move_up(speed);
            camera2.move_up(speed);
            moved = true;
        }
        if key_codes[Keycode::LCtrl] {
            camera.move_down(speed);
            camera2.move_down(speed);
            moved = true;
        }
        if key_codes[Keycode::Up] {
            camera.look_up(speed);
            camera2.look_up(speed);
            moved = true;
        }
        if key_codes[Keycode::Down] {
            camera.look_down(speed);
            camera2.look_down(speed);
            moved = true;
        }
        if key_codes[Keycode::Right] {
            camera.look_right(speed);
            camera2.look_right(speed);
            moved = true;
        }
        if key_codes[Keycode::Left] {
            camera.look_left(speed);
            camera2.look_left(speed);
            moved = true;
        }
        if key_codes[Keycode::Q] {
            camera.roll_ccw(speed);
            camera2.roll_ccw(speed);
            moved = true;
        }
        if key_codes[Keycode::E] {
            camera.roll_cw(speed);
            camera2.roll_cw(speed);
            moved = true;
        }

        matrix_uniforms.model = *planet_mesh.transform.transform();
//        planet_mesh.look_left(0.1 * time.duration().as_secs_f32());
        if freeze_camera {
            matrix_uniforms.projection = *camera2.projection();
            matrix_uniforms.view = *camera2.view();
        } else {
            matrix_uniforms.projection = *camera.projection();
            matrix_uniforms.view = *camera.view();
        }

        let matrix_uniforms_ptr = matrix_uniform_buffer.map::<MatrixUniform>();
        matrix_uniforms_ptr.copy_from_slice(&[matrix_uniforms]);
        matrix_uniform_buffer.unmap();

        imgui_context.prepare(
            [
                window_dimension.width as f32,
                window_dimension.height as f32,
            ],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [
                mouse_buttons[MouseButton::Left],
                mouse_buttons[MouseButton::Right],
            ],
            &mut chars,
        );

        unsafe {
            gl::PolygonMode(
                gl::FRONT_AND_BACK,
                if wireframe { gl::LINE } else { gl::FILL },
            );

            main_render_pass.display();

            if (old_level != max_level || moved) {
                planet_mesh.recalculate(max_level, &camera_pos, Some(&camera_forward));
                old_level = max_level;
            }

            gl::Enable(gl::CULL_FACE);
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 900, 700);

            gl::BindVertexBuffer(
                0,
                planet_mesh.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );

            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                planet_mesh.size as GLsizei,
            );

            gl::BindVertexBuffer(0, 0, 0, 0);
        }

        unsafe {
            // Draw Frustum
            let frustum = Frustum::from_perspective_camera(&camera);
            let vertices = [
                // Near
                plane_intersection(&frustum.near_face, &frustum.top_face, &frustum.left_face),
                plane_intersection(&frustum.near_face, &frustum.top_face, &frustum.right_face),
                plane_intersection(&frustum.near_face, &frustum.bottom_face, &frustum.right_face),
                plane_intersection(&frustum.near_face, &frustum.bottom_face, &frustum.left_face),

                // Far
                plane_intersection(&frustum.far_face, &frustum.top_face, &frustum.left_face),
                plane_intersection(&frustum.far_face, &frustum.top_face, &frustum.right_face),
                plane_intersection(&frustum.far_face, &frustum.bottom_face, &frustum.right_face),
                plane_intersection(&frustum.far_face, &frustum.bottom_face, &frustum.left_face),
            ];
            let a = plane_intersection(&frustum.near_face, &frustum.top_face, &frustum.left_face);
            let b = plane_intersection(&frustum.far_face, &frustum.top_face, &frustum.left_face);
//            println!("{:?} - {:?}", a, b);
            println!("{:?}, {:?}", frustum.near_face.position, frustum.far_face.position);
//            println!("{:?}", vertices);
            let indices: [u16; 24] = [
                0, 1, 1, 2, 2, 3, 3, 0,
                4, 5, 5, 6, 6, 7, 7, 4,
                0, 4, 1, 5, 2, 6, 3, 7,
            ];
            {
                let frustum_vbx_ptr = frustum_vbx.map();
                let frustum_idx_ptr = frustum_idx.map();
                frustum_vbx_ptr.copy_from_slice(&vertices[..]);
                frustum_idx_ptr.copy_from_slice(&indices[..]);
                frustum_vbx.unmap();
                frustum_idx.unmap();
            }

            matrix_uniforms.model = camera_transform;
            let matrix_uniforms_ptr = matrix_uniform_buffer.map::<MatrixUniform>();
            matrix_uniforms_ptr.copy_from_slice(&[matrix_uniforms]);
            matrix_uniform_buffer.unmap();

            gl::BindVertexBuffer(0, frustum_vbx.handle(), 0 as GLintptr, GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap());
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, frustum_idx.handle());
            gl::DrawElements(gl::LINES, 24, gl::UNSIGNED_SHORT, std::ptr::null());
        }

        let current_fps = time.fps();
        if fps.len() >= fps_values {
            fps.pop_back();
        }
        fps.push_front(current_fps);
        imgui_context.render(|ui| {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.checkbox("Wireframe", &mut wireframe);
                    ui.slider("Max Level", 0, 10, &mut max_level);
                    ui.checkbox("Freeze camera", &mut freeze_camera);
                    ui.plot_lines(format!("FPS: {}", current_fps), fps.make_contiguous())
                        .scale_min(0.)
                        .scale_max(200.)
                        .build();
                    ui.columns(2, "col", false);
                    ui.text("Vertices");
                    ui.next_column();
                    ui.text_colored([1., 0.5, 0.5, 1.], format!("{}", planet_mesh.size));
                });

            ui.window("Camera Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    if ui.slider("Field of view", 1f32.to_radians(), 179f32.to_radians(), &mut fov) {
                        camera.set_fov(fov);
                    }

                    if ui.slider("Near", 0.001, 10., &mut near) {
                        camera.set_near(near);
                    }

                    if ui.slider("Far", 0.001, 100., &mut far) {
                        camera.set_far(far);
                    }
                });
        });

        context.window().gl_swap_window();
    }
}

fn plane_intersection(p1: &Plane, p2: &Plane, p3: &Plane) -> glm::Vec3 {
    let m1 = glm::vec3(p1.normal.x, p2.normal.x, p3.normal.x);
    let m2 = glm::vec3(p1.normal.y, p2.normal.y, p3.normal.y);
    let m3 = glm::vec3(p1.normal.z, p2.normal.z, p3.normal.z);
    let d = glm::vec3(
        glm::dot(&p1.normal, &-p1.position),
        glm::dot(&p2.normal, &-p2.position),
        glm::dot(&p3.normal, &-p3.position));

    let u = glm::cross(&m2, &m3);
    let v = glm::cross(&m1, &d);

    let denom = glm::dot(&m1, &u);
    if (denom.abs() < 0.00005) {
        panic!("UH OH!");
    }

    glm::vec3(
        glm::dot(&d, &u) / denom,
        glm::dot(&m3, &v) / denom,
        -glm::dot(&m2, &v) / denom,
    )
}
