use sdl2::render::WindowCanvas;
use std::collections::VecDeque;

use super::{Speed, Vehicle};
pub struct Released {
    vehicles: VecDeque<Vehicle>,
}

impl Released {
    pub fn new() -> Self {
        Released {
            vehicles: VecDeque::new(),
        }
    }
    pub fn add_vehicles(&mut self, vs: Vec<Vehicle>) {
        for v in vs {
            self.vehicles.push_back(v);
        }
    }

    pub fn regulate(&mut self, canvas: &mut WindowCanvas) {
        let mut list = vec![];
        for ix in 0..self.vehicles.len() {
            self.vehicles[ix].drive();
            self.vehicles[ix].render(canvas);
            if self.vehicles[ix].speed != Speed::High {
                self.vehicles[ix].accelerate();
            }

            if self.vehicles[ix].is_out() {
                list.push(ix);
            }
        }
        list.reverse();
        for ix in list {
            self.vehicles.remove(ix);
        }
    }
}
