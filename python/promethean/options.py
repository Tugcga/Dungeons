from enum import IntEnum, Enum
from dataclasses import dataclass, field
from typing import List, Tuple

from promethean.point import Point


class RoomType(IntEnum):
    Square = 0
    Rectangle = 1
    Cross = 2
    Diamond = 3


class PathFinderTile(IntEnum):
    Pathable = 1
    Blocked = 0


class Tile(IntEnum):
    Empty = 1
    Floor = 0
    TopLeftInsideCorner = 2
    TopRightInsideCorner = 3
    BottomLeftInsideCorner = 4
    BottomRightInsideCorner = 5
    TopWall = 6
    RightWall = 7
    BottomWall = 8
    LeftWall = 9
    TopLeftOutsideCorner = 10
    TopRightOutsideCorner = 11
    BottomLeftOutsideCorner = 12
    BottomRightOutsideCorner = 13


class TileMask(Enum):
    Wild = None
    Open = 1
    Block = 0


@dataclass
class TilePoint:
    position: Point = Point()
    tile_type: Tile = Tile.Empty


@dataclass
class PathFinderOptions:
    search_limit: int = 2000


@dataclass
class Options:
    level_width: int = 64
    level_height: int = 64
    min_room_width: int = 5
    max_room_width: int = 7
    min_room_height: int = 5
    max_room_height: int = 7
    number_of_rooms: int = 45
    random_seed: int = 1
    border: int = 1
    room_border: int = 1
    overlap_rooms: bool = False
    room_types: List[RoomType] = field(default_factory=lambda: [RoomType.Square, RoomType.Rectangle, RoomType.Cross, RoomType.Diamond])
