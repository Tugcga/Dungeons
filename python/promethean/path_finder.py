from typing import List, Optional

from promethean.options import PathFinderTile, PathFinderOptions
from promethean.point import Point


class PathFinderNode:
    def __init__(self, position: Point, g: int, h: int, parent_node_position: Point):
        self._position: Point = position
        self._g: int = g
        self._h: int = h
        self._parent_node_position: Point = parent_node_position
        self._f: int = -1  # non-defined value
        self._closed: Optional[bool] = None  # make True when we remove the node from closed list, make False when we add the node to the open list

    def reset(self):
        self._g = 0
        self._h = 0
        self._parent_node_position = Point()
        self._f = -1
        self._closed = None

    def update(self, g: int, h: int, parent: Point):
        if self._f == -1 or self._f > g + h:
            self._g = g
            self._h = h
            self._f = g + h
            self._parent_node_position = parent

    def close(self):
        self._closed = True

    def open(self):
        self._closed = False

    def is_closed(self) -> bool:
        if self._closed is None:
            return False
        else:
            return self._closed

    def is_undefined(self) -> bool:
        if self._closed is None:
            return True
        else:
            return False

    def __repr__(self) -> str:
        return "<" + str(self._position) + ":" + str(self._g) + ":" + str(self._h) + ":" + str(self._parent_node_position) + ":" + str(self._closed) + ">"


def distance(start: Point, end: Point) -> int:
    return abs(start._x - end._x) + abs(start._y - end._y)


class PathFinderGraph:
    def __init__(self, height: int, width: int):
        self._height: int = height
        self._width: int = width
        self._internal_grid: List[List[PathFinderNode]] = [[PathFinderNode(Point(i, j), 0, 0, Point()) for j in range(width)]for i in range(height)]
        self._open: List[PathFinderNode] = []

    def reset(self):
        for i in range(self._height):
            for j in range(self._width):
                self._internal_grid[i][j].reset()
        self._open.clear()

    def open_node(self, position: Point, g: int, h: int, parent: Point):
        node: PathFinderNode = self._internal_grid[position._x][position._y]
        node.update(g, h, parent)
        node.open()
        self._open.append(node)

    def has_open_nodes(self) -> bool:
        return len(self._open) > 0

    def get_open_node_with_smallest_f(self) -> Optional[PathFinderNode]:
        if len(self._open) > 0:
            to_return_i: int = 0
            to_return_f: int = self._open[0]._f
            for i, node in enumerate(self._open):
                if node._f < to_return_f:
                    to_return_i = i
                    to_return_f = node._f
            # set the node closed
            return_node: PathFinderNode = self._open.pop(to_return_i)
            return_node.close()
            return return_node
        else:
            return None

    def add_node(self, x: int, y: int, g: int, target: Point, parent: Point):
        node: PathFinderNode = self._internal_grid[x][y]
        h: int = distance(node._position, target)
        if not node.is_closed():
            node.update(g, h, parent)
            # add to the open list
            if node.is_undefined():
                node.open()
                self._open.append(node)


def order_closed_nodes_as_array(graph: PathFinderGraph, end_node: PathFinderNode) -> List[Point]:
    current_node: PathFinderNode = end_node
    to_return: List[Point] = [Point() for i in range(end_node._g + 1)]
    for i in range(end_node._g + 1):
        to_return[len(to_return) - i - 1] = current_node._position
        current_node = graph._internal_grid[current_node._parent_node_position._x][current_node._parent_node_position._y]
    return to_return


class PathFinder:
    def __init__(self, world_grid: List[List[PathFinderTile]], options: PathFinderOptions):
        self._world_grid = world_grid
        self._options = options
        self._height = len(world_grid)
        self._width = len(world_grid[0])
        self._graph: PathFinderGraph = PathFinderGraph(self._height, self._width)

    def block_point(self, point: Point):
        self._world_grid[point._x][point._y] = PathFinderTile.Blocked

    def find_path(self, start: Point, end: Point) -> List[Point]:
        nodes_visited: int = 0
        self._graph.reset()
        self._graph.open_node(start, 0, distance(start, end), start)

        while self._graph.has_open_nodes():
            q: Optional[PathFinderNode] = self._graph.get_open_node_with_smallest_f()
            if q is not None:
                if q._position == end:
                    return order_closed_nodes_as_array(self._graph, q)

                if nodes_visited > self._options.search_limit:
                    return []

                # for each node we should select 4 adjacent walkable points
                # and add it to the open list, if the point is not visited
                x: int = q._position._x
                y: int = q._position._y
                g = q._g + 1

                if self._world_grid[x][y - 1] == PathFinderTile.Pathable:
                    self._graph.add_node(x, y - 1, g, end, q._position)
                if self._world_grid[x][y + 1] == PathFinderTile.Pathable:
                    self._graph.add_node(x, y + 1, g, end, q._position)
                if self._world_grid[x - 1][y] == PathFinderTile.Pathable:
                    self._graph.add_node(x - 1, y, g, end, q._position)
                if self._world_grid[x + 1][y] == PathFinderTile.Pathable:
                    self._graph.add_node(x + 1, y, g, end, q._position)
            else:
                break

            nodes_visited += 1

        return []

    def __repr__(self) -> str:
        return "\n".join([" ".join([str(int(self._world_grid[i][j])) for j in range(self._width)]) for i in range(self._height)])
