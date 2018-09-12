from enum import Enum

INPUT = 361527

class Direction(Enum):
    UP = 0
    DOWN = 1
    RIGHT = 2
    LEFT = 3

    '''Turn to the right'''
    def turn_right(self):
        return {
            Direction.UP      : Direction.RIGHT,
            Direction.RIGHT   : Direction.DOWN,
            Direction.DOWN    : Direction.LEFT,
            Direction.LEFT    : Direction.UP,
        }[self]

    '''Turn to the left'''
    def turn_left(self):
        return {
            Direction.UP      : Direction.LEFT,
            Direction.LEFT    : Direction.DOWN,
            Direction.DOWN    : Direction.RIGHT,
            Direction.RIGHT   : Direction.UP,
        }[self]

class Position():
    def __init__(self):
        self.x = 0
        self.y = 0

    '''Y is decremented when going up, incremented when going down'''
    def change(self, direction, n):
        if direction == Direction.UP:
            self.y -= n
        elif direction == Direction.DOWN:
            self.y += n
        elif direction == Direction.RIGHT:
            self.x += n
        elif direction == Direction.LEFT:
            self.x -= n

    '''Y is incremented when going up, decremented when going down'''
    def rev_change(self, direction, n):
        self.change(direction, -n)

    '''Transforms self into a tuple'''
    def to_tuple(self):
        return (self.x, self.y)


class Spiral():
    def __init__(self):
        self.direction = Direction.RIGHT
        self.position = Position()

    '''Lazy Iterator, yielding positions of a sprial, spiraling to the right'''
    def spiral(self):
        number_of_steps = 1
        while True:
            for _ in range(2):
                for _ in range(1, number_of_steps):

                    yield self.position.to_tuple()
                    self.position.rev_change(self.direction, 1)

                self.direction = self.direction.turn_left()
            number_of_steps += 1

    '''Resets the direction and position, to start over'''
    def reset(self):
        self.direction = Direction.RIGHT
        self.position = Position()

class Solver():
    def __init__(self):
        self.spiralizer = Spiral()
        self.storage = [(1, (0, 0))]

    def part1(self):
        (x, y) = next((x, y) for (value, (x, y)) in enumerate(self.spiralizer.spiral(), start = 1) if value == INPUT)
        return abs(x) + abs(y)

    def reset(self):
        self.spiralizer.reset()

    '''Loop over the (value, (x, y)) in self.storage, take the sum of all values where (x, y)
        are adjecent to position.
        add (s, position) to the storage.
    '''
    def adjecents(self, position):
        (pos_x, pos_y) = position
        valids = [(0, 1), (1, 0), (1, 1)]

        s = sum(value for (value, (x, y)) in self.storage if (abs(pos_x - x), abs(pos_y - y)) in valids)

        self.storage.append((s, position))
        return s

    def part2(self):
        return next(val for val in map(self.adjecents, self.spiralizer.spiral()) if val > INPUT)

if __name__ == '__main__':
    solver = Solver()

    print("Part 1: {}".format(solver.part1()))
    solver.reset()
    print("Part 2: {}".format(solver.part2()))
