use crate::geometry::Vec3;
use crate::planet::{planet_position, PLANET_COUNT};
use crate::travel;

#[derive(Debug)]
pub struct Answer {
    src: Vec3,
    reachables: [bool; PLANET_COUNT],
}

impl Answer {
    pub fn is_reachable(&self, id: usize) -> bool {
        self.reachables[id]
    }

    /// 目的地の惑星および移動力を与えたときの (実所要ターン数, 実所要エネルギー) を返す。
    /// 到達不能なら `None` を返す。
    pub fn cost(&self, id: usize, speed: u32) -> Option<(u32, u32)> {
        self.reachables[id]
            .then(|| travel::actual_turns_and_energy(self.src, planet_position(id), speed))
    }
}

#[derive(Debug)]
pub enum Query {
    Planet(QueryPlanet),
    Position(QueryPosition),
}

impl Query {
    pub fn new_planet(src: usize, energy: u32) -> Self {
        Self::Planet(QueryPlanet::new(src, energy))
    }

    pub fn default_position() -> Self {
        Self::Position(Default::default())
    }

    pub fn src_pos(&self) -> Vec3 {
        match self {
            Self::Planet(q) => planet_position(q.src()),
            Self::Position(q) => q.src(),
        }
    }

    pub fn execute(&self) -> Answer {
        let (src, reachables) = match self {
            Self::Planet(q) => {
                let reachables = travel::reachable_planets(q.src(), q.energy());
                (planet_position(q.src()), reachables)
            }
            Self::Position(q) => {
                let reachables = [true; PLANET_COUNT];
                (q.src(), reachables)
            }
        };

        Answer { src, reachables }
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::Planet(Default::default())
    }
}

#[derive(Debug)]
pub struct QueryPlanet {
    src: usize,
    energy: u32,
}

impl QueryPlanet {
    pub fn new(src: usize, energy: u32) -> Self {
        Self { src, energy }
    }

    pub fn src(&self) -> usize {
        self.src
    }

    pub fn set_src(&mut self, src: usize) {
        self.src = src;
    }

    pub fn energy(&self) -> u32 {
        self.energy
    }

    pub fn set_energy(&mut self, energy: u32) {
        self.energy = energy;
    }
}

impl Default for QueryPlanet {
    fn default() -> Self {
        Self {
            src: 0,
            energy: 100,
        }
    }
}

#[derive(Debug)]
pub struct QueryPosition(Vec3);

impl QueryPosition {
    pub fn src(&self) -> Vec3 {
        self.0
    }

    pub fn set_src_x(&mut self, x: u32) {
        self.0.x = x;
    }

    pub fn set_src_y(&mut self, y: u32) {
        self.0.y = y;
    }

    pub fn set_src_z(&mut self, z: u32) {
        self.0.z = z;
    }
}

impl Default for QueryPosition {
    fn default() -> Self {
        Self(Vec3::new(8, 8, 8))
    }
}
