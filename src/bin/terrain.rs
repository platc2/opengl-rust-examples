#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::must_use_candidate)]
extern crate alloc;
extern crate core;
extern crate gl;
extern crate sdl2;

use core::fmt::{Display, Formatter};

use imgui::SliderFlags;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use hello_triangle_rust::imgui_wrapper;
use hello_triangle_rust::key_codes::KeyCodes;
use hello_triangle_rust::mouse_buttons::MouseButtons;
use hello_triangle_rust::renderer_context::{OpenGLVersion, RendererContext, WindowDimension};

struct WGS84Coordinate {
    longitude: f32,
    latitude: f32,
}

impl WGS84Coordinate {
    pub const fn of(longitude: f32, latitude: f32) -> Self {
        Self {
            longitude,
            latitude,
        }
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
        write!(
            f,
            "{:3}°{:5.2}'{} {:3}°{:5.2}'{}",
            longitude.trunc(),
            longitude.fract() * 60f32,
            if self.longitude < 0f32 { 'W' } else { 'E' },
            latitude.trunc(),
            latitude.fract() * 60f32,
            if self.latitude > 0f32 { 'N' } else { 'S' }
        )
    }
}

fn main() -> Result<(), String> {
    // Initialize render-context
    let context = RendererContext::init(
        "Terrain",
        &WindowDimension::default(),
        &OpenGLVersion::default(),
    )
    .map_err(|e| format!("{e}"))?;

    let mut position = WGS84Coordinate::of(0f32, 0f32);
    let mut altitude = 1f32;
    let mut bearing = 0f32;
    let mut speed = 0.05f32;

    let mut mouse_buttons = MouseButtons::default();
    let mut key_codes = KeyCodes::default();
    let mut mouse_pos = (0, 0);
    let mut chars: Vec<char> = Vec::new();

    let mut event_pump = context
        .sdl()
        .event_pump()
        .expect("Failed to get event pump");

    let mut imgui_context = imgui_wrapper::Imgui::init();

    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
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
                Event::Quit { .. } => break 'main Ok(()),
                _ => {}
            }
        }

        if key_codes[Keycode::Escape] {
            break 'main Ok(());
        }

        imgui_context.prepare(
            [900f32, 700f32],
            [mouse_pos.0.into(), mouse_pos.1.into()],
            [
                mouse_buttons[MouseButton::Left],
                mouse_buttons[MouseButton::Right],
            ],
            &mut chars,
        );

        let up = key_codes[Keycode::Up];
        let down = key_codes[Keycode::Down];
        let left = key_codes[Keycode::Left];
        let right = key_codes[Keycode::Right];
        let forward = key_codes[Keycode::W];
        let backward = key_codes[Keycode::S];

        // Movement handling
        let delta_altitude = f32::from(u8::from(up ^ down)) * if up { speed } else { -speed };
        altitude = (altitude + delta_altitude).clamp(0f32, 10_000f32);

        let delta_bearing = f32::from(u8::from(left ^ right)) * if right { speed } else { -speed };
        bearing = (bearing + delta_bearing).rem_euclid(360f32);

        let delta_speed = f32::from(u8::from(forward ^ backward))
            * if forward { speed } else { -speed }
            * 0.05f32;
        position.offset(
            delta_speed * bearing.to_radians().cos(),
            delta_speed * bearing.to_radians().sin(),
        );

        imgui_context.render(|ui| {
            ui.window("Settings")
                .no_decoration()
                .movable(false)
                .save_settings(false)
                .always_auto_resize(true)
                .build(|| {
                    ui.text(format!("Position: {position}"));
                    ui.text(format!("Altitude: {altitude:.2}m"));
                    ui.text(format!(
                        "Bearing: {bearing:6.2}° ({})",
                        bearing_char(bearing)
                    ));
                    ui.separator();
                    ui.separator();
                    ui.slider_config("Movement speed", 0.1f32, 1f32)
                        .flags(SliderFlags::LOGARITHMIC)
                        .build(&mut speed);
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
