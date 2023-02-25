mod traffic;
use rand::Rng;
use traffic::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut v = Vehicle::new(800, 800, rng.gen(), rng.gen(), rng.gen());
    println!("{:?}", v);
    v.drive();
    println!("{:?}", v);
    v.accelerate();
    println!("{:?}", v);
    v.drive();
    println!("{:?}", v);
    v.deaccelerate();
    println!("{:?}", v);
    v.drive();
    println!("{:?}", v);
}
// use rand::Rng;
// use sdl2::event::Event;
// use sdl2::keyboard::Keycode;
// use sdl2::pixels::Color;
// use sdl2::rect::{Point, Rect};
// use sdl2::render::WindowCanvas;
// use std::time::Duration;

// fn main() {
//     let sdl_context = sdl2::init().unwrap();
//     let video_subsystem = sdl_context.video().unwrap();

//     let window = video_subsystem
//         .window("smart-road", 700, 700)
//         .position_centered()
//         .build()
//         .unwrap();
//     let mut canvas = window.into_canvas().build().unwrap();

//     canvas.set_draw_color(Color::RGB(0, 255, 255));
//     let (width, height) = canvas.output_size().unwrap();
//     canvas.clear();
//     canvas.present();
//     let mut event_pump = sdl_context.event_pump().unwrap();
//     let (from_north,from_east,from_west,from_south) = ((width as i32 / 2 - 20, -20),(width as i32, height as i32 / 2 - 20),(-20, height as i32 / 2),(width as i32 / 2, height as i32));
//     let mut rng = rand::thread_rng();
//     'running: loop {
//         for event in event_pump.poll_iter() {
//             match event {
//                 Event::Quit { .. }
//                 | Event::KeyDown {
//                     keycode: Some(Keycode::Escape),
//                     ..
//                 } => break 'running,
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Down),
//                     ..
//                 } => {
//                     let vehicle = Vehicle::new(from_south, rng.gen(), Direction::North, rng.gen());
//                 }
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Up),
//                     ..
//                 } => {
//                     let vehicle = Vehicle::new(from_north, rng.gen(), Direction::South, rng.gen());
//                 }
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Left),
//                     ..
//                 } => {
//                     let vehicle = Vehicle::new(from_west, rng.gen(), Direction::East, rng.gen());

//                 }
//                 Event::KeyDown {
//                     keycode: Some(Keycode::Right),
//                     ..
//                 } => {
//                     let mut vehicle = Vehicle::new(from_east, rng.gen(), Direction::West, rng.gen());
//                     vehicle.render(&mut canvas);
//                 }
//                 Event::KeyDown {
//                     keycode: Some(Keycode::R),
//                     ..
//                 } => {
//                     let direction: Direction = rng.gen();
//                     let position: (i32, i32);
//                     match direction {
//                         Direction::North => position = from_south,
//                         Direction::South => position = from_north,
//                         Direction::East => position = from_west,
//                         Direction::West => position = from_east,
//                     }
//                     let mut vehicle = Vehicle::new(position, rng.gen(), direction, rng.gen());
//                     vehicle.render(&mut canvas);
//                 }
//                 _ => {}
//             }
//         }
//         canvas.set_draw_color(Color::GREY);
//         canvas.clear();
//         update_layout(&mut canvas);
//         canvas.present();
//         ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
//     }
// }

// fn update_layout(canvas: &mut WindowCanvas) {
//     let (width, height) = canvas.output_size().unwrap();
//     let (center_w, center_h) = (width as i32 / 2, height as i32 / 2);
//     let x = 20;
//     let (l_lane1, l_lane2, l_lane3) = (center_w - x, center_w - 2 * x, center_w - 3 * x);
//     let (low_lane1, low_lane2, low_lane3) = (center_h - x, center_h - 2 * x, center_h - 3 * x);
//     let (r_lane1, r_lane2, r_lane3) = (center_w + x, center_w + 2 * x, center_w + 3 * x);
//     let (top_lane1, top_lane2, top_lane3) = (center_h + x, center_h + 2 * x, center_h + 3 * x);
//     canvas.set_draw_color(Color::BLACK);
//     canvas
//         .draw_rect(Rect::new(
//             l_lane3,
//             low_lane3,
//             6 * x as u32 + 1,
//             6 * x as u32 + 1,
//         ))
//         .unwrap();
//     canvas
//         .draw_rect(Rect::new(
//             r_lane3,
//             -1,
//             l_lane3 as u32 + 1,
//             l_lane3 as u32 + 2,
//         ))
//         .unwrap();
//     canvas
//         .draw_rect(Rect::new(-1, -1, l_lane3 as u32 + 2, l_lane3 as u32 + 2))
//         .unwrap();
//     canvas
//         .draw_rect(Rect::new(
//             -1,
//             top_lane3,
//             l_lane3 as u32 + 2,
//             l_lane3 as u32 + 2,
//         ))
//         .unwrap();
//     canvas
//         .draw_rect(Rect::new(
//             r_lane3,
//             top_lane3,
//             l_lane3 as u32 + 2,
//             l_lane3 as u32 + 2,
//         ))
//         .unwrap();
//     draw_horizontal_lines(
//         vec![top_lane1, top_lane2, low_lane1, low_lane2, center_h],
//         width as i32,
//         canvas,
//     );
//     draw_vertical_lines(
//         vec![l_lane1, l_lane2, r_lane1, r_lane2, center_w],
//         height as i32,
//         canvas,
//     );
//     canvas.present();
// }

// fn draw_horizontal_lines(y: Vec<i32>, max_w: i32, canvas: &mut WindowCanvas) {
//     for p in y {
//         canvas
//             .draw_line(Point::new(0, p), Point::new(max_w, p))
//             .unwrap();
//     }
// }
// fn draw_vertical_lines(y: Vec<i32>, max_h: i32, canvas: &mut WindowCanvas) {
//     for p in y {
//         canvas
//             .draw_line(Point::new(p, 0), Point::new(p, max_h))
//             .unwrap();
//     }
// }
