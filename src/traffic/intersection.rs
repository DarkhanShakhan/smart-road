use super::Vehicle;
use sdl2::render::WindowCanvas;
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
        println!("Adds {} vehicles", cands.len());
        for v in cands {
            let instrs = self.instruct_vehicle(&v);
            self.vehicles.push_back(InstructedVehicle::new(v, instrs));
        }
    }
    pub fn instruct_vehicle(&mut self, v: &Vehicle) -> VecDeque<Instruction> {
        let mut res = VecDeque::new();
        //TODO: program an algorithm
        res
    }
    pub fn regulate(&mut self, canvas: &mut WindowCanvas) -> Option<Vec<Vehicle>> {
        let mut list = vec![];
        for ix in 0..self.vehicles.len() {
            self.vehicles[ix].follow_instruction(canvas);
            if self.vehicles[ix].is_empty_instructions() {
                list.push(ix)
            }
        }
        list.reverse();
        let mut result = vec![];
        for jx in list {
            result.push(self.vehicles[jx].vehicle);
            self.vehicles.remove(jx);
        }
        if result.len() != 0 {
            println!("Out vehicles {}", result.len());
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct InstructedVehicle {
    pub vehicle: Vehicle,
    pub instructions: VecDeque<Instruction>,
}

impl InstructedVehicle {
    pub fn new(v: Vehicle, instrs: VecDeque<Instruction>) -> Self {
        InstructedVehicle {
            vehicle: v,
            instructions: instrs,
        }
    }
    pub fn follow_instruction(&mut self, canvas: &mut WindowCanvas) {
        match self.instructions[0] {
            Instruction::Still => {}
            Instruction::Deaccelerate => self.vehicle.deaccelerate(),
            Instruction::Accelerate => self.vehicle.accelerate(),
        }
        self.vehicle.drive();
        self.vehicle.render(canvas);
        self.instructions.pop_front();
    }
    pub fn is_empty_instructions(&self) -> bool {
        self.instructions.len() == 0
    }
}
#[derive(Clone, Debug)]
pub enum Instruction {
    Deaccelerate,
    Still,
    Accelerate,
}
