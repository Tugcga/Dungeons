#[path = "preudo_random.rs"] mod preudo_random;
#[path = "room_generator.rs"] mod room_generator;
#[path = "corridor_generator.rs"] mod corridor_generator;

use crate::level::options::{Options, Tile, TileMask, TilePoint, RoomType};
use preudo_random::PseudoRandom;
use room_generator::{Room, RoomGenerator};
use crate::level::point::Point;
use crate::level::Level;
use corridor_generator::{generate_corridors, Corridor};

use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct GridPattern {
    name: String,
    pattern: Vec<Vec<TileMask>>,
    paint_offsets: Vec<TilePoint>
}

impl GridPattern {
    fn new(name: String, pattern: Vec<Vec<TileMask>>, paint_offsets: Vec<TilePoint>) -> GridPattern {
        return GridPattern { name, pattern, paint_offsets };
    }

    pub fn pattern(&self) -> &Vec<Vec<TileMask>> {
        return &self.pattern;
    }

    pub fn paint_offset(&self, index: usize) -> &TilePoint {
        return &self.paint_offsets[index];
    }

    pub fn paint_offsets_length(&self) -> usize {
        return self.paint_offsets.len();
    }
}

#[derive(Debug)]
#[wasm_bindgen]
pub struct LevelGenerator {
    options: Options,
    room_generator: RoomGenerator,
    grid_patterns: Vec<GridPattern>
}

impl LevelGenerator {
    pub fn new_with_options(options: Options) -> LevelGenerator {
        let random = PseudoRandom::new(options.random_seed() as u64);
        let room_generator = RoomGenerator::new(random);

        let top_left = Point::new(-1, -1);
        let top = Point::new(-1, 0);
        let top_right = Point::new(-1, 1);
        let left = Point::new(0, -1);
        let right = Point::new(0, 1);
        let bottom_left = Point::new(1, -1);
        let bottom = Point::new(1, 0);
        let bottom_right = Point::new(1, 1);

        let mut grid_patterns: Vec<GridPattern> = Vec::with_capacity(14);
        grid_patterns.push(GridPattern::new("Top Left Inside Corner".to_string(), 
            vec![vec![TileMask::Open, TileMask::Open, TileMask::Open], 
            vec![TileMask::Open, TileMask::Block, TileMask::Block], 
            vec![TileMask::Wild, TileMask::Block, TileMask::Block]],
            vec![TilePoint::new(top_left.clone(), Tile::TopLeftInsideCorner), 
             TilePoint::new(top.clone(), Tile::TopWall), 
             TilePoint::new(left.clone(), Tile::LeftWall)]));

        grid_patterns.push(GridPattern::new("Top Right Inside Corner".to_string(), 
            vec![vec![TileMask::Open, TileMask::Open, TileMask::Wild], 
            vec![TileMask::Block, TileMask::Block, TileMask::Open], 
            vec![TileMask::Block, TileMask::Block, TileMask::Wild]],
            vec![TilePoint::new(top_right.clone(), Tile::TopRightInsideCorner), 
             TilePoint::new(top.clone(), Tile::TopWall), 
             TilePoint::new(right.clone(), Tile::RightWall)]));

        grid_patterns.push(GridPattern::new("Bottom Left Inside Cornerr".to_string(), 
            vec![vec![TileMask::Open, TileMask::Block, TileMask::Block], 
            vec![TileMask::Open, TileMask::Block, TileMask::Block], 
            vec![TileMask::Wild, TileMask::Open, TileMask::Wild]],
            vec![TilePoint::new(bottom_left.clone(), Tile::BottomLeftInsideCorner), 
             TilePoint::new(bottom.clone(), Tile::BottomWall), 
             TilePoint::new(left.clone(), Tile::LeftWall)]));

        grid_patterns.push(GridPattern::new("Bottom Right Inside Corner".to_string(), 
            vec![vec![TileMask::Block, TileMask::Block, TileMask::Open], 
            vec![TileMask::Block, TileMask::Block, TileMask::Open], 
            vec![TileMask::Wild, TileMask::Open, TileMask::Open]],
            vec![TilePoint::new(bottom_right.clone(), Tile::BottomRightInsideCorner), 
             TilePoint::new(bottom.clone(), Tile::BottomWall), 
             TilePoint::new(right.clone(), Tile::RightWall)]));

        grid_patterns.push(GridPattern::new("Top Wall".to_string(), 
            vec![vec![TileMask::Open, TileMask::Open, TileMask::Open], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block]],
            vec![TilePoint::new(top.clone(), Tile::TopWall)]));

        grid_patterns.push(GridPattern::new("Bottom Wall".to_string(), 
            vec![vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Open, TileMask::Open, TileMask::Open]],
            vec![TilePoint::new(bottom.clone(), Tile::BottomWall)]));

        grid_patterns.push(GridPattern::new("Left Wall".to_string(), 
            vec![vec![TileMask::Open, TileMask::Block, TileMask::Block], 
            vec![TileMask::Open, TileMask::Block, TileMask::Block], 
            vec![TileMask::Open, TileMask::Block, TileMask::Block]],
            vec![TilePoint::new(left.clone(), Tile::LeftWall)]));

        grid_patterns.push(GridPattern::new("Right Wall".to_string(), 
            vec![vec![TileMask::Block, TileMask::Block, TileMask::Open], 
            vec![TileMask::Block, TileMask::Block, TileMask::Open], 
            vec![TileMask::Block, TileMask::Block, TileMask::Open]],
            vec![TilePoint::new(right.clone(), Tile::RightWall)]));

        grid_patterns.push(GridPattern::new("Bottom Left Outside Wall".to_string(), 
            vec![vec![TileMask::Block, TileMask::Open, TileMask::Open], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block]],
            vec![TilePoint::new(top.clone(), Tile::BottomLeftOutsideCorner)]));

        grid_patterns.push(GridPattern::new("Bottom Right Outside Wall".to_string(), 
            vec![vec![TileMask::Open, TileMask::Open, TileMask::Block], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block]],
            vec![TilePoint::new(top.clone(), Tile::BottomRightOutsideCorner)]));

        grid_patterns.push(GridPattern::new("Top Right Outside Wall".to_string(), 
            vec![vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Open, TileMask::Open, TileMask::Block]],
            vec![TilePoint::new(bottom.clone(), Tile::TopRightOutsideCorner)]));

        grid_patterns.push(GridPattern::new("Top Left Outside Wall".to_string(), 
            vec![vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Block, TileMask::Block, TileMask::Block], 
            vec![TileMask::Block, TileMask::Open, TileMask::Open]],
            vec![TilePoint::new(bottom.clone(), Tile::TopLeftOutsideCorner)]));

        grid_patterns.push(GridPattern::new("Top Right Inside For Touching".to_string(), 
            vec![vec![TileMask::Block, TileMask::Open, TileMask::Open], 
            vec![TileMask::Open, TileMask::Block, TileMask::Block], 
            vec![TileMask::Open, TileMask::Block, TileMask::Block]],
            vec![TilePoint::new(top.clone(), Tile::BottomLeftOutsideCorner)]));

        grid_patterns.push(GridPattern::new("Bottom Right Inside For Touching".to_string(), 
            vec![vec![TileMask::Block, TileMask::Block, TileMask::Open], 
            vec![TileMask::Block, TileMask::Block, TileMask::Open], 
            vec![TileMask::Open, TileMask::Open, TileMask::Block]],
            vec![TilePoint::new(bottom.clone(), Tile::TopRightOutsideCorner)]));

        return LevelGenerator{
            options: options,
            room_generator,
            grid_patterns
        };
    }

    fn render_rooms_on_level(&self, level: &mut Level, rooms: &Vec<Room>) {
        for i in 0..rooms.len() {
            let room = &rooms[i];
            let tiles = room.get_tiles();
            for x_offset in 0..room.height() {
                for y_offset in 0..room.width() {
                    let x = room.position().x() as usize + x_offset;
                    let y = room.position().y() as usize + y_offset;
                    level.set_tile(x, y, tiles[x_offset][y_offset]);
                }
            }
        }
    }

    fn render_corridors_on_level(&self, level: &mut Level, corridors: &Vec<Corridor>) {
        for i in 0..corridors.len() {
            let corridor = &corridors[i];
            let points = corridor.get_tiles();
            for pi in 0..points.len() {
                let point = &points[pi];
                level.set_from_point(point, Tile::Floor);
            }
        }
    }

    fn surrounding_area_matches_pattern(&self, level: &Level, position: Point, pattern: &Vec<Vec<TileMask>>) -> bool {
        let start = Point::new(position.x() - 1, position.y() - 1);
        for x in 0..pattern.len() {
            for y in 0..pattern[x].len(){
                let pattern_value = pattern[x][y];
                if pattern_value == TileMask::Wild {
                    continue;
                }

                let mask_value = if pattern_value == TileMask::Open { Tile::Empty } else { Tile::Floor };

                let target_value = level.get_from_coordinates(start.x() as usize + x, start.y() as usize + y);
                if target_value == mask_value {
                    continue;
                }

                if target_value == Tile::Empty && mask_value != Tile::Empty {
                    return false;
                }

                if target_value != Tile::Empty && mask_value == Tile::Empty {
                    return false;
                }
            }
        }
        return true;
    }
}

#[wasm_bindgen]
impl LevelGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(level_width: usize,
               level_height: usize,
               min_room_width: usize,
               max_room_width: usize,
               min_room_height: usize,
               max_room_height: usize,
               number_of_rooms: usize,
               random_seed: usize,
               border: usize,
               room_border: usize,
               room_square: bool,
               room_rect: bool,
               room_cross: bool,
               room_diamond: bool) -> LevelGenerator {
        let mut room_types: Vec<RoomType> = Vec::with_capacity(4);
        if room_square {
            room_types.push(RoomType::Square);
        }
        if room_rect {
            room_types.push(RoomType::Rectangle);
        }
        if room_cross {
            room_types.push(RoomType::Cross);
        }
        if room_diamond {
            room_types.push(RoomType::Diamond);
        }
        if room_types.len() == 0 {
            room_types.push(RoomType::Rectangle);
        }

        let options = Options::new(
                    level_width,
                    level_height,
                    min_room_width,
                    max_room_width,
                    min_room_height,
                    max_room_height,
                    number_of_rooms,
                    random_seed,
                    border,
                    room_border,
                    false,
                    room_types);

        return LevelGenerator::new_with_options(options);
    }

    #[wasm_bindgen]
    pub fn set_level_size(&mut self, height: usize, width: usize) {
        self.options.set_level_size(height, width);
    }

    #[wasm_bindgen]
    pub fn set_room_size(&mut self, min_room_width: usize,
                                    max_room_width: usize,
                                    min_room_height: usize,
                                    max_room_height: usize) {
        self.options.set_room_size(min_room_width, max_room_width, min_room_height, max_room_height);
    }

    #[wasm_bindgen]
    pub fn set_rooms_count(&mut self, number_of_rooms: usize) {
        self.options.set_rooms_count(number_of_rooms);
    }

    #[wasm_bindgen]
    pub fn set_seed(&mut self, random_seed: usize) {
        self.options.set_seed(random_seed);
    }

    #[wasm_bindgen]
    pub fn set_borders(&mut self, level_border: usize, room_border: usize) {
        self.options.set_borders(level_border, room_border);
    }

    #[wasm_bindgen]
    pub fn add_room_type(&mut self, room_type: u8) {
        match room_type {
            0 => self.options.add_room_type(RoomType::Square),
            1 => self.options.add_room_type(RoomType::Rectangle),
            2 => self.options.add_room_type(RoomType::Cross),
            3 => self.options.add_room_type(RoomType::Diamond),
            _ => {}
        };
    }

    #[wasm_bindgen]
    pub fn remove_room_type(&mut self, room_type: u8) {
        match room_type {
            0 => self.options.remove_room_type(RoomType::Square),
            1 => self.options.remove_room_type(RoomType::Rectangle),
            2 => self.options.remove_room_type(RoomType::Cross),
            3 => self.options.remove_room_type(RoomType::Diamond),
            _ => {}
        };
    }

    #[wasm_bindgen]
    pub fn generate(&mut self) -> Level {
        let mut level = Level::new(self.options.level_height(), self.options.level_width());
        let mut rooms = self.room_generator.generate_rooms(&self.options);
        let corridors = generate_corridors(&mut rooms, &self.options);

        self.render_rooms_on_level(&mut level, &rooms);
        self.render_corridors_on_level(&mut level, &corridors);

        level.inflate(2);

        let mut tile_points: Vec<TilePoint> = Vec::with_capacity(level.height() * level.width());
        for x in 1..level.height() - 1 {
            let x_int = x as i32;
            for y in 1..level.width() - 1 {
                let y_int = y as i32;
                match level.get_from_coordinates(x, y) {
                    Tile::Floor => {
                        for p in 0..self.grid_patterns.len() {
                            let tile = &self.grid_patterns[p];
                            if self.surrounding_area_matches_pattern(&level, Point::new(x_int, y_int), tile.pattern()) {
                                for o in 0..tile.paint_offsets_length() {
                                    let paint_point = tile.paint_offset(o);
                                    tile_points.push(TilePoint::new(Point::new(paint_point.position().x() + x_int, paint_point.position().y() + y_int), paint_point.tile_type().clone()));
                                }
                            }
                        }
                    },
                    _ => {}
                };
            }
        }

        for p in 0..tile_points.len() {
            let tile_point = tile_points[p];
            level.set_tile(tile_point.position().x() as usize, tile_point.position().y() as usize, tile_point.tile_type().clone());
        } 

        let mut room_centers: Vec<Point> = Vec::with_capacity(rooms.len());
        for i in 0..rooms.len() {
            let r_center = rooms[i].center();
            room_centers.push(Point::new(r_center.x() * 2 + 1, r_center.y() * 2 + 1));
        }

        level.set_statistics(rooms.len(), corridors.len(), corridors.len() + 1 == rooms.len(), room_centers);

        return level;
    }
}