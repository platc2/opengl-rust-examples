use std::time::Instant;

use imgui::{SliderFlags, Ui};

use gl::sys::types::GLsizei;
use renderer::{Buffer, RenderPass};
use renderer::application::Application;
use renderer::input_manager::{InputManager, Key};
use renderer::time::Time;

use crate::{CameraSettings, Position, WorldSettings};

pub struct State {
    position: Position,
    speed: f32,
    world_settings: WorldSettings,
    camera_settings: CameraSettings,
    world_settings_buffer: Buffer,
    camera_settings_buffer: Buffer,
    planet_render_pass: RenderPass,
    render_pass: RenderPass,
    vertex_buffer: Buffer,
    move_time: bool,

    quit: bool,
}

impl State {
    pub fn new(world_settings_buffer: Buffer,
               camera_settings_buffer: Buffer,
               planet_render_pass: RenderPass,
               render_pass: RenderPass,
               vertex_buffer: Buffer) -> Self {
        Self {
            position: Position {
                altitude: 1f32,
                ..Position::default()
            },
            speed: 0.05,
            world_settings: WorldSettings {
                time: 0f32,
                planet_radius: 6731e3,
                atmosphere_height: 50e3,
                inscatter_points: 10,
                optical_depth_points: 10,
                g: 0.99,
                intensity: 1f32,
                rayleigh_scale_height: 7700f32,
                mie_scale_height: 1200f32,
            },
            camera_settings: CameraSettings::default(),
            world_settings_buffer,
            camera_settings_buffer,
            planet_render_pass,
            render_pass,
            vertex_buffer,
            move_time: true,
            quit: false,
        }
    }
}

impl Application for State {
    fn tick(&mut self, _: &Time<Instant>, input_manager: &dyn InputManager) {
        if input_manager.key_down(Key::ESCAPE) {
            self.quit = true;
        }

        let up = input_manager.key_down(Key::LEFT_SHIFT);
        let down = input_manager.key_down(Key::LEFT_CONTROL);
        let left = input_manager.key_down(Key::LEFT_ARROW);
        let right = input_manager.key_down(Key::RIGHT_ARROW);
        let forward = input_manager.key_down(Key::UP_ARROW);
        let backward = input_manager.key_down(Key::DOWN_ARROW);

        // Movement handling
        let delta_altitude = f32::from(u8::from(up ^ down)) * if up { self.speed } else { -self.speed } * 5e3;
        self.position.altitude =
            (self.position.altitude + delta_altitude).clamp(1f32, self.world_settings.planet_radius * 20f32);

        let delta_bearing = f32::from(u8::from(left ^ right)) * if right { self.speed } else { -self.speed };
        self.position.bearing = (self.position.bearing + delta_bearing).rem_euclid(360f32);

        let delta_speed = f32::from(u8::from(forward ^ backward))
            * if forward { self.speed } else { -self.speed }
            * 0.05f32;
        self.position.pos.offset(
            delta_speed * self.position.bearing.to_radians().cos(),
            delta_speed * self.position.bearing.to_radians().sin(),
        );

        self.camera_settings.position.y = self.world_settings.planet_radius + self.position.altitude;
        self.camera_settings.position.z = self.position.altitude * 1e2;
        if self.move_time {
            const SPEED: f32 = 0.1f32;
            self.world_settings.time = (self.world_settings.time + 0.001f32 * SPEED) % 1f32;
        }

        let camera_settings_ptr = self.camera_settings_buffer.map();
        camera_settings_ptr.copy_from_slice(&[self.camera_settings]);
        self.camera_settings_buffer.unmap();

        let world_settings_ptr = self.world_settings_buffer.map();
        world_settings_ptr.copy_from_slice(&[self.world_settings]);
        self.world_settings_buffer.unmap();

        self.planet_render_pass.display();
        render_cube(&self.vertex_buffer);

        self.render_pass.display();
        render_cube(&self.vertex_buffer);
    }

    fn gui(&mut self, ui: &Ui) {
        ui.window("Settings")
            .no_decoration()
            .movable(false)
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.text(format!("Position: {}", self.position.pos));
                ui.text(format!("Altitude: {:.0}km", self.position.altitude / 1e3));
                ui.text(format!(
                    "Bearing: {:6.2}° ({})",
                    self.position.bearing,
                    bearing_char(self.position.bearing)
                ));
                ui.separator();
                ui.separator();
                ui.slider_config("Movement speed", 0.1f32, 1f32)
                    .flags(SliderFlags::LOGARITHMIC)
                    .build(&mut self.speed);
            });

        ui.window("World Settings")
            .save_settings(false)
            .always_auto_resize(true)
            .build(|| {
                ui.slider("Time", 0f32, 1f32, &mut self.world_settings.time);
                ui.same_line();
                ui.checkbox("## Move time", &mut self.move_time);
                ui.slider_config("Planet radius", 1e6, 7e6)
                    .display_format(format!("{:.0}km", self.world_settings.planet_radius / 1e3))
                    .build(&mut self.world_settings.planet_radius);
                ui.slider_config("Atmosphere height", 0f32, 1e6)
                    .display_format(format!(
                        "{:.0}km (Total {:.0}km)",
                        self.world_settings.atmosphere_height / 1e3,
                        (self.world_settings.planet_radius + self.world_settings.atmosphere_height) / 1e3
                    ))
                    .build(&mut self.world_settings.atmosphere_height);
                ui.slider(
                    "Inscatter sample points",
                    1u32,
                    16u32,
                    &mut self.world_settings.inscatter_points,
                );
                ui.slider(
                    "Optical-depth sample points",
                    1u32,
                    16u32,
                    &mut self.world_settings.optical_depth_points,
                );
                ui.slider("g", 1e-4, 1f32 - 1e-4, &mut self.world_settings.g);
                ui.slider_config("Sun intensity", 0f32, 5f32)
                    .flags(SliderFlags::LOGARITHMIC)
                    .build(&mut self.world_settings.intensity);
                ui.slider_config("Rayleigh scale-height", 0f32, 10_000f32)
                    .flags(SliderFlags::LOGARITHMIC)
                    .build(&mut self.world_settings.rayleigh_scale_height);
                ui.slider_config("Mie scale-height", 0f32, 10_000f32)
                    .flags(SliderFlags::LOGARITHMIC)
                    .build(&mut self.world_settings.mie_scale_height);
            });
    }

    fn quit(&self) -> bool {
        self.quit
    }
}

fn render_cube(vertex_buffer: &Buffer) {
    unsafe {
        gl::sys::Clear(gl::sys::COLOR_BUFFER_BIT);

        gl::sys::BindVertexBuffer(
            0,
            vertex_buffer.handle(),
            0,
            GLsizei::try_from(std::mem::size_of::<f32>() * 2).unwrap(),
        );
        gl::sys::DrawArrays(gl::sys::TRIANGLES, 0, 6);
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
