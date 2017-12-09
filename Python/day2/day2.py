from itertools import islice
def parse():
        return [[int(word) for word in row.strip().split("\t")] for row in open('Input.txt', 'r').readlines()]

def part2(row):
    for (idx, n) in enumerate(row):
        for n2 in islice(row, idx):
            [num1, num2] = list(reversed(sorted([n, n2])))
            if num1 % num2 == 0:
                return num1 // num2
            
if __name__ == '__main__':
    parsed = parse()
    print("part 1: {}".format(sum(map(lambda row: max(row) - min(row), parsed))))
    print("part 2: {}".format(sum(map(lambda row: part2(row), parsed))))