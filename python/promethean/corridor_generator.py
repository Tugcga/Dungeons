import math
from typing import List

from promethean.point import Point
from promethean.room_generator import Room, RoomGenerator
from promethean.pseudo_random import PseudoRandom
from promethean.options import PathFinderTile, PathFinderOptions, RoomType, Options
from promethean.path_finder import PathFinder

class Corridor:
    def __init__(self):
        self._tiles: List[Point] = []

    def __repr__(self) -> str:
        return str([str(p) for p in self._tiles])


def calculate_distance_between_2_points(origin: Point, point: Point) -> float:
    xa_minux_xb_squared: int = (point._x - origin._x) ** 2
    ya_minus_yb_squared: int = (point._y - origin._y) ** 2

    return math.sqrt(xa_minux_xb_squared + ya_minus_yb_squared)


def room_distance_from_origin_comparer(room1: Room, room2: Room) -> int:
    reference = Point(0, 0)
    room1_distance_from_reference = calculate_distance_between_2_points(reference, room1._room_center)
    room2_distance_from_reference = calculate_distance_between_2_points(reference, room2._room_center)
    if room1_distance_from_reference < room2_distance_from_reference:
        return -1
    elif room1_distance_from_reference > room2_distance_from_reference:
        return 1
    else:
        return 0


def room_distance_from_origin_key(room: Room) -> float:
    reference = Point(0, 0)
    return calculate_distance_between_2_points(reference, room._room_center)


def generate_pathing_grid(rooms: List[Room], options: Options) -> List[List[PathFinderTile]]:
    pathable_level: List[List[PathFinderTile]] = [[PathFinderTile.Blocked for j in range(options.level_width)]for i in range(options.level_height)]

    x: int = 0
    y: int = 0
    for x in range(options.border, options.level_height - options.border):
        for y in range(options.border, options.level_width - options.border):
            pathable_level[x][y] = PathFinderTile.Pathable

    for room in rooms:
        for x_offset in range(0 - options.room_border, room._height + options.room_border):
            for y_offset in range(0 - options.room_border, room._width + options.room_border):
                x = room._position._x + x_offset
                y = room._position._y + y_offset

                if x < options.border or x >= options.level_height - options.border:
                    continue
                if y < options.border or y >= options.level_width - options.border:
                    continue

                if x == room._room_center._x or y == room._room_center._y:
                    pathable_level[x][y] = PathFinderTile.Pathable
                    continue

                pathable_level[x][y] = PathFinderTile.Blocked

    return pathable_level


def generate_corridors(rooms: List[Room], options: Options) -> List[Corridor]:
    pathable_level: List[List[PathFinderTile]] = generate_pathing_grid(rooms, options)

    pathfinder: PathFinder = PathFinder(pathable_level, PathFinderOptions())
    rooms.sort(key=room_distance_from_origin_key)

    corridors: List[Corridor] = []
    for index in range(0, len(rooms) - 1):
        current_room: Room = rooms[index]
        next_room: Room = rooms[index + 1]

        path = pathfinder.find_path(current_room._room_center, next_room._room_center)

        corridor_length: int = len(path)
        if corridor_length == 0:
            # if the path is empty, then we fails to connect two rooms by a corridor
            # in this case the level is not connected
            # we should change generator parameters or change the random seed
            continue

        corridor = Corridor()
        for i, p in enumerate(path):
            corridor._tiles.append(p)
            # add non-pathable tile in the corridor corners
            if i > 0 and i < corridor_length - 1:
                prev_p: Point = path[i - 1]
                next_p: Point = path[i + 1]
                x: int = p._x
                y: int = p._y
                prev_x: int = prev_p._x
                prev_y: int = prev_p._y
                next_x: int = next_p._x
                next_y: int = next_p._y
                if prev_x == x and x != next_x:
                    if prev_y < y:
                        if x < next_x:
                            pathfinder.block_point(Point(x, y + 1))
                        else:
                            pathfinder.block_point(Point(x + 1, y + 1))
                    else:
                        if x < next_x:
                            pathfinder.block_point(Point(x - 1, y - 1))
                        else:
                            pathfinder.block_point(Point(x - 1, y + 1))
                elif prev_y == y and y != next_y:
                    if prev_x < x:
                        if y < next_y:
                            pathfinder.block_point(Point(x + 1, y - 1))
                        else:
                            pathfinder.block_point(Point(x + 1, y + 1))
                    else:
                        if y < next_y:
                            pathfinder.block_point(Point(x - 1, y - 1))
                        else:
                            pathfinder.block_point(Point(x, y + 1))

        corridors.append(corridor)

    return corridors
