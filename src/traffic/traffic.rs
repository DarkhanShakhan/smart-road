use sdl2::render::WindowCanvas;

use super::{Direction, Turning, Vehicle};

#[derive(Clone, Debug)]
pub struct Traffic {
    pub north: Branch,
    pub south: Branch,
    pub west: Branch,
    pub east: Branch,
}

impl Traffic {
    pub fn new() -> Self {
        Traffic {
            north: Branch::new(),
            south: Branch::new(),
            west: Branch::new(),
            east: Branch::new(),
        }
    }
    pub fn add(&mut self, v: Vehicle) {
        match v.direction {
            Direction::North => self.north.add(v),
            Direction::South => self.south.add(v),
            Direction::East => self.east.add(v),
            Direction::West => self.west.add(v),
        }
    }
    pub fn regulate(&mut self, canvas: &mut WindowCanvas) {
        self.north.regulate(canvas);
        self.south.regulate(canvas);
        self.west.regulate(canvas);
        self.east.regulate(canvas);
    }
}

#[derive(Clone, Debug)]
pub struct Branch {
    pub left: Vec<Vehicle>,
    pub straight: Vec<Vehicle>,
    pub right: Vec<Vehicle>,
}

impl Branch {
    pub fn new() -> Self {
        Branch {
            left: vec![],
            straight: vec![],
            right: vec![],
        }
    }
    pub fn add(&mut self, v: Vehicle) {
        match v.turn {
            Turning::Left => self.left.push(v),
            Turning::Straight => self.straight.push(v),
            Turning::Right => self.right.push(v),
        }
    }
    pub fn regulate(&mut self, canvas: &mut WindowCanvas) {
        for ix in 0..self.right.len() {
            if ix != 0 {
                if self.right[ix].is_safe_distance(self.right[ix - 1]) {
                    self.right[ix].drive();
                }
                if self.right[ix].speed <= self.right[ix].speed {
                    self.right[ix].accelerate();
                } else {
                    self.right[ix].deaccelerate();
                }
            } else {
                // self.right[ix].accelerate();
                self.right[ix].drive();
            }
            self.right[ix].render(canvas)
        }
        for ix in 0..self.left.len() {
            if ix != 0 {
                if self.left[ix].is_safe_distance(self.left[ix - 1]) {
                    self.left[ix].drive();
                }
                if self.left[ix].speed <= self.left[ix].speed {
                    self.left[ix].accelerate();
                } else {
                    self.left[ix].deaccelerate();
                }
            } else {
                self.left[ix].drive();
            }
            self.left[ix].render(canvas)
        }
        for ix in 0..self.straight.len() {
            if ix != 0 {
                if self.straight[ix].is_safe_distance(self.straight[ix - 1]) {
                    self.straight[ix].drive();
                }
                if self.straight[ix].speed <= self.straight[ix].speed {
                    self.straight[ix].accelerate();
                } else {
                    self.straight[ix].deaccelerate();
                }
            } else {
                self.straight[ix].drive();
            }
            self.straight[ix].render(canvas)
        }
    }
}
