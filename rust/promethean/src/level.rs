#[path = "point.rs"] pub mod point;
#[path = "options.rs"] pub mod options;

use std::fmt;
use js_sys::Array;

use point::Point;
use options::Tile;

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct LevelStatistics {
    init: bool,
    rooms_count: usize,
    corridors_count: usize,
    all_corridors: bool,
    room_centers: Vec<Point>
}

impl LevelStatistics {
    pub fn new() -> LevelStatistics {
        return LevelStatistics {
            init: false,
            rooms_count: 0,
            corridors_count: 0,
            all_corridors: false,
            room_centers: Vec::new()
        };
    }

    pub fn set(&mut self, 
        rooms_count: usize, 
        corridors_count: usize, 
        all_corridors: bool, 
        room_centers: Vec<Point>) {
        self.init = true;
        self.rooms_count = rooms_count;
        self.corridors_count = corridors_count;
        self.all_corridors = all_corridors;
        self.room_centers = room_centers;
    }

    pub fn room_centers_inner(&self) -> &Vec<Point> {
        return &self.room_centers;
    }
}

#[wasm_bindgen]
impl LevelStatistics {
    #[wasm_bindgen(getter)]
    pub fn rooms_count(&self) -> usize {
        return self.rooms_count;
    }

    #[wasm_bindgen(getter)]
    pub fn corridors_count(&self) -> usize {
        return self.corridors_count;
    }

    #[wasm_bindgen(getter)]
    pub fn all_corridors(&self) -> bool {
        return self.all_corridors;
    }

    #[wasm_bindgen(getter)]
    pub fn room_centers(&self) -> Array {
        return self.room_centers.clone().into_iter().map(JsValue::from).collect();
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Level {
    height: usize,
    width: usize,
    level: Vec<Vec<Tile>>,
    statistics: LevelStatistics
}

impl Level {
    pub fn set_statistics(&mut self, rooms_count: usize, 
                                     corridors_count: usize, 
                                     all_corridors: bool, 
                                     room_centers: Vec<Point>) {
        self.statistics.set(rooms_count, corridors_count, all_corridors, room_centers);
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        self.level[x][y] = tile;
    }

    pub fn get_from_coordinates(&self, x: usize, y: usize) -> Tile {
        return self.level[x][y];
    }

    pub fn set_from_point(&mut self, point: &Point, value: Tile) {
        self.set_tile(point.x() as usize, point.y() as usize, value);
    }

    pub fn inflate(&mut self, inflation_factor: usize) {
        let mut inflated_matrix: Vec<Vec<Tile>> = Vec::with_capacity(inflation_factor * self.height);
        for _x in 0..(inflation_factor * self.height) {
            let mut arr_x: Vec<Tile> = Vec::with_capacity(inflation_factor * self.width);
            for _y in 0..(inflation_factor * self.width) {
                arr_x.push(Tile::Empty);
            }
            inflated_matrix.push(arr_x);
        }

        for row in 0..self.height {
            for column in 0..self.width {
                for xr in 0..inflation_factor {
                    for yr in 0..inflation_factor {
                        inflated_matrix[row * inflation_factor + xr][column * inflation_factor + yr] = self.level[row][column];
                    }
                }
            }
        }
        self.level = inflated_matrix;
        self.width = inflation_factor * self.width;
        self.height = inflation_factor * self.height;
    }

    pub fn render_inner(&self) -> &Vec<Vec<Tile>> {
        return &self.level;
    }
}

#[wasm_bindgen]
impl Level {
    #[wasm_bindgen(constructor)]
    pub fn new(height: usize, width: usize) -> Level {
        let mut level: Vec<Vec<Tile>> = Vec::with_capacity(height);
        for _x in 0..height {
            let mut row: Vec<Tile> = Vec::with_capacity(width);
            for _y in 0..width {
                row.push(Tile::Empty);
            }
            level.push(row);
        }

        return Level {
            height,
            width,
            level,
            statistics: LevelStatistics::new()
        };
    }

    #[wasm_bindgen]
    pub fn statistics(&self) -> LevelStatistics {
        return self.statistics.clone();
    }

    #[wasm_bindgen]
    pub fn height(&self) -> usize {
        return self.height;
    }

    #[wasm_bindgen]
    pub fn width(&self) -> usize {
        return self.width;
    }

    #[wasm_bindgen]
    pub fn render(&self) -> Array {
        let mut to_return: Vec<u8> = Vec::with_capacity(self.width * self.height);
        for x in 0..self.height {
            for y in 0..self.width {
                let v: &Tile = &self.level[x][y];
                to_return[x * self.width + y] = *v as u8;
            }
        }
        return to_return.into_iter().map(JsValue::from).collect();
    }
}

impl fmt::Display for Level{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut to_return = String::new();
        for x in 0..self.height {
            let mut x_str = String::new();
            for y in 0..self.width {
                x_str.push_str(match self.level[x][y] {
                    Tile::Floor => "□ ",
                    Tile::Empty =>  "■ " ,
                    _ => "● "
                });
            }
            to_return.push_str((x_str + "\n").as_str());
        }

        return write!(f, "{}", to_return);
    }
}
