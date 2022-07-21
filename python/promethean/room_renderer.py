from typing import List

from promethean.room_generator import Room
from promethean.options import Tile, RoomType


class IRoomRenderer:
    def get_tiles(self, room: Room) -> List[List[Tile]]:
        pass


class DefaultRenderer(IRoomRenderer):
    def get_tiles(self, room: Room) -> List[List[Tile]]:
        arr: List[List[Tile]] = [[Tile.Floor for j in range(room._width)] for i in range(room._height)]
        return arr


class DiamondRenderer(IRoomRenderer):
    def get_tiles(self, room: Room) -> List[List[Tile]]:
        offset: int = 0
        y_middle = room._width // 2
        arr: List[List[Tile]] = [[Tile.Empty for j in range(room._width)] for i in range(room._height)]

        for x in range(room._height):
            for y in range(y_middle - offset, y_middle + offset + 1):
                arr[x][y] = Tile.Floor
            offset = offset + 1 if x < room._height // 2 else offset - 1
        return arr


class CrossRenderer(IRoomRenderer):
    def get_tiles(self, room: Room) -> List[List[Tile]]:
        arr: List[List[Tile]] = [[Tile.Empty for j in range(room._width)] for i in range(room._height)]
        for x in range(room._height):
            for y in range(room._width):
                x_lower_bound: float = room._height * 0.333 - 1
                x_upper_bound: float = room._height * 0.666

                y_lower_bound: float = room._width * 0.333 - 1
                y_upper_bound: float = room._width * 0.666
                if (x > x_lower_bound and x < x_upper_bound) or (y > y_lower_bound and y < y_upper_bound):
                    arr[x][y] = Tile.Floor
                else:
                    arr[x][y] = Tile.Empty
        return arr
