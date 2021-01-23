extern crate sdl2;

use std::time::Duration;

use anyhow::Result;
use bounding_box::BoundingBox;
use fixed_point_physics::{bounding_box, coordinate::Coordinate, vector::Vector};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
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

fn main() -> Result<()> {
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

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    // TODO: Apply any changes to the point's momentum/velocity
                }
            }
        }

        // Where should the point move to?
        pt = pt.travel(&bounding_box)?;

        // let (x, y) = pt.position.rounded_as_ints();

        // Render the point.
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.set_draw_color(Color::RED);
        canvas.fill_rect(Some(pt.into())).map_err(Sdl2Error::from)?;
        // canvas.draw_point((x, y)).map_err(Sdl2Error::from)?;
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
