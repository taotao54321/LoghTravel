#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Vec3 {
    pub const fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn distance(self, other: Self) -> u32 {
        num_integer::sqrt(self.distance_squared(other))
    }

    fn distance_squared(self, other: Self) -> u32 {
        let dx2 = self.x.abs_diff(other.x).pow(2);
        let dy2 = self.y.abs_diff(other.y).pow(2);
        let dz2 = self.z.abs_diff(other.z).pow(2);

        dx2 + dy2 + dz2
    }
}
