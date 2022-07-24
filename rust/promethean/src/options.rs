use super::point::Point;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoomType {
    Square = 0,
    Rectangle = 1,
    Cross = 2,
    Diamond = 3
}

#[derive(Clone, Copy, Debug)]
pub enum PathFinderTile {
    Blocked = 0,
    Pathable = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Floor = 0,
    Empty = 1,
    TopLeftInsideCorner = 2,
    TopRightInsideCorner = 3,
    BottomLeftInsideCorner = 4,
    BottomRightInsideCorner = 5,
    TopWall = 6,
    RightWall = 7,
    BottomWall = 8,
    LeftWall = 9,
    TopLeftOutsideCorner = 10,
    TopRightOutsideCorner = 11,
    BottomLeftOutsideCorner = 12,
    BottomRightOutsideCorner = 13
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileMask {
    Wild = 0,
    Block = 1,
    Open = 2
}

#[derive(Clone, Copy, Debug)]
pub struct TilePoint {
    position: Point,
    tile_type: Tile
}

impl TilePoint {
    pub fn new(position: Point, tile_type: Tile) -> TilePoint {
        return TilePoint { position, tile_type };
    }

    pub fn position(&self) -> &Point {
        return &self.position;
    }

    pub fn tile_type(&self) -> &Tile {
        return &self.tile_type;
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    level_width: usize,
    level_height: usize,
    min_room_width: usize,
    max_room_width: usize,
    min_room_height: usize,
    max_room_height: usize,
    number_of_rooms: usize,
    random_seed: usize,
    border: usize,
    room_border: usize,
    overlap_rooms: bool,
    room_types: Vec<RoomType>
}

impl Options {
    pub fn new_default() -> Options {
        return Options {
            level_width: 64,
            level_height: 64,
            min_room_width : 5,
            max_room_width: 7,
            min_room_height: 5,
            max_room_height: 7,
            number_of_rooms: 45,
            random_seed: 1,
            border: 1,
            room_border: 1,
            overlap_rooms: false,
            room_types: vec![RoomType::Square, RoomType::Rectangle, RoomType::Cross, RoomType::Diamond]
        };
    }

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
               overlap_rooms: bool,
               room_types: Vec<RoomType>) -> Options {
        return Options {
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
            overlap_rooms,
            room_types
        };
    }

    pub fn set(&mut self, level_width: usize,
                      level_height: usize,
                      min_room_width: usize,
                      max_room_width: usize,
                      min_room_height: usize,
                      max_room_height: usize,
                      number_of_rooms: usize,
                      random_seed: usize,
                      border: usize,
                      room_border: usize,
                      overlap_rooms: bool,
                      room_types: Vec<RoomType>) {
        self.level_width = level_width;
        self.level_height = level_height;
        self.min_room_width = min_room_width;
        self.max_room_width = max_room_width;
        self.min_room_height = min_room_height;
        self.max_room_height = max_room_height;
        self.number_of_rooms = number_of_rooms;
        self.random_seed = random_seed;
        self.border = border;
        self.room_border = room_border;
        self.overlap_rooms = overlap_rooms;
        self.room_types = room_types;
    }

    pub fn set_level_size(&mut self, height: usize, width: usize) {
        self.level_height = height;
        self.level_width = width;
    }

    pub fn set_room_size(&mut self, min_room_width: usize,
                                    max_room_width: usize,
                                    min_room_height: usize,
                                    max_room_height: usize,) {
        if min_room_width > max_room_width {
            self.min_room_width = max_room_width;
            self.max_room_width = min_room_width;
        }
        else {
            self.min_room_width = min_room_width;
            self.max_room_width = max_room_width;
        }
        
        if min_room_height > max_room_height {
            self.min_room_height = max_room_height;
            self.max_room_height = min_room_height;
        }
        else {
            self.min_room_height = min_room_height;
            self.max_room_height = max_room_height;
        }
    }

    pub fn set_rooms_count(&mut self, number_of_rooms: usize) {
        self.number_of_rooms = number_of_rooms;
    }

    pub fn set_seed(&mut self, random_seed: usize) {
        self.random_seed = random_seed;
    }

    pub fn set_borders(&mut self, level_border: usize, room_border: usize) {
        self.border = level_border;
        self.room_border = room_border;
    }

    pub fn set_overlaps(&mut self, overlap_rooms: bool) {
        self.overlap_rooms = overlap_rooms;
    }

    pub fn set_room_types(&mut self, room_types: Vec<RoomType>) {
        self.room_types = room_types;
    }

    pub fn add_room_type(&mut self, room_type: RoomType) {
        if !self.room_types.contains(&room_type) {
            self.room_types.push(room_type);
        }
    }

    pub fn remove_room_type(&mut self, room_type: RoomType) {
        let res_index = self.room_types.iter().position(|&r| r == room_type);
        match res_index {
            Some(index) => {
                self.room_types.remove(index);
            },
            None => {},
        }
    }

    pub fn room_types(&self) -> &Vec<RoomType> {
        return &self.room_types;
    }

    pub fn min_room_width(&self) -> usize {
        return self.min_room_width;
    }

    pub fn max_room_width(&self) -> usize {
        return self.max_room_width;
    }

    pub fn min_room_height(&self) -> usize {
        return self.min_room_height;
    }

    pub fn max_room_height(&self) -> usize {
        return self.max_room_height;
    }

    pub fn level_height(&self) -> usize {
        return self.level_height;
    }

    pub fn level_width(&self) -> usize {
        return self.level_width;
    }

    pub fn border(&self) -> usize {
        return self.border;
    }

    pub fn room_border(&self) -> usize {
        return self.room_border;
    }

    pub fn number_of_rooms(&self) -> usize {
        return self.number_of_rooms;
    }

    pub fn overlap_rooms(&self) -> bool {
        return self.overlap_rooms;
    }

    pub fn random_seed(&self) -> usize {
        return self.random_seed;
    }
}