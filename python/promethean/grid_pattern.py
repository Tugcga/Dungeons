from typing import List
from dataclasses import dataclass

from promethean.options import TileMask, Tile, TilePoint
from promethean.point import Point


class GridPattern:
    def __init__(self, name: str, pattern: List[List[TileMask]], paint_offsets: List[TilePoint]):
        self._name = name
        self._pattern = pattern
        self._paint_offsets = paint_offsets


top_left = Point(-1, -1)
top = Point(-1, 0)
top_right = Point(-1, 1)
left = Point(0, -1)
current = Point(0, 0)
right = Point(0, 1)
bottom_left = Point(1, -1)
bottom = Point(1, 0)
bottom_right = Point(1, 1)

grid_patterns: List[GridPattern] = [
    GridPattern("Top Left Inside Corner", 
                [[TileMask.Open, TileMask.Open, TileMask.Open], 
                [TileMask.Open, TileMask.Block, TileMask.Block], 
                [TileMask.Wild, TileMask.Block, TileMask.Block]],
                [TilePoint(top_left, Tile.TopLeftInsideCorner), 
                 TilePoint(top, Tile.TopWall), 
                 TilePoint(left, Tile.LeftWall)]),
    GridPattern("Top Right Inside Corner", 
                [[TileMask.Open, TileMask.Open, TileMask.Wild], 
                [TileMask.Block, TileMask.Block, TileMask.Open], 
                [TileMask.Block, TileMask.Block, TileMask.Wild]],
                [TilePoint(top_right, Tile.TopRightInsideCorner), 
                 TilePoint(top, Tile.TopWall), 
                 TilePoint(right, Tile.RightWall)]),
    GridPattern("Bottom Left Inside Corner", 
                [[TileMask.Open, TileMask.Block, TileMask.Block], 
                [TileMask.Open, TileMask.Block, TileMask.Block], 
                [TileMask.Wild, TileMask.Open, TileMask.Wild]],
                [TilePoint(bottom_left, Tile.BottomLeftInsideCorner), 
                 TilePoint(bottom, Tile.BottomWall), 
                 TilePoint(left, Tile.LeftWall)]),
    GridPattern("Bottom Right Inside Corner", 
                [[TileMask.Block, TileMask.Block, TileMask.Open], 
                [TileMask.Block, TileMask.Block, TileMask.Open], 
                [TileMask.Wild, TileMask.Open, TileMask.Open]],
                [TilePoint(bottom_right, Tile.BottomRightInsideCorner), 
                 TilePoint(bottom, Tile.BottomWall), 
                 TilePoint(right, Tile.RightWall)]),
    GridPattern("Top Wall", 
                [[TileMask.Open, TileMask.Open, TileMask.Open], 
                [TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Block, TileMask.Block, TileMask.Block]],
                [TilePoint(top, Tile.TopWall)]),
    GridPattern("Bottom Wall", 
                [[TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Open, TileMask.Open, TileMask.Open]],
                [TilePoint(bottom, Tile.BottomWall)]),
    GridPattern("Left Wall", 
                [[TileMask.Open, TileMask.Block, TileMask.Block], 
                [TileMask.Open, TileMask.Block, TileMask.Block], 
                [TileMask.Open, TileMask.Block, TileMask.Block]],
                [TilePoint(left, Tile.LeftWall)]),
    GridPattern("Right Wall", 
                [[TileMask.Block, TileMask.Block, TileMask.Open], 
                [TileMask.Block, TileMask.Block, TileMask.Open], 
                [TileMask.Block, TileMask.Block, TileMask.Open]],
                [TilePoint(right, Tile.RightWall)]),
    GridPattern("Bottom Left Outside Wall", 
                [[TileMask.Block, TileMask.Open, TileMask.Open], 
                [TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Block, TileMask.Block, TileMask.Block]],
                [TilePoint(top, Tile.BottomLeftOutsideCorner)]),
    GridPattern("Bottom Right Outside Wall", 
                [[TileMask.Open, TileMask.Open, TileMask.Block], 
                [TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Block, TileMask.Block, TileMask.Block]],
                [TilePoint(top, Tile.BottomRightOutsideCorner)]),
    GridPattern("Top Right Outside Wall", 
                [[TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Open, TileMask.Open, TileMask.Block]],
                [TilePoint(bottom, Tile.TopRightOutsideCorner)]),
    GridPattern("Top Left Outside Wall", 
                [[TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Block, TileMask.Block, TileMask.Block], 
                [TileMask.Block, TileMask.Open, TileMask.Open]],
                [TilePoint(bottom, Tile.TopLeftOutsideCorner)]),
    GridPattern("Top Right Inside For Touching", 
                [[TileMask.Block, TileMask.Open, TileMask.Open], 
                [TileMask.Open, TileMask.Block, TileMask.Block], 
                [TileMask.Open, TileMask.Block, TileMask.Block]],
                [TilePoint(top, Tile.BottomLeftOutsideCorner)]),
    GridPattern("Bottom Right Inside For Touching", 
                [[TileMask.Block, TileMask.Block, TileMask.Open], 
                [TileMask.Block, TileMask.Block, TileMask.Open], 
                [TileMask.Open, TileMask.Open, TileMask.Block]],
                [TilePoint(bottom, Tile.TopRightOutsideCorner)])]
