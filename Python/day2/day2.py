from itertools import islice
from functools import partial
import re

def clamp(xs):
    return max(xs) - min(xs)

def yank(match):
    return int(match.group(1))

def convert(line, reg):
    return map(yank, reg.finditer(line))

def deviding(xs):
    for (idx, n) in enumerate(xs):
        for n2 in islice(xs, idx):
            [num1, num2] = list(reversed(sorted([n, n2])))

            if num1 % num2 == 0:
                return num1 // num2

if __name__ == '__main__':
    matcher = re.compile(r'(\d+)')
    parser = partial(convert, reg = matcher)

    with open("Input.txt", 'r') as f:
        lines = f.readlines()
        print(sum(map(clamp, map(list, map(parser, lines)))))
        print(sum(map(deviding, map(list, map(parser, lines)))))
