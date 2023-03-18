use sdl2::render::{Texture, WindowCanvas};

use super::{Intersection, Vehicle};

pub struct SmartRoad {
    pub intersection: Intersection,
    total_cars: u32,
    average_velocity: u32,
}

impl SmartRoad {
    pub fn new() -> Self {
        SmartRoad {
            intersection: Intersection::new(),
            total_cars: 0,
            average_velocity: 0,
        }
    }

    pub fn add_vehicle(&mut self, v: Vehicle) {
        self.intersection.add_vehicle(v);
        self.total_cars += 1;
    }
    pub fn regulate(&mut self, canvas: &mut WindowCanvas, texture: &Texture) {
        self.intersection.waiting();
        self.intersection.regulate(canvas, texture);
        self.intersection.moves.drop_state();
        self.average_velocity = (self.intersection.average_velocity() + self.average_velocity) / 2;
    }
    pub fn stats(self) -> (u32, u32) {
        (self.total_cars, self.average_velocity)
    }
}
