from typing import List
from dataclasses import dataclass, field

from promethean.options import Tile, TileMask, TilePoint
from promethean.point import Point
from promethean.grid_pattern import GridPattern, grid_patterns


@dataclass
class LevelStatistics:
    init: bool = False
    rooms_count: int = 0
    corridors_count: int = 0
    all_corridors: bool = False
    room_centers: List[Point] = field(default_factory=lambda: [])


class Level:
    def __init__(self, height: int, width: int):
        self._height = height
        self._width = width
        self._level: List[List[Tile]] = [[Tile.Empty for j in range(width)] for i in range(height)]
        self._statistics = LevelStatistics()

    def set_statistics(self, rooms_count: int, corridors_count: int, all_corridors: bool, room_centers: List[Point]):
        self._statistics.init = True
        self._statistics.rooms_count = rooms_count
        self._statistics.corridors_count = corridors_count
        self._statistics.all_corridors = all_corridors
        self._statistics.room_centers = room_centers

    def get_statistics(self) -> LevelStatistics:
        return self._statistics

    def get_height(self) -> int:
        return self._height

    def get_width(self) -> int:
        return self._width

    def set_tile(self, x: int, y: int, tile: Tile):
        self._level[x][y] = tile

    def get_from_point(self, point: Point) -> Tile:
        return self._level[point._x][point._y]

    def get_from_coordinates(self, x: int, y: int) -> Tile:
        return self._level[x][y]

    def set_from_point(self, point: Point, value: Tile):
        self.set_tile(point._x, point._y, value)

    def render(self) -> List[List[Tile]]:
        return self._level

    def inflate(self, inflation_factor: int):
        inflated_matrix = [[Tile.Empty for j in range(inflation_factor * self._width)] for i in range(inflation_factor * self._height)]
        for row in range(self._height):
            for column in range(self._width):
                for xr in range(inflation_factor):
                    for yr in range(inflation_factor):
                        inflated_matrix[row * inflation_factor + xr][column * inflation_factor + yr] = self._level[row][column]
        self._level = inflated_matrix
        self._width = inflation_factor * self._width
        self._height = inflation_factor * self._height

    def __repr__(self) -> str:
        return "\n".join([" ".join([str(int(self._level[i][j])) for j in range(self._width)]) for i in range(self._height)])


def surrounding_area_matches_pattern(level: Level, position: Point, pattern: List[List[TileMask]]) -> bool:
    start: Point = Point(position._x - 1, position._y - 1)
    for x in range(len(pattern)):
        for y in range(len(pattern[x])):
            pattern_value: TileMask = pattern[x][y]
            if pattern_value is TileMask.Wild:
                continue

            mask_value: Tile = Tile.Empty if pattern_value == TileMask.Open else Tile.Floor

            target_value: Tile = level.get_from_coordinates(start._x + x, start._y + y)
            if target_value == mask_value:
                continue

            if target_value == Tile.Empty and mask_value != Tile.Empty:
                return False

            if target_value != Tile.Empty and mask_value == Tile.Empty:
                return False
    return True


def tile_level(level: Level, grid_patterns: List[GridPattern]):
    tile_points: List[TilePoint] = []
    for x in range(1, level._height - 1):
        for y in range(1, level._width - 1):
            if level.get_from_coordinates(x, y) != Tile.Floor:
                continue

            for tile in grid_patterns:
                if surrounding_area_matches_pattern(level, Point(x, y), tile._pattern):
                    for paint_point in tile._paint_offsets:
                        tile_points.append(TilePoint(Point(paint_point.position._x + x, paint_point.position._y + y), paint_point.tile_type))

    for tile_point in tile_points:
        level.set_tile(tile_point.position._x, tile_point.position._y, tile_point.tile_type)
