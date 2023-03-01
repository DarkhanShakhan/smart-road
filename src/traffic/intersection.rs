use super::Vehicle;
use std::collections::VecDeque;

pub struct Intersection {
    pub vehicles: VecDeque<InstructedVehicle>,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            vehicles: VecDeque::new(),
        }
    }
    pub fn add_vehicles(&mut self, cands: Vec<Vehicle>) {
        for v in cands {
            
            self.vehicles.push_back(v);
        }
    }
}

pub struct InstructedVehicle {
    pub vehicle:Vehicle,
    pub instructions:VecDeque<Instruction>
}

pub enum Instruction {
    Deaccelerate,
    Still,
    Accelerate,
}