use crate::level::point::Point;
use crate::level::options::PathFinderTile;
use std::fmt;

#[derive(Debug)]
pub struct PathFinderNode {
    position: Point,
    g: usize,
    h: usize,
    parent_node_position: Point,
    f: usize,
    closed: u8
}

impl PathFinderNode{
    pub fn new(position: Point, g: usize, h: usize, parent_node_position: Point) -> PathFinderNode {
        return PathFinderNode{ position: position,
                               g: g,
                               h: h,
                               parent_node_position: parent_node_position,
                               f: 0,
                               closed: 0};
    }

    pub fn clone(&self) -> PathFinderNode {
        return PathFinderNode { position: self.position.clone(),
                                g: self.g,
                                h: self.h,
                                parent_node_position: self.parent_node_position.clone(),
                                f: self.f,
                                closed: self.closed};
    }

    pub fn f(&self) -> usize{
        return self.f;
    }

    pub fn g(&self) -> usize{
        return self.g;
    }

    pub fn position(&self) -> &Point{
        return &self.position;
    }

    pub fn parent_position(&self) -> &Point{
        return &self.parent_node_position;
    }

    pub fn reset(&mut self) {
        self.g = 0;
        self.h = 0;
        self.parent_node_position.set(0, 0);
        self.f = 0;
        self.closed = 0;
    }

    pub fn update(&mut self, g: usize, h: usize, parent: Point) {
        if self.f == 0 || self.f > g + h{
            self.g = g;
            self.h = h;
            self.f = g + h;
            self.parent_node_position = parent;
        }
    }

    pub fn close(&mut self) {
        self.closed = 2;
    }

    pub fn open(&mut self) {
        self.closed = 1;
    }

    pub fn is_closed(&self) -> bool {
        if self.closed != 0{
            return self.closed == 2
        }
        else {
            return false;
        }
    }

    pub fn is_undefined(&self) -> bool {
        if self.closed != 0 {
            return false;
        }
        else {
            return true;
        }
    }
}

impl fmt::Display for PathFinderNode{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "<{}:{}:{}:{}:{}>", self.position, self.g, self.h, self.parent_node_position, self.closed);
    }
}

pub fn distance(start: &Point, end: &Point) -> usize{
    let sx = start.x();
    let sy = start.y();
    let ex = end.x();
    let ey = end.y();
    let x: usize = if sx > ex { (sx - ex) as usize } else { (ex - sx) as usize };
    let y: usize = if sy > ey { (sy - ey) as usize } else { (ey - sy) as usize };
    return x + y;
}

#[derive(Debug)]
pub struct PathFinderGraph {
    height: usize,
    width: usize,
    internal_grid: Vec<Vec<PathFinderNode>>,
    open: Vec<Point>
}

impl PathFinderGraph {
    pub fn new(height: usize, width: usize) -> PathFinderGraph {
        let mut internal_grid = Vec::with_capacity(height);
        for x in 0..height {
            let mut x_array = Vec::with_capacity(width);
            for y in 0..width {
                x_array.push(PathFinderNode::new(Point::new(x as i32, y as i32), 0, 0, Point::new(0, 0)));
            }
        internal_grid.push(x_array);
        }

        let open = Vec::with_capacity(width);

        return PathFinderGraph{ height, width, internal_grid, open};
    }

    pub fn get_node_from_grid(&self, x: usize, y: usize) -> &PathFinderNode {
        return &self.internal_grid[x][y];
    }

    pub fn reset(&mut self) {
        for x in 0.. self.height {
            for y in 0.. self.width {
                self.internal_grid[x][y].reset();
            }
        }
        self.open.clear();
    }

    pub fn open_node(&mut self, position: &Point, g: usize, h: usize, parent: &Point) {
        let node: &mut PathFinderNode = &mut self.internal_grid[position.x() as usize][position.y() as usize];
        node.update(g, h, Point::new(parent.x(), parent.y()));
        node.open();

        self.open.push(Point::new(node.position.x(), node.position.y()));
    }

    pub fn has_open_nodes(&self) -> bool {
        return self.open.len() > 0;
    }

    pub fn get_open_node_with_smallest_f(&mut self) -> PathFinderNode {
        let zero_point: &Point = &self.open[0];
        let mut to_return_i: usize = 0;
        let mut to_return_f: usize = self.internal_grid[zero_point.x() as usize][zero_point.y() as usize].f();

        for i in 0..self.open.len() {
            let i_point: &Point = &self.open[i];
            let node: &PathFinderNode = &self.internal_grid[i_point.x() as usize][i_point.y() as usize];
            let node_f: usize = node.f();

            if node_f < to_return_f {
                to_return_i = i;
                to_return_f = node_f;
            }
        }

        let zero_point = &self.open[to_return_i];
        let mut return_node: PathFinderNode = self.internal_grid[zero_point.x() as usize][zero_point.y() as usize].clone();
        self.open.remove(to_return_i);

        return_node.close();

        return return_node;
    }

    pub fn add_node(&mut self, x: usize, y: usize, g: usize, target: &Point, parent: &Point) {
        let node: &mut PathFinderNode = &mut self.internal_grid[x][y];
        let h = distance(node.position(), target);
        if !node.is_closed() {
            node.update(g, h, Point::new(parent.x(), parent.y()));
            if node.is_undefined() {
                node.open();
                self.open.push(Point::new(node.position.x(), node.position.y()));
            }
        }
    }
}

pub fn order_closed_nodes_as_array(graph: &PathFinderGraph, end_node: &PathFinderNode) -> Vec<Point> {
    let mut current_node: &PathFinderNode = end_node;
    let length = end_node.g() + 1;
    let mut to_return = Vec::with_capacity(length);

    for _ in 0..length {
        to_return.push(current_node.position().clone());
        let pnp: &Point = current_node.parent_position();
        current_node = graph.get_node_from_grid(pnp.x() as usize, pnp.y() as usize);
    }
    to_return.reverse();
    return to_return;
}

#[derive(Debug)]
pub struct PathFinder {
    world_grid: Vec<Vec<PathFinderTile>>,
    search_limit: usize,
    height: usize,
    width: usize,
    graph: PathFinderGraph
}

impl PathFinder {
    pub fn new(world_grid: Vec<Vec<PathFinderTile>>, search_limit: usize) -> PathFinder {
        let height = world_grid.len();
        let width = world_grid[0].len();
        return PathFinder { world_grid, search_limit, height, width, graph: PathFinderGraph::new(height, width)};
    }

    pub fn block_point(&mut self, point: Point) {
        self.world_grid[point.x() as usize][point.y() as usize] = PathFinderTile::Blocked;
    }

    pub fn find_path(&mut self, start: &Point, end: &Point) -> Vec<Point> {
        let mut nodes_visited: usize = 0;
        self.graph.reset();
        self.graph.open_node(start, 0, distance(start, end), start);

        while self.graph.has_open_nodes() {
            let q: PathFinderNode = self.graph.get_open_node_with_smallest_f();
            if q.position().equal(end) {
                return order_closed_nodes_as_array(&self.graph, &q);
            }

            if nodes_visited > self.search_limit {
                return Vec::new();
            }

            let x: usize = q.position.x() as usize;
            let y: usize = q.position.y() as usize;
            let g: usize = q.g() + 1;

            match self.world_grid[x][y - 1] {
                PathFinderTile::Pathable => self.graph.add_node(x, y - 1, g, end, q.position()),
                PathFinderTile::Blocked => {}
            }
            match self.world_grid[x][y + 1] {
                PathFinderTile::Pathable => self.graph.add_node(x, y + 1, g, end, q.position()),
                PathFinderTile::Blocked => {}
            }
            match self.world_grid[x - 1][y] {
                PathFinderTile::Pathable => self.graph.add_node(x - 1, y, g, end, q.position()),
                PathFinderTile::Blocked => {}
            }
            match self.world_grid[x + 1][y] {
                PathFinderTile::Pathable => self.graph.add_node(x + 1, y, g, end, q.position()),
                PathFinderTile::Blocked => {}
            }

            nodes_visited += 1;
        }
        return Vec::new();
    }
}

impl fmt::Display for PathFinder{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut to_return = String::new();
        for x in 0..self.height {
            let mut x_str = String::new();
            for y in 0..self.width {
                x_str.push_str(match self.world_grid[x][y] {PathFinderTile::Blocked => "■ ", PathFinderTile::Pathable =>  "□ " });
            }
            to_return.push_str((x_str + "\n").as_str());
        }

        return write!(f, "{}", to_return);
    }
}