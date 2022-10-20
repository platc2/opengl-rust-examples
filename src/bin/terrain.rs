#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use core::fmt::{Display, Formatter};
use std::path::Path;

use imgui::SliderFlags;

use hello_triangle_rust::imgui_wrapper;
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};
use hello_triangle_rust::resources::Resources;

struct WGS84Coordinate {
    longitude: f32,
    latitude: f32,
}

impl WGS84Coordinate {
    pub const fn of(longitude: f32, latitude: f32) -> Self {
        Self { longitude, latitude }
    }

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

fn main() -> Result<(), String> {
    println!("{:.2}", 2.0);
    // Initialize render-context
    let context = RendererContext::init("Terrain", WindowDimension::default(), OpenGLVersion::default())
        .map_err(|e| format!("{e}"))?;

    // TODO - Remove if unused
    let _res = Resources::from_relative_exe_path(Path::new("../../assets/terrain"))
        .map_err(|e| format!("{e}"))?;

    let mut position = WGS84Coordinate::of(0f32, 0f32);
    let mut altitude = 1f32;
    let mut bearing = 0f32;
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

    let mut event_pump = context.sdl().event_pump().expect("Failed to get event pump");

    let mut chars: Vec<char> = Vec::new();

    let mut imgui_context = imgui_wrapper::Imgui::init();

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

        imgui_context.prepare(
            [900f32, 700f32],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [mouse_left, mouse_right],
            &mut chars);

        // Movement handling
        let delta_altitude = f32::from(u8::from(up ^ down)) * if up { speed } else { -speed };
        altitude = (altitude + delta_altitude).clamp(0f32, 10_000f32);

        let delta_bearing = f32::from(u8::from(left ^ right)) * if right { speed } else { -speed };
        bearing = (bearing + delta_bearing).rem_euclid(360f32);

        let delta_speed = f32::from(u8::from(forward ^ backward)) * if forward { speed } else { -speed } * 0.05f32;
        position.offset(delta_speed * bearing.to_radians().cos(), delta_speed * bearing.to_radians().sin());

        imgui_context.render(|ui| {
            imgui::Window::new("Settings")
                .no_decoration()
                .movable(false)
                .save_settings(false)
                .always_auto_resize(true)
                .build(ui, || {
                    ui.text(format!("Position: {}", position));
                    ui.text(format!("Altitude: {:.2}m", altitude));
                    ui.text(format!("Bearing: {:6.2}° ({})", bearing, bearing_char(bearing)));
                    ui.separator();
                    ui.separator();
                    imgui::Slider::new("Movement speed", 0.1f32, 1f32)
                        .flags(SliderFlags::LOGARITHMIC)
                        .build(ui, &mut speed);
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
