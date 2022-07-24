 use std::fmt;
 use wasm_bindgen::prelude::*;
 use js_sys::Array;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct Point {
     x: i32,
     y: i32
 }

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Point {
        return Point{x, y};
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> i32{
        return self.x;
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> i32{
        return self.y;
    }

    #[wasm_bindgen]
    pub fn to_array_array(size: i32) -> Array {
        let v = Point::to_array(size);
        return v.into_iter().map(JsValue::from).collect();
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

    pub fn to_array(size: i32) -> Vec<Point> {
        let mut to_return: Vec<Point> = Vec::with_capacity(size as usize);
        for i in 0..size {
            to_return.push(Point::new(0, i));
        }
        return to_return;
    }
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}