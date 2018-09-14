import re
from functools import partial
def formula(a, t, v, p):
    return a * t * t / 2 + (v + a / 2 ) * t + p

def distances(matcher, fname):
    
    time = 10000

    with open(fname) as f:

        for line in f:
            matched = matcher.findall(line)

            p, v, a = matched

            convert = lambda iter: map(int, iter)

            x_pos, y_pos, z_pos = convert(p)
            x_vel, y_vel, z_vel = convert(v)
            x_acc, y_acc, z_acc = convert(a)

            x_speed = formula(x_acc, time, x_vel, x_pos)
            y_speed = formula(y_acc, time, y_vel, y_pos)
            z_speed = formula(z_acc, time, z_vel, z_pos)


            yield abs(x_speed) + abs(y_speed) + abs(z_speed)

if __name__ == '__main__':
    matcher = re.compile(r"(-?\d+),(-?\d+),(-?\d+)")

    print(min(enumerate(distances(matcher, "Input.txt")), key = lambda position: position[1])[0])
