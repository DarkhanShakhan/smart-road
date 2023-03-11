use super::Vehicle;
use sdl2::render::WindowCanvas;
use std::collections::{HashMap, VecDeque};

//TODO: safe distance in intersection
pub struct Intersection {
    pub vehicles: VecDeque<InstructedVehicle>,
    pub moves: Moves,
}

impl Intersection {
    pub fn new() -> Self {
        Intersection {
            vehicles: VecDeque::new(),
            moves: Moves::new(),
        }
    }
    pub fn add_vehicles(&mut self, cands: Vec<Vehicle>) {
        for v in cands {
            let instrs = self.instruct_vehicle(&v);
            self.vehicles.push_back(InstructedVehicle::new(v, instrs));
        }
    }
    pub fn instruct_vehicle(&mut self, v: &Vehicle) -> VecDeque<Instruction> {
        let mut algo = Algorithm::new();
        let mut res = algo.algorithm(&self.moves, v, VecDeque::new());
        let mut sim_v = v.clone();
        let mut ix = 0;
        let center = v.environment.center;
        while sim_v.in_intersection() {
            let mut x = (sim_v.position.x - (center.x - 60)) / 2;
            let mut y = (sim_v.position.y - (center.y - 60)) / 2;
            if ix >= self.moves.states.len() {
                self.moves.add_state();
            }
            if x < 50 && y < 50 && x > -10 && y > -10 {
                if x < 0 {
                    x = 0
                }
                if y < 0 {
                    y = 0
                }
                let (mut xs, mut ys) = (vec![x / 10], vec![y / 10]);
                if x % 10 != 0 {
                    xs.push((x / 10) + 1);
                }
                if y % 10 != 0 {
                    ys.push((y / 10) + 1);
                }
                for a in xs {
                    for b in &ys {
                        self.moves.states[ix].occupy(a as usize, *b as usize);
                    }
                }
            }
            if ix >= res.len() {
                if sim_v.speed != super::Speed::High {
                    res.push_back(Instruction::Accelerate);
                    sim_v.accelerate();
                } else {
                    res.push_back(Instruction::Still);
                }
                sim_v.drive();
                continue;
            }
            match res[ix] {
                Instruction::Accelerate => sim_v.accelerate(),
                Instruction::Deaccelerate => sim_v.deaccelerate(),
                Instruction::Still => {}
            }
            sim_v.drive();
            ix += 1;
        }
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

#[derive(Clone, Debug)]
pub struct Moves {
    states: VecDeque<State>,
}

impl Moves {
    pub fn new() -> Self {
        Moves {
            states: VecDeque::new(),
        }
    }
    pub fn add_state(&mut self) {
        self.states.push_back(State::new())
    }
    pub fn drop_state(&mut self) {
        self.states.pop_front();
    }
}

#[derive(Clone, Debug)]
pub struct State {
    board: Vec<Vec<bool>>,
}

impl State {
    pub fn new() -> Self {
        let line = vec![false; 6];
        State {
            board: vec![line; 6],
        }
    }
    pub fn is_occupied(&mut self, x: usize, y: usize) -> bool {
        self.board[x][y]
    }

    pub fn occupy(&mut self, x: usize, y: usize) {
        if x > 5 || y > 5 {
            return;
        }
        self.board[x][y] = true
    }
}
// pub fn route(v: &Vehicle) -> Vec<(usize, usize)> {
//     let mut res = vec![];
//     match v.direction {
//         super::Direction::North => match v.turn {
//             Turning::Right => res.push((5, 5)),
//             Turning::Straight => {
//                 for ix in 0..5 {
//                     res.push((4, ix));
//                 }
//             }
//             Turning::Left => {
//                 for ix in 2..5 {
//                     res.push((3, ix));
//                 }
//                 for ix in 0..2 {
//                     res.push((ix, 2));
//                 }
//             }
//         },
//         super::Direction::South => match v.turn {
//             Turning::Right => res.push((0, 0)),
//             Turning::Straight => {
//                 for ix in 0..5 {
//                     res.push((1, ix));
//                 }
//             }
//             Turning::Left => {
//                 for ix in 0..3 {
//                     res.push((2, ix));
//                 }
//                 for ix in 3..5 {
//                     res.push((ix, 3))
//                 }
//             }
//         },
//         _ => {}
//     }
//     res
// }

pub struct Algorithm {
    visited: HashMap<String, VecDeque<Instruction>>,
}

impl Algorithm {
    pub fn new() -> Self {
        Algorithm {
            visited: HashMap::new(),
        }
    }

    pub fn algorithm(
        &mut self,
        moves: &Moves,
        v: &Vehicle,
        instr: VecDeque<Instruction>,
    ) -> VecDeque<Instruction> {
        if !v.in_intersection() || moves.states.len() == 0 {
            return instr;
        }
        let mut algo = (0, 0, 0);
        for s in &instr {
            match *s {
                Instruction::Accelerate => algo.0 += 1,
                Instruction::Still => algo.1 += 1,
                Instruction::Deaccelerate => algo.2 += 1,
            }
        }
        let key = format!("{}:{}:{}", algo.0, algo.1, algo.2);
        if self.visited.contains_key(&key) {
            let mut res = VecDeque::new();
            match self.visited.get(&key) {
                Some(v) => res = v.clone(),
                None => {}
            }
            return res;
        }
        let center = v.environment.center;
        let mut x = (v.position.x - (center.x - 60)) / 2;
        let mut y = (v.position.y - (center.y - 60)) / 2;
        if x >= 50 || y >= 50 || x < -10 || y < -10 {
            let mut sim_v1 = v.clone();
            let mut m1 = moves.clone();
            let mut instr1 = instr.clone();
            let mut res = VecDeque::new();
            if v.speed != super::Speed::High {
                sim_v1.accelerate();
                sim_v1.drive();
                m1.drop_state();
                instr1.push_back(Instruction::Accelerate);
                res = self.algorithm(&m1, &sim_v1, instr1);
                if res.len() > 0 {
                    self.visited.insert(key, res.clone());
                    return res;
                }
            }
            if v.speed != super::Speed::No {
                sim_v1 = v.clone();
                m1 = moves.clone();
                instr1 = instr.clone();
                sim_v1.deaccelerate();
                sim_v1.drive();
                m1.drop_state();
                instr1.push_back(Instruction::Deaccelerate);
                res = self.algorithm(&m1, &sim_v1, instr1);
                if res.len() > 0 {
                    self.visited.insert(key, res.clone());
                    return res;
                }
            }
            sim_v1 = v.clone();
            m1 = moves.clone();
            instr1 = instr.clone();
            sim_v1.drive();
            m1.drop_state();
            instr1.push_back(Instruction::Still);
            res = self.algorithm(&m1, &sim_v1, instr1);
            if res.len() > 0 {
                self.visited.insert(key, res.clone());
                return res;
            }
            self.visited.insert(key, res.clone());
            return res;
        }
        if x < 0 {
            x = 0
        }
        if y < 0 {
            y = 0
        }
        let (mut xs, mut ys) = (vec![x / 10], vec![y / 10]);
        let mut sim_moves = moves.clone();
        if x % 10 != 0 {
            xs.push((x / 10) + 1);
        }
        if y % 10 != 0 {
            ys.push((y / 10) + 1);
        }
        for a in xs {
            for b in &ys {
                let ok = sim_moves.states[0].is_occupied(a as usize, *b as usize);
                if ok {
                    return VecDeque::new();
                }
                if sim_moves.states.len() > 1 {
                    let ok = sim_moves.states[1].is_occupied(a as usize, *b as usize);
                    if ok {
                        return VecDeque::new();
                    }
                }
                if sim_moves.states.len() > 2 {
                    let ok = sim_moves.states[2].is_occupied(a as usize, *b as usize);
                    if ok {
                        return VecDeque::new();
                    }
                }
                if sim_moves.states.len() > 3 {
                    let ok = sim_moves.states[3].is_occupied(a as usize, *b as usize);
                    if ok {
                        return VecDeque::new();
                    }
                }
                // sim_moves.states[0].occupy(a as usize, *b as usize);
            }
        }
        let mut sim_v1 = v.clone();
        let mut m1 = moves.clone();
        let mut instr1 = instr.clone();
        let mut res = VecDeque::new();
        if v.speed != super::Speed::High {
            sim_v1.accelerate();
            sim_v1.drive();
            m1.drop_state();
            instr1.push_back(Instruction::Accelerate);
            res = self.algorithm(&m1, &sim_v1, instr1);
            if res.len() > 0 {
                self.visited.insert(key, res.clone());
                return res;
            }
        }
        if v.speed != super::Speed::No {
            sim_v1 = v.clone();
            m1 = moves.clone();
            instr1 = instr.clone();
            sim_v1.deaccelerate();
            sim_v1.drive();
            m1.drop_state();
            instr1.push_back(Instruction::Deaccelerate);
            res = self.algorithm(&m1, &sim_v1, instr1);
            if res.len() > 0 {
                self.visited.insert(key, res.clone());
                return res;
            }
        }
        sim_v1 = v.clone();
        m1 = moves.clone();
        instr1 = instr.clone();
        sim_v1.drive();
        m1.drop_state();
        instr1.push_back(Instruction::Still);
        res = self.algorithm(&m1, &sim_v1, instr1);
        if res.len() > 0 {
            self.visited.insert(key, res.clone());
            return res;
        }
        self.visited.insert(key, res.clone());
        res
    }
}
