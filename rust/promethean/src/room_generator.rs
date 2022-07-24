use crate::level::point::Point;
use crate::level::options::{Options, RoomType, Tile};
use super::preudo_random::PseudoRandom;
use std::fmt;

pub fn determine_max_position(level_dimension: usize, room_dimension: usize, border: usize) -> usize {
    return level_dimension - room_dimension - border;
}

#[derive(Debug)]
pub struct Room {
    height: usize,
    width: usize,
    position: Point,
    room_center: Point,
    bottom_right: Point,
    room_type: RoomType
}

impl Room {
    pub fn new(room_height: usize, room_width: usize, room_x: usize, room_y: usize, room_type: RoomType) -> Room {
        return Room {
            height: room_height,
            width: room_width,
            position: Point::new(room_x as i32, room_y as i32),
            room_center: Point::new((room_x + room_height / 2) as i32, (room_y + room_width / 2) as i32),
            bottom_right: Point::new((room_x + room_height - 1) as i32, (room_y + room_width - 1) as i32),
            room_type: room_type
        };
    }

    pub fn room_type(&self) -> RoomType {
        return self.room_type;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn position(&self) -> &Point {
        return &self.position;
    }

    pub fn bottom_right(&self) -> &Point {
        return &self.bottom_right;
    }

    pub fn center(&self) -> &Point {
        return &self.room_center;
    }

    pub fn intersects(&self, other: &Room, buffer: usize) -> bool {
        if self.bottom_right.y() as usize + buffer < other.position().y() as usize - buffer || other.bottom_right().y() as usize + buffer < self.position.y() as usize - buffer {
            return false;
        }

        if self.bottom_right.x() as usize + buffer < other.position().x() as usize - buffer || other.bottom_right().x() as usize + buffer < self.position.x() as usize - buffer {
            return false;
        }

        return true;
    }

    pub fn get_tiles(&self) -> Vec<Vec<Tile>> {
        match self.room_type {
            RoomType::Diamond => {
                let mut offset: i32 = 0;
                let y_middle: i32 = (self.width / 2) as i32;
                
                let mut arr: Vec<Vec<Tile>> = Vec::with_capacity(self.height);
                for x in 0..self.height {
                    let mut arr_x: Vec<Tile> = Vec::with_capacity(self.width);
                    for y in 0..self.width {
                        let y_int = y as i32;
                        if y_int < y_middle - offset || y_int >= y_middle + offset + 1 {
                            arr_x.push(Tile::Empty);
                        }
                        else {
                            arr_x.push(Tile::Floor);
                        }
                    }
                    offset =  if x < self.height / 2 { offset + 1 } else { offset - 1 };
                    arr.push(arr_x);
                }
                return arr;
            },
            RoomType::Cross => {
                let mut arr: Vec<Vec<Tile>> = Vec::with_capacity(self.height);

                let height_float = self.height as f64;
                let width_float = self.width as f64;
                for x in 0..self.height {
                    let mut arr_x: Vec<Tile> = Vec::with_capacity(self.width);
                    let x_float = x as f64;
                    for y in 0..self.width {
                        let y_float = y as f64;
                        let x_lower_bound = height_float * 0.333 - 1.0;
                        let x_upper_bound = height_float * 0.666;

                        let y_lower_bound = width_float * 0.333 - 1.0;
                        let y_upper_bound = width_float * 0.666;
                        if (x_float > x_lower_bound && x_float < x_upper_bound) || (y_float > y_lower_bound && y_float < y_upper_bound) {
                            arr_x.push(Tile::Floor);
                        }
                        else {
                            arr_x.push(Tile::Empty);
                        }
                    }
                    arr.push(arr_x);
                }
                return arr;
            },
            _ => {
                let mut arr: Vec<Vec<Tile>> = Vec::with_capacity(self.height);
                for _x in 0..self.height {
                    let mut arr_x: Vec<Tile> = Vec::with_capacity(self.width);
                    for _y in 0..self.width {
                        arr_x.push(Tile::Floor);
                    }
                    arr.push(arr_x);
                }
                return arr;
            }
        };
    }
}

impl fmt::Display for Room{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Room{}:{}:{}:t{:?}", self.position, self.height, self.width, self.room_type);
    }
}

#[derive(Debug)]
pub struct RoomGenerator {
    random: PseudoRandom
}

impl RoomGenerator {
    pub fn new(in_random: PseudoRandom) -> RoomGenerator {
        return RoomGenerator{ random: in_random };
    }

    fn generate(&mut self, options : &Options) -> Option<Room> {
        let room_types = options.room_types();
        let room_type: RoomType = room_types[self.random.next(1, room_types.len()) - 1];
        let room_width = self.random.next_odd(options.min_room_width(), options.max_room_width());
        let room_height = match room_type {
            RoomType::Rectangle => self.random.next_odd(options.min_room_height(), options.max_room_height()),
            _ => room_width
        };
        let max_x = determine_max_position(options.level_height(), room_height, options.border());
        let max_y = determine_max_position(options.level_width(), room_width, options.border());
        if max_x >= options.border() && max_y >= options.border() {
            let room_x = self.random.next(options.border(), max_x);
            let room_y = self.random.next(options.border(), max_y);
            return Some(Room::new(room_height, room_width, room_x, room_y, room_type));
        }
        else{
            return Option::None;
        }
    }

    fn is_intersections(&self, rooms: &Vec<Room>, target: &Room, border: usize) -> bool {
        for room in rooms {
            if target.intersects(room, border) {
                return true;
            }
        }
        return false;
    }

    fn reposition(&self, rooms: &Vec<Room>, room: &Room, options: &Options) -> Option<Room> {
        let min_x: usize = options.border();
        let min_y: usize = options.border();
        let max_x: usize = determine_max_position(options.level_height(), room.height(), options.border());
        let max_y: usize = determine_max_position(options.level_width(), room.width(), options.border());

        let lower_bound = Point::new(min_x as i32, min_y as i32);
        let upper_bound = Point::new(max_x as i32, max_y as i32);

        let mut offset: usize  = 0;
        let mut is_find: bool;
        let mut is_switch_direction: bool;
        let mut top_left: Point = Point::new(0, 0);
        let mut top_right: Point = Point::new(0, 0);
        let mut bottom_right: Point = Point::new(0, 0);
        let mut bottom_left: Point = Point::new(0, 0);
        let mut direction: i32 = -1;
        let mut direction_iterator: i32 = top_left.y();
        let mut position: Option<Point>;

        loop {
            position = Option::None;
            is_switch_direction = false;
            is_find = false;
            if direction == -1 {
                is_switch_direction = true;
            }
            else if direction == 0 {
                if top_left.x() >= lower_bound.x() {
                    while direction_iterator <= top_right.y() && direction_iterator <= upper_bound.y() {
                        if direction_iterator < lower_bound.y() {
                            direction_iterator += 1;
                        }
                        else {
                            position = Option::Some(Point::new(top_left.x(), direction_iterator));
                            direction_iterator += 1;
                            is_find = true;
                            break;
                        }
                    }
                    if !is_find {
                        is_switch_direction = true;
                    }
                }
                else {
                    is_switch_direction = true;
                }
            }
            else if direction == 1 {
                if top_right.y() <= upper_bound.y() {
                    while direction_iterator < bottom_right.x() && direction_iterator <= upper_bound.x() {
                        if direction_iterator < lower_bound.x() {
                            direction_iterator += 1;
                        }
                        else {
                            position = Option::Some(Point::new(direction_iterator, top_right.y()));
                            direction_iterator += 1;
                            is_find = true;
                            break;
                        }
                    }
                    if !is_find {
                        is_switch_direction = true;
                    }
                }
                else {
                    is_switch_direction = true;
                }
            }
            else if direction == 2 {
                if bottom_right.x() <= upper_bound.x() {
                    while direction_iterator >= bottom_left.y() && direction_iterator >= lower_bound.y() {
                        if direction_iterator > upper_bound.y() {
                            direction_iterator -= 1;
                        }
                        else {
                            position = Option::Some(Point::new(bottom_right.x(), direction_iterator));
                            direction_iterator -= 1;
                            is_find = true;
                            break;
                        }
                    }
                    if !is_find {
                        is_switch_direction = true;
                    }
                }
                else {
                    is_switch_direction = true;
                }
            }
            else if direction == 3 {
                if bottom_left.y() >= lower_bound.y() {
                    while direction_iterator > top_left.x() && direction_iterator >= lower_bound.x() + 1 {
                        if direction_iterator > upper_bound.x() {
                            direction_iterator -= 1;
                        }
                        else {
                            position = Option::Some(Point::new(direction_iterator, bottom_left.y()));
                            direction_iterator -= 1;
                            is_find = true;
                            break;
                        }
                    }
                    if !is_find {
                        is_switch_direction = true;
                    }
                }
                else {
                    is_switch_direction = true;
                }
            }

            if is_switch_direction {
                if direction == 0 {
                    direction = 1;
                    direction_iterator = top_right.x() + 1;
                }
                else if direction == 1 {
                    direction = 2;
                    direction_iterator = bottom_right.y();
                }
                else if direction == 2 {
                    direction = 3;
                    direction_iterator = bottom_left.x() - 1;
                }
                else {
                    offset += 1;
                    let rx = room.position().x();
                    let ry = room.position().y();
                    top_left.set(rx - offset as i32, ry - offset as i32);
                    top_right.set(rx - offset as i32, ry + offset as i32);
                    bottom_right.set(rx + offset as i32, ry + offset as i32);
                    bottom_left.set(rx + offset as i32, ry - offset as i32);

                    if (rx - (offset as i32) < lower_bound.x()) && (rx + (offset as i32) > upper_bound.x()) && (ry - (offset as i32) < lower_bound.y()) && (ry + (offset as i32) > upper_bound.y()) {
                        break;
                    }
                    else {
                        direction = 0;
                        direction_iterator = top_left.y();
                    }
                }
            }
            else {
                match position {
                    Some(p) => {
                        let new_room_candidate = Room::new(room.height(), room.width(), p.x() as usize, p.y() as usize, room.room_type());
                        if !self.is_intersections(rooms, &new_room_candidate, options.room_border()) {
                            return Option::Some(new_room_candidate);
                        }
                    },
                    None => break,
                }
            }
        }

        return Option::None;
    }

    fn generate_overlapping_rooms(&mut self, options: &Options) -> Vec<Room> {
        let mut rooms: Vec<Room> = Vec::with_capacity(options.number_of_rooms());
        for _ in 0..options.number_of_rooms() {
            let new_room = self.generate(options);
            match new_room {
                Some(room) => rooms.push(room),
                None => {}
            };
        }

        return rooms;
    }

    fn generate_non_overlapping_rooms(&mut self, options: &Options) -> Vec<Room> {
        let mut rooms: Vec<Room> = Vec::with_capacity(options.number_of_rooms());
        for _ in 0..options.number_of_rooms() {
            let new_room = self.generate(options);
            match new_room {
                Some(room) => {
                    if self.is_intersections(&rooms, &room, options.room_border()){
                        let repositioned_room = self.reposition(&rooms, &room, options);
                        match repositioned_room {
                            Some(r) => {
                                rooms.push(r);
                            },
                            None => {
                                break;
                            }
                        }
                    }
                    else {
                        rooms.push(room);
                    }
                },
                None => {}
            };
        }

        return rooms;
    }

    pub fn generate_rooms(&mut self, options: &Options) -> Vec<Room> {
        if options.overlap_rooms() {
            return self.generate_overlapping_rooms(options);
        }
        else {
            return self.generate_non_overlapping_rooms(options);
        }
    }
}