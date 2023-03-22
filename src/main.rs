mod layout;
mod traffic;
use layout::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use traffic::*;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("smart-road", 800, 800)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::WHITE);
    let (width, height) = canvas.output_size().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut smart_road = SmartRoad::new();
    let texture_creator = canvas.texture_creator();
    let car_texture = texture_creator.load_texture("assets/car.png").unwrap();
    let city_texture_creator = canvas.texture_creator();
    let city_texture = city_texture_creator
        .load_texture("assets/city.png")
        .unwrap();

    let ttf_context = sdl2::ttf::init().unwrap();
    let exp_font = ttf_context
        .load_font(std::path::Path::new("assets/expressway.otf"), 150)
        .unwrap();
    let mut is_stats = false;
    let mut stats = Stats::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    if is_stats {
                        break 'running;
                    } else {
                        is_stats = true;
                        stats = smart_road.stats();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rng.gen(),
                        Direction::North,
                        rng.gen(),
                    ));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rng.gen(),
                        Direction::South,
                        rng.gen(),
                    ));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rng.gen(),
                        Direction::East,
                        rng.gen(),
                    ));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rng.gen(),
                        Direction::West,
                        rng.gen(),
                    ));
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    smart_road.add_vehicle(Vehicle::new(
                        width,
                        height,
                        rng.gen(),
                        rng.gen(),
                        rng.gen(),
                    ));
                }
                _ => {}
            }
        }
        if is_stats {
            stats_layout(&mut canvas, stats, &exp_font, &city_texture);
        } else {
            update_layout(&mut canvas, &city_texture);
            smart_road.regulate(&mut canvas, &car_texture);
        }
        canvas.present();
        std::thread::sleep(Duration::from_millis(100));
    }
}
