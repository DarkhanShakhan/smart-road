use sdl2::render::WindowCanvas;

use super::{Intersection, Traffic, Vehicle};

pub struct SmartRoad {
    pub intersection: Intersection,
    pub traffic: Traffic,
    pub released: Traffic,
}

impl SmartRoad {
    pub fn new() -> Self {
        SmartRoad {
            intersection: Intersection::new(),
            traffic: Traffic::new(),
            released: Traffic::new(),
        }
    }

    pub fn add_vehicle(&mut self, v: Vehicle) {
        self.traffic.add_vehicle(v);
    }
    pub fn regulate(&mut self, canvas: &mut WindowCanvas) {
        let mut to_int = vec![];
        match self.traffic.regulate(canvas) {
            Some(vehicles) => to_int = vehicles,
            None => {}
        }
        let mut from_int = vec![];
        match self.intersection.regulate(canvas) {
            Some(vehicles) => from_int = vehicles,
            None => {}
        }
        match self.released.regulate(canvas) {
            Some(vehicles) => println!("still there: {}", vehicles.len()),
            None => {}
        }
        if to_int.len() > 0 {
            self.intersection.add_vehicles(to_int)
        }
        if from_int.len() > 0 {
            self.released.add_vehicles(from_int)
        }
    }
}