import random

class PseudoRandom:
    def __init__(self, in_seed: int):
        self._random = random.Random(in_seed)

    def next(self, in_min: int, in_max: int) -> int:
        return self._random.randint(in_min, in_max)

    def next_odd(self, in_min: int, in_max: int) -> int:
        next_value: int = self.next(in_min, in_max)

        if next_value % 2 != 0:
            return next_value
        else:
            if next_value < in_max:
                return next_value + 1
            else:
                return next_value - 1
