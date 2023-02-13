use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

//tiles
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

//map
pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
    // check if entity is in bounds of screen
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x < SCREEN_WIDTH && point.x >= 0 && point.y < SCREEN_HEIGHT && point.y >= 0
    }
    // check if entity can walk on tile
    pub fn can_enter_tile(&self, point: Point) -> bool {
        let tile = map_idx(point.x, point.y);
        self.in_bounds(point) && self.tiles[tile] == TileType::Floor
    }
    // check if requested tile is out of bounds or not
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}

// calculate tile index in a 1D vec
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
