def solve(input, n):
    return sum(map(lambda i_c: int(i_c[1]), filter(lambda i_c: i_c[1] == input[(int(i_c[0]) + n) % len(input)], enumerate(input))))
if __name__ == '__main__':
    with open('Input.txt', 'r') as f:
        s = f.readline()
        print("part 1: {}".format(solve(s, 1)))
        print("part 2: {}".format(solve(s, len(s) // 2)))
    print("part 1: {}\npart2: {}".format(sum(map(lambda i_c: int(i_c[1]), filter(lambda i_c: i_c[1] == open("Input.txt", 'r').readline()[(int(i_c[0]) + 1) % len(open("Input.txt", 'r').readline())], enumerate(open("Input.txt", 'r').readline())))), sum(map(lambda i_c: int(i_c[1]), filter(lambda i_c: i_c[1] == open("Input.txt", 'r').readline()[(int(i_c[0]) + int(len(open("Input.txt", 'r').readline()) / 2 )) % len(open("Input.txt", 'r').readline())], enumerate(open("Input.txt", 'r').readline()))))))