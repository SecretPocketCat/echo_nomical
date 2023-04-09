use rltk::{BaseMap, Point};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new(width: i32, height: i32, mut f: impl FnMut(i32, i32) -> TileType) -> Self {
        let mut map = Self {
            tiles: vec![],
            width,
            height,
        };

        for y in 0..height {
            for x in 0..width {
                map.tiles.push(f(x, y));
            }
        }
        map
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    pub fn xy(&self, x: i32, y: i32) -> &TileType {
        &self.tiles[self.xy_idx(x, y)]
    }

    pub fn xy_mut(&mut self, x: i32, y: i32) -> &mut TileType {
        let idx = self.xy_idx(x, y);
        &mut self.tiles[idx]
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let (x, y) = self.idx_xy(idx);
        NEIGHBORS.iter().for_each(|&[dx, dy]| {
            let delta = if dx == 0 || dy == 0 { 1.0f32 } else { 1.45f32 };
            let (x, y) = (x + dx, y + dy);
            if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                return;
            }
            exits.push((self.xy_idx(x, y), delta));
        });
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let [p1, p2] = [idx1, idx2]
            .map(|i| self.idx_xy(i))
            .map(|(x, y)| rltk::Point::new(x, y));
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

const NEIGHBORS: [[i32; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

pub fn gen_map() -> Map {
    let mut rng = rltk::RandomNumberGenerator::new();
    let mut map = Map::new(24, 14, move |_x, _y| {
        let roll = rng.roll_dice(1, 100);
        if roll > 55 {
            TileType::Floor
        } else {
            TileType::Wall
        }
    });

    for _ in 0..15 {
        let mut newmap = map.clone();

        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                let neighbors: u8 = NEIGHBORS
                    .map(|[dx, dy]| map.xy(x + dx, y + dy) == &TileType::Wall)
                    .map(|i| i as u8)
                    .iter()
                    .sum();

                *newmap.xy_mut(x, y) = if neighbors > 4 || neighbors == 0 {
                    TileType::Wall
                } else {
                    TileType::Floor
                };
            }
        }

        map = newmap;
    }

    for y in 0..map.height {
        *map.xy_mut(0, y) = TileType::Wall;
        *map.xy_mut(map.width - 1, y) = TileType::Wall;
    }
    for x in 0..map.width {
        *map.xy_mut(x, 0) = TileType::Wall;
        *map.xy_mut(x, map.height - 1) = TileType::Wall;
    }

    map
}
