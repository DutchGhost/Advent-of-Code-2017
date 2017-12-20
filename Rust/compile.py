import os
from subprocess import run
if __name__ == '__main__':
    for d in os.listdir(os.getcwd()):
        if d != 'compile.py':
            os.chdir(r"{}".format(d))
            run(["cargo", "rustc", "--release", "--", "-C", "target-cpu=native", "-C", "lto"])
            os.chdir('..')
