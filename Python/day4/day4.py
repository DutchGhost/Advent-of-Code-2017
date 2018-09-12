import re

def group(iter):
    return map(lambda item: item.group(1), iter)

def clamp(line):
    return ''.join(sorted(line))

def compared(matches):
    lst = list(map(clamp, matches))

    return len(set(lst)) == len(lst)

if __name__ == '__main__':
    matcher = re.compile(r'(\w+)')

    with open("Input.txt") as f:

        print(sum(1 for _ in filter(compared, map(group, map(matcher.finditer, f)))))
