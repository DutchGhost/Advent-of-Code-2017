def solve(input, n):
    return sum(
                map(lambda i_c: int(i_c[1]),
                    filter(lambda i_c: i_c[1] == input[(int(i_c[0]) + n) % len(input)],
                            enumerate(input)
                        )
                )
            )
if __name__ == '__main__':
    with open('Input.txt', 'r') as f:
        s = f.readline()
        print("part 1: {}".format(solve(s, 1)))
        print("part 2: {}".format(solve(s, len(s) // 2)))