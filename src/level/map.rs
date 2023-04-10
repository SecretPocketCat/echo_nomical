use bevy::prelude::{IVec2, Resource};
use bracket_algorithm_traits::prelude::{BaseMap, SmallVec};
use bracket_geometry::prelude::{DistanceAlg, Point};
use bracket_pathfinding::prelude::DijkstraMap;

use crate::enemy::EnemyType;

// A lot of things here taken from https://bfnightly.bracketproductions.com/rustbook/chapter_27.html
// and the rest of the Rust Roguelike Tutorial.

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Goal,
    PlayerSpawn,
    Enemy(EnemyType),
}

#[derive(Default, Clone, Resource)]
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
        (y as usize)
            .wrapping_mul(self.width as usize)
            .wrapping_add(x as usize)
    }

    pub fn idx_xy(&self, idx: usize) -> (i32, i32) {
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        (x, y)
    }

    pub fn xy(&self, x: i32, y: i32) -> &TileType {
        &self.tiles[self.xy_idx(x, y) % self.tiles.len()]
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

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let (x, y) = self.idx_xy(idx);
        ADJACENTS.iter().for_each(|&[dx, dy]| {
            let (x, y) = (x + dx, y + dy);
            if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
                return;
            }
            if self.xy(x, y) == &TileType::Wall {
                return;
            }
            exits.push((self.xy_idx(x, y), 1.0f32));
        });
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let [p1, p2] = [idx1, idx2]
            .map(|i| self.idx_xy(i))
            .map(|(x, y)| Point::new(x, y));
        DistanceAlg::Pythagoras.distance2d(p1, p2)
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
const ADJACENTS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

pub fn generate(width: i32, height: i32) -> Option<Map> {
    let mut map = Map::new(width, height, move |_x, _y| {
        if rand::random::<f32>() > 0.55 {
            TileType::Floor
        } else {
            TileType::Wall
        }
    });

    // Cellular automata
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

    // Make sure entities are trapped in the level
    for y in 0..map.height {
        *map.xy_mut(0, y) = TileType::Wall;
        *map.xy_mut(map.width - 1, y) = TileType::Wall;
    }
    for x in 0..map.width {
        *map.xy_mut(x, 0) = TileType::Wall;
        *map.xy_mut(x, map.height - 1) = TileType::Wall;
    }

    // Make sure everything is accessible
    let mut start_pos = IVec2::new(map.width / 2, map.height / 2);
    while map.xy(start_pos.x, start_pos.y) != &TileType::Floor {
        if start_pos.x == 0 {
            return None;
        }
        start_pos.x -= 1;
    }

    let start_idx = map.xy_idx(start_pos.x, start_pos.y);
    let dijkstra_map = DijkstraMap::new(
        map.width as usize,
        map.height as usize,
        &vec![start_idx],
        &map,
        200.0,
    );
    for (i, tile) in map.tiles.iter_mut().enumerate() {
        if *tile == TileType::Floor {
            let distance_to_start = dijkstra_map.map[i];
            if distance_to_start == std::f32::MAX {
                *tile = TileType::Wall;
            }
        }
    }

    // Set player spawns
    map.tiles[start_idx] = TileType::PlayerSpawn;

    // Set enemy spawns
    for y in 1..map.height - 1 {
        for x in 1..map.width - 1 {
            // First, the static ones
            if map.xy(x, y) == &TileType::Wall
                && ADJACENTS
                    .iter()
                    .all(|[dx, dy]| map.xy(x + dx, y + dy) == &TileType::Floor)
                && rand::random::<f32>() > 0.5
            {
                *map.xy_mut(x, y) = TileType::Enemy(EnemyType::Spiky);
            }

            // Next, have a 5% chance of spawning following ones.
            if map.xy(x, y) == &TileType::Floor && rand::random::<f32>() > 0.95 {
                let idx = map.xy_idx(x, y);
                if map.get_pathing_distance(idx, start_idx) > 2.0 {
                    *map.xy_mut(x, y) = TileType::Enemy(EnemyType::FollowPing);
                }
            }
        }
    }

    // Set goal spawn
    let mut idx = 0usize;
    for _ in 0..100 {
        idx = rand::random::<usize>() % map.tiles.len();
        if map.tiles[idx] == TileType::Floor
            && map.get_pathing_distance(idx, start_idx) > width as f32 * 0.3f32
        {
            break;
        }
    }
    map.tiles[idx] = TileType::Goal;

    Some(map)
}
