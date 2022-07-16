use crate::geometry::Vec3;
use crate::planet::{planet_distance, planet_neighbors, PLANET_COUNT};

/// 現在地の惑星とエネルギーを与えたときに移動命令を出せる惑星の集合を返す。
/// 結果は `bool` 配列として返す。
pub fn reachable_planets(src: usize, energy: u32) -> [bool; PLANET_COUNT] {
    #[derive(Debug)]
    struct Dfs {
        src: usize,
        energy: u32,
        reachables: [bool; PLANET_COUNT],
    }
    impl Dfs {
        fn new(src: usize, energy: u32) -> Self {
            Self {
                src,
                energy,
                reachables: [false; PLANET_COUNT],
            }
        }
        fn solve(&mut self) {
            self.reachables[self.src] = true;
            self.dfs(self.src);
        }
        fn dfs(&mut self, id: usize) {
            for &dst in planet_neighbors(id) {
                if self.reachables[dst] {
                    continue;
                }
                if self.energy > planet_distance(self.src, dst) {
                    self.reachables[dst] = true;
                    self.dfs(dst);
                }
            }
        }
    }

    let mut dfs = Dfs::new(src, energy);
    dfs.solve();

    dfs.reachables
}

/// 艦隊の現在地、目的地、移動力を与えたときの到着までの (実所要ターン数, 実所要エネルギー) を返す。
///
/// `src == target` の場合、`(0, 0)` を返す。
pub fn actual_turns_and_energy(src: Vec3, target: Vec3, speed: u32) -> (u32, u32) {
    // 愚直にシミュレートする。

    let mut turn = 0;
    let mut energy = 0;
    let mut p = src;

    while p != target {
        let p_nxt = move_fleet(p, target, speed);
        turn += 1;
        energy += p.distance(p_nxt);
        p = p_nxt;
    }

    (turn, energy)
}

/// 艦隊の現在地、目的地、移動力を与えたときの現ターンの移動先を返す。
fn move_fleet(src: Vec3, target: Vec3, speed: u32) -> Vec3 {
    let dist_to_target = src.distance(target);

    // 現ターンで目的地に到達できるなら、目的地の座標を返す。
    if speed >= dist_to_target {
        return target;
    }

    // 到達までの所要ターン数を求める。
    let turns_needed = num_integer::div_ceil(dist_to_target, speed);

    // x, y, z 各方向について、残り移動距離を所要ターン数で割った値を移動量とする。

    let dx = src.x.abs_diff(target.x) / turns_needed;
    let dy = src.y.abs_diff(target.y) / turns_needed;
    let dz = src.z.abs_diff(target.z) / turns_needed;

    fn f(src_x: u32, target_x: u32, dx: u32) -> u32 {
        if src_x <= target_x {
            src_x + dx
        } else {
            src_x - dx
        }
    }

    let x = f(src.x, target.x, dx);
    let y = f(src.y, target.y, dy);
    let z = f(src.z, target.z, dz);

    Vec3::new(x, y, z)
}
