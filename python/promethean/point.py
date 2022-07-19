
class Point:
    def __init__(self, x: int = 0, y: int = 0):
        self._x = x
        self._y = y

    def __eq__(self, other) -> bool:
        return self._x == other._x and self._y == other._y

    def __ne__(self, other) -> bool:
        return self._x != other._x or self._y != other._y

    def __repr__(self) -> str:
        return "[" + str(self._x) + "." + str(self._y) + "]"

    def get_hash_code(self) -> int:
        value: int = 17
        value = value * 23 + self._x
        value = value * 23 + self._y
        return value
