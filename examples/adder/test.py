import os
import sys
from adder import adder

if __name__ == "__main__":
    adder_dir =  os.path.dirname(os.path.realpath(__file__))
    adder_lib = os.path.join(adder_dir, "build", "libadder.so")
    sys.path.append(adder_dir)
    adder(adder_lib)