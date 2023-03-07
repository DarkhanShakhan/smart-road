use rand::{Rand, Rng};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

// vehicle
#[derive(Clone, Debug, Copy)]
pub struct Vehicle {
    pub position: Position,
    pub turn: Turning,
    pub direction: Direction,
    pub speed: Speed,
    pub environment: Environment,
    pub pivot: Pivot,
}

const SAFE_DISTANCE: i32 = 10;

impl Vehicle {
    pub fn new(w: u32, h: u32, turn: Turning, direction: Direction, speed: Speed) -> Self {
        Vehicle {
            position: Position::new(w, h, turn, direction),
            turn,
            direction,
            speed,
            environment: Environment::new(w as i32, h as i32),
            pivot: Pivot::new(Environment::new(w as i32, h as i32), direction, turn),
        }
    }
    pub fn accelerate(&mut self) {
        match self.speed {
            Speed::No => self.speed = Speed::Low,
            Speed::Low => self.speed = Speed::Normal,
            Speed::Normal => self.speed = Speed::High,
            Speed::High => {}
        }
    }
    pub fn deaccelerate(&mut self) {
        match self.speed {
            Speed::No => {}
            Speed::Low => self.speed = Speed::No,
            Speed::Normal => self.speed = Speed::Low,
            Speed::High => self.speed = Speed::Normal,
        }
    }

    pub fn drive(&mut self) {
        match self.direction {
            Direction::North => self.position.y -= self.speed as i32,
            Direction::South => self.position.y += self.speed as i32,
            Direction::East => self.position.x += self.speed as i32,
            Direction::West => self.position.x -= self.speed as i32,
        }
        match self.turn {
            Turning::Straight => {}
            Turning::Left => {
                if self.is_at_pivot() {
                    self.turn = Turning::Straight;
                    self.position = self.pivot.position;
                    match self.direction {
                        Direction::North => {
                            self.direction = Direction::West;
                        }
                        Direction::South => {
                            self.direction = Direction::East;
                        }
                        Direction::East => {
                            self.direction = Direction::North;
                        }
                        Direction::West => {
                            self.direction = Direction::South;
                        }
                    }
                }
            }
            Turning::Right => {
                if self.is_at_pivot() {
                    self.turn = Turning::Straight;
                    self.position = self.pivot.position;
                    match self.direction {
                        Direction::North => {
                            self.direction = Direction::East;
                        }
                        Direction::South => {
                            self.direction = Direction::West;
                        }
                        Direction::East => {
                            self.direction = Direction::South;
                        }
                        Direction::West => {
                            self.direction = Direction::North;
                        }
                    }
                }
            }
        }
    }
    pub fn is_at_pivot(self) -> bool {
        match self.pivot.over {
            true => {
                self.position.x >= self.pivot.position.x && self.position.y >= self.pivot.position.y
            }
            false => {
                self.position.x <= self.pivot.position.x && self.position.y <= self.pivot.position.y
            }
        }
    }

    pub fn is_safe_distance(self, previous: Vehicle) -> bool {
        match self.direction {
            Direction::North => {
                self.position.y - previous.position.y - 30 - self.speed as i32 > SAFE_DISTANCE
            }
            Direction::South => {
                previous.position.y - self.position.y - 30 - self.speed as i32 > SAFE_DISTANCE
            }
            Direction::West => {
                self.position.x - previous.position.x - 30 - self.speed as i32 > SAFE_DISTANCE
            }
            Direction::East => {
                previous.position.x - self.position.x - 30 - self.speed as i32 > SAFE_DISTANCE
            }
        }
    }
    pub fn is_out(self) -> bool {
        self.position.x > self.environment.width
            || self.position.x < -20
            || self.position.y > self.environment.height
            || self.position.y < -20
    }

    pub fn in_intersection(self) -> bool {
        self.position.x > (self.environment.center.x - 120)
            && self.position.x < (self.environment.center.x + 100)
            && self.position.y > (self.environment.center.y - 120)
            && self.position.y < (self.environment.center.y + 100)
    }
    pub fn render(&mut self, canvas: &mut WindowCanvas) {
        let rect = Rect::new(self.position.x, self.position.y, 20, 20);
        canvas.set_draw_color(Color::GREEN);
        canvas.fill_rect(rect).unwrap();
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Pivot {
    position: Position,
    over: bool,
}

impl Pivot {
    pub fn new(env: Environment, dir: Direction, turn: Turning) -> Self {
        let mut pos = env.center;
        let mut over = true;
        match turn {
            Turning::Straight => {}
            Turning::Right => match dir {
                Direction::North => {
                    pos = Position {
                        x: (env.center.x + 40),
                        y: (env.center.y + 40),
                    };
                    over = false;
                }
                Direction::South => {
                    pos = Position {
                        x: (env.center.x - 60),
                        y: (env.center.y - 60),
                    };
                    over = true;
                }
                Direction::West => {
                    pos = Position {
                        x: (env.center.x + 40),
                        y: (env.center.y - 60),
                    };
                    over = false;
                }
                Direction::East => {
                    pos = Position {
                        x: (env.center.x - 60),
                        y: (env.center.y + 40),
                    };
                    over = true;
                }
            },
            Turning::Left => match dir {
                Direction::North => {
                    pos = Position {
                        x: (env.center.x),
                        y: (env.center.y -20),
                    };
                    over = false;
                }
                Direction::South => {
                    pos = Position {
                        x: (env.center.x - 20),
                        y: (env.center.y),
                    };
                    over = true;
                }
                Direction::West => {
                    pos = Position {
                        x: (env.center.x -20),
                        y: (env.center.y - 20),
                    };
                    over = false;
                }
                Direction::East => {
                    pos = Position {
                        x: (env.center.x ),
                        y: (env.center.y ),
                    };
                    over = true;
                }
            },
        }
        Pivot {
            position: pos,
            over,
        }
    }
}

// turning

#[derive(Clone, Copy, Debug)]
pub enum Turning {
    Left,
    Right,
    Straight,
}

impl Rand for Turning {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => Turning::Left,
            1 => Turning::Right,
            _ => Turning::Straight,
        }
    }
}

// direction

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Rand for Direction {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 4) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            _ => Direction::East,
        }
    }
}

//speed
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Speed {
    No = 0,
    Low = 5,
    Normal = 10,
    High = 15,
}

impl Rand for Speed {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        match rng.gen_range(0, 3) {
            0 => Speed::Low,
            1 => Speed::Normal,
            _ => Speed::High,
        }
    }
}

//position
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(w: u32, h: u32, turn: Turning, dir: Direction) -> Self {
        let mut n = 0;
        match turn {
            Turning::Left => {}
            Turning::Right => n += 40,
            Turning::Straight => n += 20,
        }
        match dir {
            Direction::North => Position {
                x: (w as i32 / 2 + n),
                y: (h as i32),
            },
            Direction::West => Position {
                x: (w as i32),
                y: (h as i32 / 2 - 20 - n),
            },
            Direction::South => Position {
                x: (w as i32 / 2 - 20 - n),
                y: (-20),
            },
            Direction::East => Position {
                x: (-20),
                y: (w as i32 / 2 + n),
            },
        }
    }
}

// environment
#[derive(Clone, Copy, Debug)]
pub struct Environment {
    pub width: i32,
    pub height: i32,
    center: Position,
}

impl Environment {
    pub fn new(width: i32, height: i32) -> Environment {
        Environment {
            width,
            height,
            center: Position {
                x: width / 2,
                y: height / 2,
            },
        }
    }
}
