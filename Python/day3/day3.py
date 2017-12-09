def spiral():
    number_of_steps = 1
    x = 0
    y = 0
    value = 1
    direction = 1
    while True:
        for _ in range(2):
            for must_change in range(number_of_steps):
                yield (value, x, y)
                #if we go up
                if direction % 4 == 0:
                    y += 1
                #right
                elif direction % 4 == 1:
                    x += 1
                #down
                elif direction % 4 == 2:
                    y -= 1
                #left
                else:
                    x -= 1
                
                value += 1
                if must_change == number_of_steps - 1:
                    direction -= 1
        number_of_steps += 1
            


if __name__ == '__main__':
    inp = 361527
    for (v, x, y) in spiral():
        
        if v == inp:
            print("part 1: {}".format(abs(x) + abs(y)))
            break