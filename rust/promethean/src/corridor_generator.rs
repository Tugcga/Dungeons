#[path = "path_finder.rs"] mod path_finder;

use crate::level::point::Point;
use crate::level::options::{PathFinderTile, Options};
use crate::level_generator::room_generator::{Room};
use path_finder::PathFinder;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct Corridor {
    tiles: Vec<Point>
}

impl Corridor {
    pub fn new(points: Vec<Point>) -> Corridor {
        return Corridor { tiles: points };
    }

    pub fn get_tiles(&self) -> &Vec<Point> {
        return &self.tiles;
    }
}

fn calculate_distance_between_2_points(origin: &Point, point: &Point) -> f64 {
    let xa_minux_xb_squared: i32 = (point.x() - origin.x()) * (point.x() - origin.x());
    let ya_minus_yb_squared: i32 = (point.y() - origin.y()) * (point.y() - origin.y());

    return ((xa_minux_xb_squared + ya_minus_yb_squared) as f64).sqrt();
}

pub fn generate_pathing_grid(rooms: &Vec<Room>, options: &Options) -> Vec<Vec<PathFinderTile>> {
    let mut pathable_level: Vec<Vec<PathFinderTile>> = Vec::with_capacity(options.level_height());
    for x in 0..options.level_height() {
        let mut x_array: Vec<PathFinderTile> = Vec::with_capacity(options.level_width());
        for y in 0..options.level_width() {
            if x < options.border() || x >= options.level_height() - options.border() || y < options.border() || y >= options.level_width() - options.border() {
                x_array.push(PathFinderTile::Blocked);
            }
            else {
                x_array.push(PathFinderTile::Pathable);
            }
        }
        pathable_level.push(x_array);
    }

    let mut x: usize;
    let mut y: usize;

    for r_index in 0..rooms.len() {
        let room = &rooms[r_index];
        for x_offset in -(options.room_border() as i32)..((room.height() + options.room_border()) as i32) {
            for y_offset in -(options.room_border() as i32)..((room.width() + options.room_border()) as i32) {
                x = (room.position().x() + x_offset) as usize;
                y = (room.position().y() + y_offset) as usize;

                if x < options.border() || x >= options.level_height() - options.border() {
                    continue;
                }
                if y < options.border() || y >= options.level_width() - options.border() {
                    continue;
                }
                
                let room_center = room.center();
                if x == room_center.x() as usize || y == room_center.y() as usize {
                    pathable_level[x][y] = PathFinderTile::Pathable;
                    continue;
                }

                pathable_level[x][y] = PathFinderTile::Blocked;
            }
        }
    }

    return pathable_level;
}

pub fn generate_corridors(rooms: &mut Vec<Room>, options: &Options) -> Vec<Corridor> {
    if rooms.len() <= 1 {
        return Vec::new();
    }

    let pathable_level: Vec<Vec<PathFinderTile>> = generate_pathing_grid(rooms, options);

    let mut pathfinder: PathFinder = PathFinder::new(pathable_level, 2000);
    rooms.sort_by(|room1, room2| {
        let reference = Point::new(0, 0);
        let room1_distance_from_reference = calculate_distance_between_2_points(&reference, room1.center());
        let room2_distance_from_reference = calculate_distance_between_2_points(&reference, room2.center());
        if room1_distance_from_reference < room2_distance_from_reference {
            return Ordering::Less;
        }
        else if room1_distance_from_reference > room2_distance_from_reference {
            return Ordering::Greater;
        }
        else{
            return Ordering::Equal;
        }
    });

    let mut corridors = Vec::with_capacity(rooms.len() - 1);
    for index in 0..rooms.len() - 1 {
        let current_room = &rooms[index];
        let next_room = &rooms[index + 1];

        let path: Vec<Point> = pathfinder.find_path(current_room.center(), next_room.center());

        let corridor_length = path.len();
        if corridor_length == 0 {
            continue;
        }

        for i in 1..corridor_length - 1 {
            let p = &path[i];
            let prev_p = &path[i - 1];
            let next_p = &path[i + 1];
            let x = p.x();
            let y = p.y();
            let prev_x: i32 = prev_p.x();
            let prev_y: i32 = prev_p.y();
            let next_x: i32 = next_p.x();
            let next_y: i32 = next_p.y();
            if prev_x == x && x != next_x {
                if prev_y < y {
                    if x < next_x {
                        pathfinder.block_point(Point::new(x, y + 1));
                    }
                    else {
                        pathfinder.block_point(Point::new(x + 1, y + 1));
                    }
                }
                else {
                    if x < next_x {
                        pathfinder.block_point(Point::new(x - 1, y - 1));
                    }
                    else {
                        pathfinder.block_point(Point::new(x - 1, y + 1));
                    }
                }
            }
            else if prev_y == y && y != next_y {
                if prev_x < x {
                    if y < next_y {
                        pathfinder.block_point(Point::new(x + 1, y - 1));
                    }
                    else {
                        pathfinder.block_point(Point::new(x + 1, y + 1));
                    }
                }
                else {
                    if y < next_y {
                        pathfinder.block_point(Point::new(x - 1, y - 1));
                    }
                    else {
                        pathfinder.block_point(Point::new(x, y + 1));
                    }
                }
            }
        }
        let corridor = Corridor::new(path);

        corridors.push(corridor);
    }

    return corridors;
}