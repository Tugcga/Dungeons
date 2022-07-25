 use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Point {
     x: i32,
     y: i32
 }

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        return Point{x, y};
    }

    pub fn x(&self) -> i32{
        return self.x;
    }

    pub fn y(&self) -> i32{
        return self.y;
    }
}

impl Point {
    pub fn equal(&self, other: &Point) -> bool{
        return self.x == other.x && self.y == other.y;
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn clone(&self) -> Point {
        return Point{ x: self.x, y: self.y };
    }
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}