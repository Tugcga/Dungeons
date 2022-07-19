from typing import List
from dataclasses import dataclass, field

from promethean.options import Tile, RoomType, Options
from promethean.pseudo_random import PseudoRandom
from promethean.room_generator import RoomGenerator, Room
from promethean.corridor_generator import Corridor, generate_corridors
from promethean.level import tile_level, Level, LevelStatistics
from promethean.room_renderer import DefaultRenderer, CrossRenderer, DiamondRenderer, IRoomRenderer
from promethean.grid_pattern import GridPattern, grid_patterns
from promethean.point import Point


class LevelGenerator:
    def __init__(self, options: Options):
        self._options = options
        self._random = PseudoRandom(options.random_seed)
        self._room_generator = RoomGenerator(self._random)

        self._renderer_default = DefaultRenderer()
        self._renderer_diamond = DiamondRenderer()
        self._renderer_cross = CrossRenderer()

        self._grid_patterns: List[GridPattern] = grid_patterns

    def generate(self) -> Level:
        level: Level = Level(self._options.level_height, self._options.level_width)
        rooms: List[Room] = self._room_generator.generate_rooms(self._options)
        corridors: List[Corridor] = generate_corridors(rooms, self._options)

        self._render_rooms_on_level(level, rooms)
        self._render_corridors_on_level(level, corridors)

        level.inflate(2)
        tile_level(level, self._grid_patterns)

        level.set_statistics(len(rooms), len(corridors), len(corridors) == len(rooms) - 1, [Point(r._room_center._x * 2 + 1, r._room_center._y * 2 + 1) for r in rooms])

        return level

    def _get_renderer(self, room_type: RoomType) -> IRoomRenderer:
        if room_type == RoomType.Diamond:
            return self._renderer_diamond
        elif room_type == RoomType.Cross:
            return self._renderer_cross
        else:
            return self._renderer_default

    def _render_rooms_on_level(self, level: Level, rooms: List[Room]):
        for room in rooms:
            renderer: IRoomRenderer = self._get_renderer(room._room_type)
            tiles: List[List[Tile]] = renderer.get_tiles(room)
            for x_offset in range(room._height):
                for y_offset in range(room._width):
                    x: int = room._position._x + x_offset
                    y: int = room._position._y + y_offset
                    level.set_tile(x, y, tiles[x_offset][y_offset])


    def _render_corridors_on_level(self, level: Level, corridors: List[Corridor]):
        for corridor in corridors:
            for point in corridor._tiles:
                level.set_from_point(point, Tile.Floor)
