mod traffic;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
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

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    let (width, height) = canvas.output_size().unwrap();
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut rng = rand::thread_rng();
    let mut smart_road = SmartRoad::new();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
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
                    keycode: Some(Keycode::Up),
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
                    keycode: Some(Keycode::Left),
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
                    keycode: Some(Keycode::Right),
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
        canvas.set_draw_color(Color::GREY);
        canvas.clear();
        update_layout(&mut canvas);
        smart_road.regulate(&mut canvas);
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
    }
}

fn update_layout(canvas: &mut WindowCanvas) {
    let (width, height) = canvas.output_size().unwrap();
    let (center_w, center_h) = (width as i32 / 2, height as i32 / 2);
    let x = 20;
    let (l_lane1, l_lane2, l_lane3) = (center_w - x, center_w - 2 * x, center_w - 3 * x);
    let (low_lane1, low_lane2, low_lane3) = (center_h - x, center_h - 2 * x, center_h - 3 * x);
    let (r_lane1, r_lane2, r_lane3) = (center_w + x, center_w + 2 * x, center_w + 3 * x);
    let (top_lane1, top_lane2, top_lane3) = (center_h + x, center_h + 2 * x, center_h + 3 * x);
    canvas.set_draw_color(Color::BLACK);
    canvas
        .draw_rect(Rect::new(
            l_lane3,
            low_lane3,
            6 * x as u32 + 1,
            6 * x as u32 + 1,
        ))
        .unwrap();
    canvas
        .draw_rect(Rect::new(
            r_lane3,
            -1,
            l_lane3 as u32 + 1,
            l_lane3 as u32 + 2,
        ))
        .unwrap();
    canvas
        .draw_rect(Rect::new(-1, -1, l_lane3 as u32 + 2, l_lane3 as u32 + 2))
        .unwrap();
    canvas
        .draw_rect(Rect::new(
            -1,
            top_lane3,
            l_lane3 as u32 + 2,
            l_lane3 as u32 + 2,
        ))
        .unwrap();
    canvas
        .draw_rect(Rect::new(
            r_lane3,
            top_lane3,
            l_lane3 as u32 + 2,
            l_lane3 as u32 + 2,
        ))
        .unwrap();
    draw_horizontal_lines(
        vec![top_lane1, top_lane2, low_lane1, low_lane2, center_h],
        width as i32,
        canvas,
    );
    draw_vertical_lines(
        vec![l_lane1, l_lane2, r_lane1, r_lane2, center_w],
        height as i32,
        canvas,
    );
    canvas.present();
}

fn draw_horizontal_lines(y: Vec<i32>, max_w: i32, canvas: &mut WindowCanvas) {
    for p in y {
        canvas
            .draw_line(Point::new(0, p), Point::new(max_w, p))
            .unwrap();
    }
}
fn draw_vertical_lines(y: Vec<i32>, max_h: i32, canvas: &mut WindowCanvas) {
    for p in y {
        canvas
            .draw_line(Point::new(p, 0), Point::new(p, max_h))
            .unwrap();
    }
}
