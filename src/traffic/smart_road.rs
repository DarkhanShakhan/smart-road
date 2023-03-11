use sdl2::render::WindowCanvas;

use super::{Intersection, Released, Traffic, Vehicle};

pub struct SmartRoad {
    pub intersection: Intersection,
    pub traffic: Traffic,
    pub released: Released,
}

impl SmartRoad {
    pub fn new() -> Self {
        SmartRoad {
            intersection: Intersection::new(),
            traffic: Traffic::new(),
            released: Released::new(),
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
        self.released.regulate(canvas);
        if to_int.len() > 0 {
            self.intersection.add_vehicles(to_int)
        }
        if from_int.len() > 0 {
            self.released.add_vehicles(from_int)
        }
        self.intersection.moves.drop_state();
    }
}
