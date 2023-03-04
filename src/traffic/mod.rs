mod intersection;
mod smart_road;
mod traffic;
mod vehicle;
// use std::collections::HashMap;
// use std::vec;
pub use intersection::*;
pub use smart_road::*;
pub use traffic::*;
pub use vehicle::*;
// use sdl2::pixels::Color;
// use sdl2::rect::Rect;
// use sdl2::render::WindowCanvas;

// pub struct Traffic {
//     pub vehicles: Vec<Vec<Vehicle>>,
//     pub branches: HashMap<Direction, HashMap<Turning, Vec<Vehicle>>>,
//     pub lights: Vec<(Light, i32)>,
//     pub intersection: Vec<Vehicle>,
//     pub passed: Vec<Vehicle>,
// }

// #[derive(Clone, Copy)]
// pub enum Light {
//     Red,
//     Green,
// }

// impl Traffic {
//     pub fn new() -> Self {
//         Traffic {
//             vehicles: vec![vec![]; 4],
//             branches: HashMap::new(),
//             lights: vec![
//                 (Light::Green, 16),
//                 (Light::Red, 16),
//                 (Light::Red, 0),
//                 (Light::Red, 32),
//             ],
//             intersection: vec![],
//             passed: vec![],
//         }
//     }

//     pub fn update_vehicles(&mut self, canvas: &mut WindowCanvas) {
//         let mut to_release = vec![];
//         for ix in 0..self.passed.len() {
//             update_vehicle(canvas, &mut self.passed[ix], 10);
//             if passed_scope(&self.passed[ix], canvas) {
//                 to_release.push(ix);
//             }
//         }
//         for ix in (0..to_release.len()).rev() {
//             self.passed.remove(to_release[ix]);
//         }
//         let mut to_passed = vec![];
//         for ix in 0..self.intersection.len() {
//             update_vehicle(canvas, &mut self.intersection[ix], 10);
//             if passed_intersection(&self.intersection[ix], canvas) {
//                 to_passed.push(ix);
//             }
//         }
//         for ix in (0..to_passed.len()).rev() {
//             self.passed.push(self.intersection.remove(to_passed[ix]));
//         }
//         for route in &mut self.vehicles {
//             for ix in 0..route.len() {
//                 if stop_vehicle(&self.lights, canvas, &route[ix]) {
//                     update_vehicle(canvas, &mut route[ix], 0);
//                 } else if ix != 0 {
//                     if is_safe_distance(&route[ix], &route[ix - 1]) {
//                         update_vehicle(canvas, &mut route[ix], 10);
//                     } else {
//                         update_vehicle(canvas, &mut route[ix], 0);
//                     }
//                 } else {
//                     update_vehicle(canvas, &mut route[ix], 10);
//                 }
//             }
//             if route.len() > 0 && in_intersection(&route[0], &canvas) {
//                 self.intersection.push(route.remove(0));
//             }
//         }
//     }

//     pub fn add_vehicle(&mut self, vehicle: Vehicle) {
//         match vehicle.direction {
//             Direction::North => {
//                 self.vehicles[0].push(vehicle);
//             }
//             Direction::South => {
//                 self.vehicles[1].push(vehicle);
//             }
//             Direction::West => {
//                 self.vehicles[2].push(vehicle);
//             }
//             Direction::East => {
//                 self.vehicles[3].push(vehicle);
//             }
//         }
//     }
//     pub fn traffic_light_system(&mut self) {
//         for ix in 0..4 {
//             match self.lights[ix].0 {
//                 Light::Red => {
//                     self.lights[ix].1 += 1;
//                     if self.lights[ix].1 == 48 {
//                         self.lights[ix].0 = Light::Green;
//                         self.lights[ix].1 = 16;
//                     }
//                 }
//                 Light::Green => {
//                     self.lights[ix].1 -= 1;
//                     if self.lights[ix].1 == 0 {
//                         self.lights[ix].0 = Light::Red;
//                     } else if self.lights[ix].1 == 1 && self.is_busier(ix) {
//                         self.update_light_counter();
//                     }
//                 }
//             }
//         }
//     }

//     fn update_light_counter(&mut self) {
//         for light in &mut self.lights {
//             match light.0 {
//                 Light::Red => light.1 -= 16,
//                 Light::Green => light.1 += 16,
//             }
//         }
//     }
//     fn is_busier(&mut self, ix: usize) -> bool {
//         let total = self.vehicles[ix].len();
//         for i in 0..4 {
//             if i != ix {
//                 if total < self.vehicles[i].len() {
//                     return false;
//                 }
//             }
//         }
//         true
//     }
//     pub fn update_ligths(&mut self, canvas: &mut WindowCanvas) {
//         let (w, h) = canvas.output_size().unwrap();
//         //north - south lights
//         let south_light = Rect::new(w as i32 / 2 - 40, h as i32 / 2 - 40, 20, 20);
//         draw_lights(canvas, south_light, self.lights[1].0);
//         let north_light = Rect::new(w as i32 / 2 + 20, h as i32 / 2 + 20, 20, 20);
//         draw_lights(canvas, north_light, self.lights[0].0);
//         //west - east lights
//         let west_light = Rect::new(w as i32 / 2 + 20, h as i32 / 2 - 40, 20, 20);
//         draw_lights(canvas, west_light, self.lights[2].0);
//         let east_light = Rect::new(w as i32 / 2 - 40, h as i32 / 2 + 20, 20, 20);
//         draw_lights(canvas, east_light, self.lights[3].0);
//     }
// }

// fn draw_lights(canvas: &mut WindowCanvas, rect: Rect, light: Light) {
//     match light {
//         Light::Green => {
//             canvas.set_draw_color(Color::GREEN);
//             canvas.fill_rect(rect).unwrap();
//         }
//         Light::Red => {
//             canvas.set_draw_color(Color::RED);
//             canvas.fill_rect(rect).unwrap();
//         }
//     }
// }

// fn stop_vehicle(lights: &Vec<(Light, i32)>, canvas: &WindowCanvas, vehicle: &Vehicle) -> bool {
//     let (w, h) = canvas.output_size().unwrap();
//     match vehicle.direction {
//         Direction::North => match lights[0].0 {
//             Light::Green => return false,
//             Light::Red => {
//                 if vehicle.position.1 == h as i32 / 2 + 20 {
//                     return true;
//                 } else {
//                     return false;
//                 }
//             }
//         },
//         Direction::South => match lights[1].0 {
//             Light::Green => return false,
//             Light::Red => {
//                 if vehicle.position.1 == h as i32 / 2 - 40 {
//                     return true;
//                 } else {
//                     return false;
//                 }
//             }
//         },
//         Direction::West => match lights[2].0 {
//             Light::Green => return false,
//             Light::Red => {
//                 if vehicle.position.0 == w as i32 / 2 + 20 {
//                     return true;
//                 } else {
//                     return false;
//                 }
//             }
//         },
//         Direction::East => match lights[3].0 {
//             Light::Green => return false,
//             Light::Red => {
//                 if vehicle.position.0 == w as i32 / 2 - 40 {
//                     return true;
//                 } else {
//                     return false;
//                 }
//             }
//         },
//     }
// }
