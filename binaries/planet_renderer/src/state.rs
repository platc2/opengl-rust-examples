use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use imgui::Ui;
use nalgebra_glm as glm;

use gl::sys::types::{GLintptr, GLsizei};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;
use renderer::{Buffer, BufferUsage, RenderPass};

use crate::camera::{Camera, CameraController};
use crate::frustum::{Frustum, Plane};
use crate::matrix_uniform::MatrixUniform;
use crate::movable::Movable;
use crate::planet::Planet;
use crate::polyhedron::Polyhedron;
use crate::transform::Transform;
use crate::{camera, planet2};

struct CameraSettings {
    active_camera: Rc<RefCell<camera::PerspectiveCamera>>,
}

pub struct State {
    // Window configs
    camera_window_enabled: bool,
    terrain_window_enabled: bool,

    wireframe: bool,
    freeze_camera: bool,
    planet_mesh: Planet,
    fov: f32,
    near: f32,
    far: f32,
    camera: Rc<RefCell<camera::PerspectiveCamera>>,
    camera2: Rc<RefCell<camera::PerspectiveCamera>>,
    active_camera: Rc<RefCell<camera::PerspectiveCamera>>,
    camera_pos: glm::Vec3,
    camera_forward: glm::Vec3,
    camera_transform: glm::Mat4,
    frustum_vbx: Buffer,
    frustum_idx: Buffer,
    matrix_uniforms: MatrixUniform,
    matrix_uniform_buffer: Buffer,
    main_render_pass: RenderPass,
    quit: bool,

    planet2: planet2::Planet,
}

impl State {
    pub fn new(frustum_vbx: Buffer, frustum_idx: Buffer, matrix_uniform_buffer: Buffer, main_render_pass: RenderPass) -> Self {
        let fov: f32 = 60.;
        let near = 0.01;
        let far = 100.;

        let camera = Rc::new(RefCell::new(camera::PerspectiveCamera::new(
            900. / 700.,
            fov.to_radians(),
            near,
            far)));
        let camera2 = Rc::new(RefCell::new(camera::PerspectiveCamera::new(
            900. / 700.,
            fov.to_radians(),
            0.01,
            500.,
        )));

        let active_camera = camera.clone();

        let camera_pos = *camera.borrow().transform().position();
        let camera_forward = *camera.borrow().transform().forward();
        let camera_transform = *camera.borrow().transform().transform();
        Self {
            camera_window_enabled: false,
            terrain_window_enabled: false,

            wireframe: false,
            freeze_camera: false,
            planet_mesh: Planet::new().unwrap(),
            fov,
            near,
            far,
            camera,
            camera2,
            active_camera,
            camera_pos,
            camera_forward,
            camera_transform,
            frustum_vbx,
            frustum_idx,
            matrix_uniforms: MatrixUniform::default(),
            matrix_uniform_buffer,
            main_render_pass,
            quit: false,

            planet2: planet2::Planet::new(Transform::default()),
        }
    }
}

impl Application for State {
    fn tick(&mut self, time: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) { self.quit = true; }

        let moved = {
            self.active_camera.borrow_mut().update();
            let speed = time.duration().as_secs_f32();
            let camera_controller = CameraController::new(&self.active_camera);
            camera_controller.handle_input(input_manager, speed);
            self.active_camera.borrow().changed()
        };

        if !self.freeze_camera {
            let mut camera = self.camera.borrow();
            self.camera_pos = *camera.transform().position();
            self.camera_forward = *camera.transform().forward();
            self.camera_transform = *camera.transform().transform();
        }

        self.matrix_uniforms.model = *self.planet_mesh.transform.transform();
        //        planet_mesh.look_left(0.1 * time.duration().as_secs_f32());
        self.matrix_uniforms.projection = *self.active_camera.borrow().projection();
        self.matrix_uniforms.view = *self.active_camera.borrow().view();

        let matrix_uniforms_ptr = self.matrix_uniform_buffer.map::<MatrixUniform>();
        matrix_uniforms_ptr.copy_from_slice(&[self.matrix_uniforms]);
        self.matrix_uniform_buffer.unmap();

        unsafe {
            gl::sys::PolygonMode(
                gl::sys::FRONT_AND_BACK,
                if self.wireframe { gl::sys::LINE } else { gl::sys::FILL },
            );

            self.main_render_pass.display();

            let data = &[
                -0.5, -3f32.sqrt() / 6., 0.,
                0.5, -3f32.sqrt() / 6., 0.,
                0., 3f32.sqrt() / 3., 0.0,
            ];
            let mut vertex_buffer = Buffer::allocate(BufferUsage::Vertex, size_of::<f32>() * data.len())
                .unwrap();
            let ptr = vertex_buffer.map();
            ptr.copy_from_slice(data);
            vertex_buffer.unmap();

            gl::sys::Enable(gl::sys::CULL_FACE);
            gl::sys::CullFace(gl::sys::BACK);
            gl::sys::FrontFace(gl::sys::CCW);
            gl::sys::Enable(gl::sys::DEPTH_TEST);
            gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT | gl::sys::DEPTH_BUFFER_BIT);
            gl::sys::Viewport(0, 0, 900, 700);

            /*
                        gl::sys::BindVertexBuffer(
                            0,
                            self.planet_mesh.vertex_buffer.handle(),
                            0 as GLintptr,
                            GLsizei::try_from(size_of::<f32>() * 3).unwrap(),
                        );
            */

            gl::sys::BindVertexBuffer(
                0,
                vertex_buffer.handle(),
                0 as GLintptr,
                GLsizei::try_from(size_of::<f32>() * 3).unwrap(),
            );

            /*
                        gl::sys::DrawArrays(
                            gl::sys::TRIANGLES,
                            0,
                            self.planet_mesh.size as GLsizei,
                        );
            */
            gl::sys::Uniform1f(0, 1.);

            // Collect all nodes
            for node in self.planet2.all_nodes().iter() {
                let transform = node.transform;
                self.matrix_uniforms.model = *transform.transform();
                let matrix_uniforms_ptr = self.matrix_uniform_buffer.map::<MatrixUniform>();
                matrix_uniforms_ptr.copy_from_slice(&[self.matrix_uniforms]);
                self.matrix_uniform_buffer.unmap();
                gl::sys::DrawArrays(
                    gl::sys::TRIANGLES,
                    0,
                    3);
            }

            self.planet2.update(self.active_camera.borrow().transform().position(), &vec![
                10.,
//                50.,
            ]);

/*
            gl::sys::Uniform1f(0, 0.);
            self.matrix_uniforms.model = *self.planet_mesh.transform.transform();
            let matrix_uniforms_ptr = self.matrix_uniform_buffer.map::<MatrixUniform>();
            matrix_uniforms_ptr.copy_from_slice(&[self.matrix_uniforms]);
            self.matrix_uniform_buffer.unmap();
            for (a, b, c) in Polyhedron::regular_icosahedron().triangles.iter() {
                let mut v = Buffer::allocate(BufferUsage::Vertex, size_of::<f32>() * 9).unwrap();
                let ptr = v.map();
                let data = &[
                    a[0], a[1], a[2],
                    b[0], b[1], b[2],
                    c[0], c[1], c[2],
                ];
                ptr.copy_from_slice(data);
                v.unmap();
                gl::sys::BindVertexBuffer(0, v.handle(), 0, GLsizei::try_from(size_of::<f32>() * 3).unwrap());
                gl::sys::DrawArrays(
                    gl::sys::TRIANGLES,
                    0,
                    3);
                break;
            }
*/

            gl::sys::BindVertexBuffer(0, 0, 0, 0);
        }

        unsafe {
            // Draw Frustum
            let frustum = Frustum::from_perspective_camera(&self.camera.borrow());
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

            gl::sys::BindVertexBuffer(0, self.frustum_vbx.handle(), 0 as GLintptr, GLsizei::try_from(size_of::<f32>() * 3).unwrap());
            gl::sys::BindBuffer(gl::sys::ELEMENT_ARRAY_BUFFER, self.frustum_idx.handle());
            gl::sys::DrawElements(gl::sys::LINES, 24, gl::sys::UNSIGNED_SHORT, std::ptr::null());
        }
    }

    fn gui(&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu("Windows", || {
                ui.checkbox("Camera", &mut self.camera_window_enabled);
                ui.checkbox("Terrain", &mut self.terrain_window_enabled);
            });
        });

        if self.camera_window_enabled {
            ui.window("Camera")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    if ui.slider("Field of view", 1f32.to_radians(), 179f32.to_radians(), &mut self.fov) {
                        self.camera.borrow_mut().set_fov(self.fov);
                    }

                    if ui.slider("Near", 0.001, 10., &mut self.near) {
                        self.camera.borrow_mut().set_near(self.near);
                    }

                    if ui.slider("Far", 0.001, 100., &mut self.far) {
                        self.camera.borrow_mut().set_far(self.far);
                    }
                });
        }

        if self.terrain_window_enabled {
            ui.window("Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.checkbox("Wireframe", &mut self.wireframe);
                    if ui.checkbox("Freeze camera", &mut self.freeze_camera) {
                        let camera = self.camera.borrow();
                        let transform = camera.transform();
                        let mut camera2 = self.camera2.borrow_mut();
                        let mut transform2 = camera2.transform_mut();
                        transform2.set_position(*transform.position());;
                        transform2.set_rotation(*transform.rotation());
                        transform2.set_scale(*transform.scale());
                        self.active_camera = if self.freeze_camera {
                            self.camera2.clone()
                        } else {
                            self.camera.clone()
                        }
                    }
                    ui.columns(2, "col", false);
                    ui.text("Vertices");
                    ui.next_column();
                    ui.text_colored([1., 0.5, 0.5, 1.], format!("{}", self.planet_mesh.size));
                });
        }
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
    if denom.abs() < 0.00005 {
        panic!("UH OH!");
    }

    glm::vec3(
        glm::dot(&d, &u) / denom,
        glm::dot(&m3, &v) / denom,
        -glm::dot(&m2, &v) / denom,
    )
}
