#![feature(bool_to_option)]

extern crate sdl2;

use std::{convert::TryInto, path::Path, time::Duration, vec};

use anyhow::Result;
use bounding_box::BoundingBox;
use fixed_point_physics::{
    bounding_box,
    coordinate::Coordinate,
    drag::Drag,
    gravity::Gravity,
    traits::{Force, GenericForce},
    vector::Vector,
};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    ttf::Font,
    video::Window,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Sdl2Error {
    #[error("{0}")]
    GenericError(String),
}

impl From<String> for Sdl2Error {
    fn from(err: String) -> Self {
        Sdl2Error::GenericError(err)
    }
}

fn is_direction_press(code: Scancode) -> bool {
    match code {
        Scancode::W
        | Scancode::A
        | Scancode::S
        | Scancode::D
        | Scancode::Up
        | Scancode::Left
        | Scancode::Down
        | Scancode::Right => true,
        _ => false,
    }
}

fn draw_fps(fps: f64, font: &Font, canvas: &mut Canvas<Window>, fps_box: &Rect) -> Result<()> {
    // render a surface, and convert it to a texture bound to the canvas
    let surface = font.render(&fps.to_string()).blended(Color::WHITE)?;
    // .blended(Color::RGBA(255, 0, 0, 255))?;
    // .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface)?;
    // .map_err(|e| e.to_string())?;

    canvas
        .copy(&texture, None, *fps_box)
        .map_err(Sdl2Error::from)?;

    Ok(())
}

fn main() -> Result<()> {
    // Grab font .ttf file as a CLI param
    let args: Vec<_> = std::env::args().collect();

    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());

    let path = match args.len() < 2 {
        true => {
            println!("Usage: ./demo font.[ttf|ttc|fon]");
            std::process::exit(1);
        }
        false => Path::new(&args[1]),
    };

    // else {
    //     let path: &Path =
    // }

    // SDL setup
    let sdl_context = sdl2::init().map_err(Sdl2Error::from)?;

    let video_subsystem = sdl_context.video().unwrap();

    let width = 800;
    let height = 600;

    let window = video_subsystem
        .window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Initialize font
    let ttf_context = sdl2::ttf::init()?;
    let font = ttf_context.load_font(path, 128).map_err(Sdl2Error::from)?;
    let fps_box = Rect::new(0, 0, 200, 80);

    // Create the simulation environment
    let bounding_box = BoundingBox::new(width.into(), height.into())?;

    // This point will bounce around the bounding box according to forces
    // applied to it.
    let mut pt = fixed_point_physics::point::Point {
        position: Coordinate {
            x: (width / 2) as f64,
            y: (height / 2) as f64,
        },
        velocity: Vector { x: 2.0, y: 2.0 },
    };

    // std::thread::sleep(Duration::new(2, 0));

    let forces: Vec<Box<dyn Force>> = vec![Box::new(Gravity::new(0.2)), Box::new(Drag::new(0.98))];
    let mut user_force = GenericForce::default();

    let mut fps_counter = fps_counter::FPSCounter::default();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Figure out which keys are being pressed and add up a vector
        let keys: Vec<Scancode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(|code| is_direction_press(code).then_some(code))
            .collect();
        user_force.vec.zero_out();
        for key in keys {
            let vector: Vector = key.try_into()?;
            user_force.vec = user_force.vec + vector.scale(0.3);
        }
        user_force.vec.min(1.0);

        // Where should the point move to?
        pt = pt.travel(&bounding_box, &forces, Some(&user_force))?;

        // Render the point.
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(Some(pt.into())).map_err(Sdl2Error::from)?;

        // Draw the frames per second in the corner.
        let fps = fps_counter.tick();
        draw_fps(fps as f64, &font, &mut canvas, &fps_box)?;

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
