use std::collections::VecDeque;

use sdl2::render::WindowCanvas;

use super::{Direction, Speed, Turning, Vehicle};

#[derive(Clone, Debug)]
pub struct Traffic {
    north: Branch,
    south: Branch,
    west: Branch,
    east: Branch,
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
    pub fn add_vehicle(&mut self, v: Vehicle) {
        match v.direction {
            Direction::North => self.north.add_vehicle(v),
            Direction::South => self.south.add_vehicle(v),
            Direction::East => self.east.add_vehicle(v),
            Direction::West => self.west.add_vehicle(v),
        }
    }

    pub fn add_vehicles(&mut self, vs: Vec<Vehicle>) {
        for v in vs {
            self.add_vehicle(v);
        }
    }
    pub fn regulate(&mut self, canvas: &mut WindowCanvas) -> Option<Vec<Vehicle>> {
        let mut res = vec![];
        match self.north.regulate(canvas) {
            Some(mut v) => res.append(&mut v),
            None => {}
        }
        match self.south.regulate(canvas) {
            Some(mut v) => res.append(&mut v),
            None => {}
        };
        match self.west.regulate(canvas) {
            Some(mut v) => res.append(&mut v),
            None => {}
        };
        match self.east.regulate(canvas) {
            Some(mut v) => res.append(&mut v),
            None => {}
        };
        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
struct Branch {
    left: Lane,
    straight: Lane,
    right: Lane,
}

impl Branch {
    fn new() -> Self {
        Branch {
            left: Lane::new(),
            straight: Lane::new(),
            right: Lane::new(),
        }
    }
    fn add_vehicle(&mut self, v: Vehicle) {
        match v.turn {
            Turning::Left => self.left.add_back(v),
            Turning::Straight => self.straight.add_back(v),
            Turning::Right => self.right.add_back(v),
        }
    }
    fn regulate(&mut self, canvas: &mut WindowCanvas) -> Option<Vec<Vehicle>> {
        let mut res = vec![];
        match self.left.regulate(canvas) {
            Some(v) => res.push(v),
            None => {}
        }
        match self.straight.regulate(canvas) {
            Some(v) => res.push(v),
            None => {}
        };
        match self.right.regulate(canvas) {
            Some(v) => res.push(v),
            None => {}
        }
        if res.len() > 0 {
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lane {
    vehicles: VecDeque<Vehicle>,
}

impl Lane {
    fn new() -> Self {
        Lane {
            vehicles: VecDeque::new(),
        }
    }
    fn regulate(&mut self, canvas: &mut WindowCanvas) -> Option<Vehicle> {
        for ix in 0..self.vehicles.len() {
            if ix != 0 {
                if self.vehicles[ix].is_safe_distance(self.vehicles[ix - 1]) {
                    self.vehicles[ix].drive();
                }
                if self.vehicles[ix].speed <= self.vehicles[ix - 1].speed {
                    self.vehicles[ix].accelerate();
                } else {
                    self.vehicles[ix].deaccelerate();
                }
            } else {
                if self.vehicles[ix].speed != Speed::High {
                    self.vehicles[ix].accelerate()
                }
                self.vehicles[ix].drive();
            }
            self.vehicles[ix].render(canvas);
        }
        if self.vehicles.len() > 0 && self.vehicles[0].is_out() {
            self.drop_vehicle();
        }
        if self.vehicles.len() > 0 && self.vehicles[0].in_intersection() {
            self.drop_vehicle()
        } else {
            None
        }
    }
    fn add_back(&mut self, v: Vehicle) {
        self.vehicles.push_back(v);
    }
    fn drop_vehicle(&mut self) -> Option<Vehicle> {
        self.vehicles.pop_front()
    }
}
