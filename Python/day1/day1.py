import re
def solve(captcha, n):
    return sum(map(int, re.findall(fr'(\d)(?=.{{{n-1}}}\1)', captcha + captcha[:n])))

if __name__ == '__main__':
    with open('Input.txt', 'r') as f:
        s = f.readline()
        print (solve(s, 1))
        print (solve(s, len(s) // 2))
