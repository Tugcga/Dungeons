from typing import List, Optional

from promethean.pseudo_random import PseudoRandom
from promethean.options import RoomType, Tile, Options
from promethean.point import Point


def determine_max_position(level_dimension: int, room_dimension: int, border: int) -> int:
    return level_dimension - room_dimension - border

class Room:
    def __init__(self, room_height: int, room_width: int, room_x: int, room_y: int, room_type: RoomType=RoomType.Rectangle):
        self._height: int = room_height
        self._width: int = room_width
        self._position = Point(room_x, room_y)
        self._room_center = Point(self._position._x + self._height // 2, self._position._y + self._width // 2)
        self._bottom_left = Point(self._position._x + self._height - 1, self._position._y)
        self._top_right = Point(self._position._x, self._position._y + self._width - 1)
        self._bottom_right = Point(self._position._x + self._height - 1, self._position._y + self._width - 1)
        self._room_type = room_type

    def intersects(self, other, buffer: int) -> bool:
        if self._bottom_right._y + buffer < other._position._y - buffer or other._bottom_right._y + buffer < self._position._y - buffer:
            return False

        if self._bottom_right._x + buffer < other._position._x - buffer or other._bottom_right._x + buffer < self._position._x - buffer:
            return False

        return True

    def __repr__(self) -> str:
        return "Room" + str(self._position) + ":" + str(self._width) + ":" + str(self._height)


class RoomGenerator:
    def __init__(self, in_random: PseudoRandom):
        self._random = in_random

    def generate_rooms(self, options: Options) -> List[Room]:
        if options.overlap_rooms:
            return self._generate_overlapping_rooms(options)
        else:
            return self._generate_non_overlapping_rooms(options)

    def _generate_overlapping_rooms(self, options: Options) -> List[Room]:
        rooms: List[Room] = []
        for room_count in range(0, options.number_of_rooms):
            new_room: Optional[Room] = self._generate(options)
            if new_room is not None:
                rooms.append(new_room)
        return rooms

    def _generate_non_overlapping_rooms(self, options: Options) -> List[Room]:
        rooms: List[Room] = []
        for room_count in range(0, options.number_of_rooms):
            new_room: Optional[Room] = self._generate(options)
            if new_room is not None:
                if self._is_intersections(rooms, new_room, options):
                    repositioned_room: Optional[Room] = self._reposition(rooms, new_room, options)
                    if repositioned_room is None:
                        return rooms

                    new_room = repositioned_room
                rooms.append(new_room)

        return rooms

    def _reposition(self, rooms: List[Room], room: Room, options: Options) -> Optional[Room]:
        min_x: int = options.border
        min_y: int = options.border
        max_x: int = determine_max_position(options.level_height, room._height, options.border)
        max_y: int = determine_max_position(options.level_width, room._width, options.border)

        lower_bound = Point(min_x, min_y)
        upper_bound = Point(max_x, max_y)

        offset: int  = 0
        is_finish: bool = False
        is_find: bool = False
        is_switch_direction: bool = False
        top_left: Point = Point()
        top_right: Point = Point()
        bottom_right: Point = Point()
        bottom_left: Point = Point()
        direction: int = -1
        direction_iterator: int = top_left._y
        position: Optional[Point] = None

        while not is_finish:
            # calculate next point for position
            # at start position is None
            position = None
            # this point may be from one of four directions
            is_switch_direction = False
            if direction == -1:
                is_switch_direction = True
                is_find = False
            elif direction == 0:
                is_find = False
                if top_left._x >= lower_bound._x:
                    while direction_iterator <= top_right._y and direction_iterator <= upper_bound._y:
                        if direction_iterator < lower_bound._y:
                            direction_iterator += 1
                        else:
                            position = Point(top_left._x, direction_iterator)
                            direction_iterator += 1
                            is_find = True
                            break
                    if not is_find:
                        # switch direction
                        is_switch_direction = True
                else:
                    is_switch_direction = True
            elif direction == 1:
                is_find = False
                if top_right._y <= upper_bound._y:
                    while direction_iterator < bottom_right._x and direction_iterator <= upper_bound._x:
                        if direction_iterator < lower_bound._x:
                            direction_iterator += 1
                        else:
                            position = Point(direction_iterator, top_right._y)
                            direction_iterator += 1
                            is_find = True
                            break
                    if not is_find:
                        is_switch_direction = True
                else:
                    is_switch_direction = True
            elif direction == 2:
                is_find = False
                if bottom_right._x <= upper_bound._x:
                    while direction_iterator >= bottom_left._y and direction_iterator >= lower_bound._y:
                        if direction_iterator > upper_bound._y:
                            direction_iterator -= 1
                        else:
                            position = Point(bottom_right._x, direction_iterator)
                            direction_iterator -= 1
                            is_find = True
                            break
                    if not is_find:
                        is_switch_direction = True
                else:
                    is_switch_direction = True
            elif direction == 3:
                is_find = False
                if bottom_left._y >= lower_bound._y:
                    while direction_iterator > top_left._x and direction_iterator >= lower_bound._x + 1:
                        if direction_iterator > upper_bound._x:
                            direction_iterator -= 1
                        else:
                            position = Point(direction_iterator, bottom_left._y)
                            direction_iterator -= 1
                            is_find = True
                            break
                    if not is_find:
                        is_switch_direction = True
                else:
                    is_switch_direction = True
            if is_switch_direction:
                if direction == 0:
                    direction = 1
                    direction_iterator = top_right._x + 1
                elif direction == 1:
                    direction = 2
                    direction_iterator = bottom_right._y
                elif direction == 2:
                    direction = 3
                    direction_iterator = bottom_left._x - 1
                else:
                    offset += 1
                    top_left = Point(room._position._x - offset, room._position._y - offset)
                    top_right = Point(room._position._x - offset, room._position._y + offset)
                    bottom_right = Point(room._position._x + offset, room._position._y + offset)
                    bottom_left = Point(room._position._x + offset, room._position._y - offset)

                    if room._position._x - offset < lower_bound._x and room._position._x + offset > upper_bound._x and room._position._y - offset < lower_bound._y and room._position._y + offset > upper_bound._y:
                        break
                    else:
                        direction = 0
                        direction_iterator = top_left._y
            else:
                # this means that is_find = True
                # process the positions
                if position is not None:
                    new_room_candidate = Room(room._height, room._width, position._x, position._y, room._room_type)
                    if not self._is_intersections(rooms, new_room_candidate, options):
                        return new_room_candidate
                else:
                    break
        return None

    def _is_intersections(self, rooms: List[Room], target: Room, options: Options) -> bool:
        for i, r in enumerate(rooms):
            if target.intersects(r, options.room_border):
                return True
        return False

    def _generate(self, options: Options) -> Optional[Room]:
        room_type: RoomType = options.room_types[self._random.next(1, len(options.room_types)) - 1]
        room_width = self._random.next_odd(options.min_room_width, options.max_room_width)
        room_height = self._random.next_odd(options.min_room_height, options.max_room_height) if room_type == RoomType.Rectangle else room_width
        max_x = determine_max_position(options.level_height, room_height, options.border)
        max_y = determine_max_position(options.level_width, room_width, options.border)
        if max_x >= options.border and max_y >= options.border:
            room_x = self._random.next(options.border, max_x)
            room_y = self._random.next(options.border, max_y)
            room = Room(room_height, room_width, room_x, room_y, room_type)

            return room
        else:
            return None
