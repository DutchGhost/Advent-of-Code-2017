#git ls-files | grep rs | xargs wc -l
import os
from subprocess import run
if __name__ == '__main__':
    for directory in os.listdir(os.getcwd()):
        if directory != 'compile.py':
            print("going to compile {}".format(directory))
            os.chdir(directory)
            run(["cargo", "clean"])
            run(["cargo", "update"])
            run(["cargo", "rustc", "--release", "--", "-C", "target-cpu=native", "-C", "lto"])
            os.chdir('..')