#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use core::fmt::{Display, Formatter};
use std::path::Path;

use gl::types::GLsizei;
use imgui::SliderFlags;

use hello_triangle_rust::imgui_wrapper;
use hello_triangle_rust::renderer::{Buffer, BufferUsage, RenderPass, Shader, ShaderKind, Texture, VertexAttribute, VertexAttributeFormat, VertexBinding};
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

#[derive(Default)]
struct WGS84Coordinate {
    longitude: f32,
    latitude: f32,
}

impl WGS84Coordinate {
    pub fn offset(&mut self, longitude: f32, latitude: f32) {
        self.longitude = (self.longitude + longitude + 180f32).rem_euclid(360f32) - 180f32;
        self.latitude = (self.latitude + latitude + 90f32).rem_euclid(180f32) - 90f32;
    }
}

impl Display for WGS84Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let longitude = self.longitude.abs();
        let latitude = self.latitude.abs();
        write!(f, "{:3}°{:5.2}'{} {:3}°{:5.2}'{}",
               longitude.trunc(), longitude.fract() * 60f32, if self.longitude < 0f32 { 'W' } else { 'E' },
               latitude.trunc(), latitude.fract() * 60f32, if self.latitude > 0f32 { 'N' } else { 'S' })
    }
}

#[derive(Default)]
struct Position {
    pub pos: WGS84Coordinate,
    pub altitude: f32,
    pub bearing: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
struct CameraSettings {
    pub position: nalgebra_glm::TVec3<f32>,
    pub direction: nalgebra_glm::TVec3<f32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct WorldSettings {
    pub time: f32,
    pub planet_radius: f32,
    pub atmosphere_height: f32,
    pub inscatter_points: u32,
    pub optical_depth_points: u32,
    pub g: f32,
    pub intensity: f32,
    pub rayleigh_scale_height: f32,
    pub mie_scale_height: f32,
}

fn main() -> Result<(), String> {
    // Planet looks better if screen is a square
    let window_dimension = WindowDimension::of(800, 800);

    let context = RendererContext::init("Atmospheric Scattering", &window_dimension, OpenGLVersion::default())
        .map_err(|e| format!("{e}"))?;

    let res = Resources::from_relative_exe_path(Path::new("../../assets/atmospheric_scattering"))
        .map_err(|e| format!("{e}"))?;

    let vertices = vec![
        -1f32, 1f32, -1f32, -1f32, 1f32, -1f32,
        1f32, -1f32, 1f32, 1f32, -1f32, 1f32,
    ];
    let mut vertex_buffer = Buffer::allocate(BufferUsage::Vertex,
                                             std::mem::size_of::<f32>() * vertices.len())
        .map_err(|e| format!("{e}"))?;
    let vertex_ptr = vertex_buffer.map();
    vertex_ptr.copy_from_slice(&vertices);
    vertex_buffer.unmap();

    let mut camera_settings = CameraSettings::default();
    let mut camera_settings_buffer = Buffer::allocate(BufferUsage::Uniform,
                                                      std::mem::size_of::<CameraSettings>())
        .map_err(|e| format!("{e}"))?;

    let mut world_settings = WorldSettings {
        time: 0f32,
        planet_radius: 6731e3,
        atmosphere_height: 50e3,
        inscatter_points: 10,
        optical_depth_points: 10,
        g: 0.99,
        intensity: 1f32,
        rayleigh_scale_height: 7700f32,
        mie_scale_height: 1200f32,
    };
    let mut world_settings_buffer = Buffer::allocate(BufferUsage::Uniform,
                                                     std::mem::size_of::<WorldSettings>())
        .map_err(|e| format!("{e}"))?;

    let vertex = Shader::from_source(
        &res.load_string("/shaders/cube.vert").map_err(|e| format!("{e}"))?,
        ShaderKind::Vertex)
        .map_err(|e| format!("{e}"))?;
    let planet_fragment = Shader::from_source(
        &res.load_string("/shaders/planet.frag").map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment)
        .map_err(|e| format!("{e}"))?;
    let planet_vertex_bindings = [
        VertexBinding::new(0, VertexAttribute::new(VertexAttributeFormat::RG32F, 0))
    ];
    let planet_texture = Texture::blank(window_dimension.width, window_dimension.height);
    let planet_render_pass = RenderPass::new(&vertex, &planet_fragment,
                                             &planet_vertex_bindings, &[&camera_settings_buffer, &world_settings_buffer],
                                             &[], &[&planet_texture])
        .map_err(|e| format!("{e}"))?;

    let fragment = Shader::from_source(
        &res.load_string("/shaders/sky.frag").map_err(|e| format!("{e}"))?,
        ShaderKind::Fragment)
        .map_err(|e| format!("{e}"))?;

    let vertex_bindings = [
        VertexBinding::new(0, VertexAttribute::new(VertexAttributeFormat::RG32F, 0))
    ];

    let render_pass = RenderPass::new(&vertex, &fragment,
                                      &vertex_bindings, &[&camera_settings_buffer,
            &world_settings_buffer], &[&planet_texture], &[])
        .map_err(|e| format!("{e}"))?;

    let mut position = Position { altitude: 1f32, ..Position::default() };
    let mut speed = 0.05f32;

    let mut mouse_pos = (0, 0);
    let mut mouse_left = false;
    let mut mouse_right = false;
    let mut up = false;
    let mut down = false;
    let mut left = false;
    let mut right = false;
    let mut forward = false;
    let mut backward = false;

    let mut move_time = true;

    let mut event_pump = context.sdl().event_pump().expect("Failed to get event pump");

    let mut chars: Vec<char> = Vec::new();

    let mut imgui = imgui_wrapper::Imgui::init();

    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            use sdl2::mouse::MouseButton;
            use sdl2::keyboard::Keycode;
            match event {
                Event::MouseMotion { x, y, .. } => mouse_pos = (
                    // This is ok - Mouse coordinates shouldn't reach numbers which overflow 16bit
                    i16::try_from(x).unwrap_or(0),
                    i16::try_from(y).unwrap_or(0)),
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => mouse_left = true,
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => mouse_left = false,
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => mouse_right = true,
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => mouse_right = false,
                Event::KeyDown { keycode: Some(Keycode::LCtrl), .. } => down = true,
                Event::KeyUp { keycode: Some(Keycode::LCtrl), .. } => down = false,
                Event::KeyDown { keycode: Some(Keycode::LShift), .. } => up = true,
                Event::KeyUp { keycode: Some(Keycode::LShift), .. } => up = false,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => left = true,
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => left = false,
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => right = true,
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => right = false,
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => forward = true,
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => forward = false,
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => backward = true,
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => backward = false,
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'main Ok(()),
                Event::KeyDown { keycode: Some(key_code), .. } => {
                    let key_code = key_code as u32;
                    if (32..512).contains(&key_code) { chars.push(char::from_u32(key_code).unwrap()); }
                }
                _ => {}
            }
        }

        imgui.prepare(
            [window_dimension.width as f32, window_dimension.height as f32],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [mouse_left, mouse_right],
            &mut chars);

        // Movement handling
        let delta_altitude = f32::from(u8::from(up ^ down)) * if up { speed } else { -speed } * 5e3;
        position.altitude = (position.altitude + delta_altitude).clamp(1f32, world_settings.planet_radius * 20f32);

        let delta_bearing = f32::from(u8::from(left ^ right)) * if right { speed } else { -speed };
        position.bearing = (position.bearing + delta_bearing).rem_euclid(360f32);

        let delta_speed = f32::from(u8::from(forward ^ backward)) * if forward { speed } else { -speed } * 0.05f32;
        position.pos.offset(delta_speed * position.bearing.to_radians().cos(), delta_speed * position.bearing.to_radians().sin());

        camera_settings.position.y = world_settings.planet_radius + position.altitude;
        camera_settings.position.z = position.altitude * 1e2;
        if move_time {
            const SPEED: f32 = 0.1f32;
            world_settings.time = (world_settings.time + 0.001f32 * SPEED) % 1f32;
        }

        let camera_settings_ptr = camera_settings_buffer.map();
        camera_settings_ptr.copy_from_slice(&[camera_settings]);
        camera_settings_buffer.unmap();

        let world_settings_ptr = world_settings_buffer.map();
        world_settings_ptr.copy_from_slice(&[world_settings]);
        world_settings_buffer.unmap();

        planet_render_pass.display();
        render_cube(&vertex_buffer);

        render_pass.display();
        render_cube(&vertex_buffer);

        imgui.render(|ui| {
            imgui::Window::new("Settings")
                .no_decoration()
                .movable(false)
                .save_settings(false)
                .always_auto_resize(true)
                .build(ui, || {
                    ui.text(format!("Position: {}", position.pos));
                    ui.text(format!("Altitude: {:.0}km", position.altitude / 1e3));
                    ui.text(format!("Bearing: {:6.2}° ({})", position.bearing, bearing_char(position.bearing)));
                    ui.separator();
                    ui.separator();
                    imgui::Slider::new("Movement speed", 0.1f32, 1f32)
                        .flags(SliderFlags::LOGARITHMIC)
                        .build(ui, &mut speed);
                });

            imgui::Window::new("World Settings")
                .save_settings(false)
                .always_auto_resize(true)
                .build(ui, || {
                    imgui::Slider::new("Time", 0f32, 1f32)
                        .build(ui, &mut world_settings.time);
                    ui.same_line();
                    ui.checkbox("", &mut move_time);
                    imgui::Slider::new("Planet radius", 1e6, 7e6)
                        .display_format(format!("{:.0}km", world_settings.planet_radius / 1e3))
                        .build(ui, &mut world_settings.planet_radius);
                    imgui::Slider::new("Atmosphere height", 0f32, 1e6)
                        .display_format(format!("{:.0}km (Total {:.0}km)",
                                                world_settings.atmosphere_height / 1e3,
                                                (world_settings.planet_radius + world_settings.atmosphere_height) / 1e3))
                        .build(ui, &mut world_settings.atmosphere_height);
                    imgui::Slider::new("Inscatter sample points", 1u32, 16u32)
                        .build(ui, &mut world_settings.inscatter_points);
                    imgui::Slider::new("Optical-depth sample points", 1u32, 16u32)
                        .build(ui, &mut world_settings.optical_depth_points);
                    imgui::Slider::new("g", 1e-4, 1f32 - 1e-4)
                        .build(ui, &mut world_settings.g);
                    imgui::Slider::new("Sun intensity", 0f32, 5f32)
                        .flags(SliderFlags::LOGARITHMIC)
                        .build(ui, &mut world_settings.intensity);
                    imgui::Slider::new("Rayleigh scale-height", 0f32, 10_000f32)
                        .flags(SliderFlags::LOGARITHMIC)
                        .build(ui, &mut world_settings.rayleigh_scale_height);
                    imgui::Slider::new("Mie scale-height", 0f32, 10_000f32)
                        .flags(SliderFlags::LOGARITHMIC)
                        .build(ui, &mut world_settings.mie_scale_height);
                });
        });

        context.window().gl_swap_window();
    }
}

fn bearing_char(bearing: f32) -> &'static str {
    // TODO - How to make a safe cast for clippy?
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let segment = (((bearing + 22.5f32) % 360f32) / 45f32).trunc() as u8;
    match segment {
        0 => "N",
        1 => "NE",
        2 => "E",
        3 => "SE",
        4 => "S",
        5 => "SW",
        6 => "W",
        7 => "NW",
        _ => panic!("Uh oh"),
    }
}

fn render_cube(vertex_buffer: &Buffer) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::BindVertexBuffer(0, vertex_buffer.handle(), 0, GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap());
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}
