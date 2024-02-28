use std::time::Instant;

use imgui::Ui;
use nalgebra_glm as glm;

use gl::sys::types::{GLintptr, GLsizei};
use renderer::{Buffer, RenderPass};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;

use crate::camera;
use crate::camera::Camera;
use crate::frustum::{Frustum, Plane};
use crate::matrix_uniform::MatrixUniform;
use crate::movable::Movable;
use crate::planet::Planet;

pub struct State {
    wireframe: bool,
    max_level: u16,
    old_level: u16,
    freeze_camera: bool,
    last_freeze_state: bool,
    planet_mesh: Planet,
    fov: f32,
    near: f32,
    far: f32,
    camera: camera::PerspectiveCamera,
    camera2: camera::PerspectiveCamera,
    camera_pos: glm::Vec3,
    camera_forward: glm::Vec3,
    camera_transform: glm::Mat4,
    frustum_vbx: Buffer,
    frustum_idx: Buffer,
    matrix_uniforms: MatrixUniform,
    matrix_uniform_buffer: Buffer,
    main_render_pass: RenderPass,
    quit: bool,
}

impl State {
    pub fn new(frustum_vbx: Buffer, frustum_idx: Buffer, matrix_uniform_buffer: Buffer, main_render_pass: RenderPass) -> Self {
        let fov: f32 = 60.;
        let near = 0.01;
        let far = 100.;
        let camera = camera::PerspectiveCamera::new(900. / 700., fov.to_radians(), near, far);
        let camera_pos = *camera.transform().position();
        let camera_forward = *camera.transform().forward();
        let camera_transform = *camera.transform().transform();
        Self {
            wireframe: false,
            max_level: 0,
            old_level: 0,
            freeze_camera: false,
            last_freeze_state: false,
            planet_mesh: Planet::new().unwrap(),
            fov,
            near,
            far,
            camera,
            camera2: camera::PerspectiveCamera::new(900. / 700., fov.to_radians(), 0.01, 500.),
            camera_pos,
            camera_forward,
            camera_transform,
            frustum_vbx,
            frustum_idx,
            matrix_uniforms: MatrixUniform::default(),
            matrix_uniform_buffer,
            main_render_pass,
            quit: false,
        }
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        self.camera.update();
        self.camera2.update();

        if self.last_freeze_state != self.freeze_camera {
            self.last_freeze_state = self.freeze_camera;
        }

        if !self.freeze_camera {
            self.camera_pos = *self.camera.transform_mut().position();
            self.camera_forward = *self.camera.transform_mut().forward();
            self.camera_transform = *self.camera.transform().transform();
        }

        let mut moved = false;
        let speed = time.duration().as_secs_f32();
        if input_manager.key_down(Key::W) {
            self.camera.move_forward(speed);
            self.camera2.move_forward(speed);
            moved = true;
        }
        if input_manager.key_down(Key::S) {
            self.camera.move_backward(speed);
            self.camera2.move_backward(speed);
            moved = true;
        }
        if input_manager.key_down(Key::D) {
            self.camera.move_right(-speed);
            self.camera2.move_right(-speed);
            moved = true;
        }
        if input_manager.key_down(Key::A) {
            self.camera.move_left(-speed);
            self.camera2.move_left(-speed);
            moved = true;
        }
        if input_manager.key_down(Key::SPACE) {
            self.camera.move_up(speed);
            self.camera2.move_up(speed);
            moved = true;
        }
        if input_manager.key_down(Key::LEFT_CONTROL) {
            self.camera.move_down(speed);
            self.camera2.move_down(speed);
            moved = true;
        }
        if input_manager.key_down(Key::UP_ARROW) {
            self.camera.look_up(speed);
            self.camera2.look_up(speed);
            moved = true;
        }
        if input_manager.key_down(Key::DOWN_ARROW) {
            self.camera.look_down(speed);
            self.camera2.look_down(speed);
            moved = true;
        }
        if input_manager.key_down(Key::RIGHT_ARROW) {
            self.camera.look_right(speed);
            self.camera2.look_right(speed);
            moved = true;
        }
        if input_manager.key_down(Key::LEFT_ARROW) {
            self.camera.look_left(speed);
            self.camera2.look_left(speed);
            moved = true;
        }
        if input_manager.key_down(Key::Q) {
            self.camera.roll_ccw(speed);
            self.camera2.roll_ccw(speed);
            moved = true;
        }
        if input_manager.key_down(Key::E) {
            self.camera.roll_cw(speed);
            self.camera2.roll_cw(speed);
            moved = true;
        }

        self.matrix_uniforms.model = *self.planet_mesh.transform.transform();
//        planet_mesh.look_left(0.1 * time.duration().as_secs_f32());
        if self.freeze_camera {
            self.matrix_uniforms.projection = *self.camera2.projection();
            self.matrix_uniforms.view = *self.camera2.view();
        } else {
            self.matrix_uniforms.projection = *self.camera.projection();
            self.matrix_uniforms.view = *self.camera.view();
        }

        let matrix_uniforms_ptr = self.matrix_uniform_buffer.map::<MatrixUniform>();
        matrix_uniforms_ptr.copy_from_slice(&[self.matrix_uniforms]);
        self.matrix_uniform_buffer.unmap();

        unsafe {
            gl::sys::PolygonMode(
                gl::sys::FRONT_AND_BACK,
                if self.wireframe { gl::sys::LINE } else { gl::sys::FILL },
            );

            self.main_render_pass.display();

            if (self.old_level != self.max_level || moved) {
                self.planet_mesh.recalculate(self.max_level, &self.camera_pos, Some(&self.camera_forward));
                self.old_level = self.max_level;
            }

//            gl_bindings::Enable(gl_bindings::CULL_FACE);
            gl::sys::CullFace(gl::sys::BACK);
            gl::sys::FrontFace(gl::sys::BACK);
            gl::sys::Enable(gl::sys::DEPTH_TEST);
            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT | gl::sys::DEPTH_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);

            gl::sys::BindVertexBuffer(
                0,
                self.planet_mesh.vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap(),
            );

            gl::sys::DrawArrays(
                gl::sys::TRIANGLES,
                0,
                self.planet_mesh.size as GLsizei,
            );

            gl::sys::BindVertexBuffer(0, 0, 0, 0);
        }

        unsafe {
            // Draw Frustum
            let frustum = Frustum::from_perspective_camera(&self.camera);
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
                let frustum_vbx_ptr = self.frustum_vbx.map();
                let frustum_idx_ptr = self.frustum_idx.map();
                frustum_vbx_ptr.copy_from_slice(&vertices[..]);
                frustum_idx_ptr.copy_from_slice(&indices[..]);
                self.frustum_vbx.unmap();
                self.frustum_idx.unmap();
            }

            self.matrix_uniforms.model = self.camera_transform;
            let matrix_uniforms_ptr = self.matrix_uniform_buffer.map::<MatrixUniform>();
            matrix_uniforms_ptr.copy_from_slice(&[self.matrix_uniforms]);
            self.matrix_uniform_buffer.unmap();

            gl::sys::BindVertexBuffer(0, self.frustum_vbx.handle(), 0 as GLintptr, GLsizei::try_from(std::mem::size_of::<f32>() * 3).unwrap());
            gl::sys::BindBuffer(gl::sys::ELEMENT_ARRAY_BUFFER, self.frustum_idx.handle());
            gl::sys::DrawElements(gl::sys::LINES, 24, gl::sys::UNSIGNED_SHORT, std::ptr::null());
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.window("Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.checkbox("Wireframe", &mut self.wireframe);
                ui.slider("Max Level", 0, 10, &mut self.max_level);
                ui.checkbox("Freeze camera", &mut self.freeze_camera);
                /*
                                ui.plot_lines(format!("FPS: {}", self.current_fps), self.fps.make_contiguous())
                                    .scale_min(0.)
                                    .scale_max(200.)
                                    .build();
                */
                ui.columns(2, "col", false);
                ui.text("Vertices");
                ui.next_column();
                ui.text_colored([1., 0.5, 0.5, 1.], format!("{}", self.planet_mesh.size));
            });

        ui.window("Camera Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                if ui.slider("Field of view", 1f32.to_radians(), 179f32.to_radians(), &mut self.fov) {
                    self.camera.set_fov(self.fov);
                }

                if ui.slider("Near", 0.001, 10., &mut self.near) {
                    self.camera.set_near(self.near);
                }

                if ui.slider("Far", 0.001, 100., &mut self.far) {
                    self.camera.set_far(self.far);
                }
            });
    }

    fn quit(&self) -> bool {
        self.quit
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
