use sdl2::render::WindowCanvas;

use super::{Intersection, Vehicle};

pub struct SmartRoad {
    pub intersection: Intersection,
}

impl SmartRoad {
    pub fn new() -> Self {
        SmartRoad {
            intersection: Intersection::new(),
        }
    }

    pub fn add_vehicle(&mut self, v: Vehicle) {
        self.intersection.add_vehicle(v);
    }
    pub fn regulate(&mut self, canvas: &mut WindowCanvas) {
        self.intersection.waiting();
        self.intersection.regulate(canvas);
        self.intersection.moves.drop_state();
    }
}
