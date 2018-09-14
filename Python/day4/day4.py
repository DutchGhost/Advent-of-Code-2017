import re
from collections import Counter

def group(iter):
    return map(lambda item: item.group(1), iter)

def clamp(line):
    return ''.join(sorted(line))

def compared(matches):
    lst = list(map(clamp, matches))

    return len(set(lst)) == len(lst)

def check_counter(counter):
    return counter.most_common(1)[0][1] == 1
    
if __name__ == '__main__':
    matcher = re.compile(r'(\w+)')

    with open("Input.txt") as f:
        lines = f.readlines()
        
    print(sum(1 for _ in filter(check_counter, map(Counter, map(group, map(matcher.finditer, lines))))))
    print(sum(1 for _ in filter(compared, map(group, map(matcher.finditer, lines)))))