import numpy as np
import torch
from time import perf_counter_ns
import os
import sys
import argparse
from device import Device

def relu(lib, n, num_vec_words):
    minv = -64
    maxv = 64
    max_cycle = 1000000
    a = np.random.randint(minv, maxv, size=n, dtype="int8")
    x = torch.tensor(a, dtype=torch.int8)
    dev = Device(lib, n, num_vec_words)
    dev.reset(3)
    dev.set_raddr()
    dev.set_waddr()
    dev.set_length()
    dev.write_mem(x)
    dev.launch()
    start = perf_counter_ns()
    dev.run(max_cycle)
    stop = perf_counter_ns()
    finish = dev.finish()
    z = x.clamp(min=0)
    assert finish == 1, "Relu accelerator did not finish"
    y = dev.read_mem(n)
    cycle_counter = dev.get_cycle_counter()
    assert torch.all(torch.eq(y, z)), "Relu fail, mismatch"
    elapsed = stop-start
    return cycle_counter, elapsed

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--num-vec-words", type=int, required=True, help="number of vector words")
    args = parser.parse_args()
    relu_dir =  os.path.dirname(os.path.realpath(__file__))
    sys.path.append(relu_dir)
    n = 16384
    dir = "relu_{}".format(args.num_vec_words)
    lib = "librelu_{}.so".format(args.num_vec_words)
    relu_lib = os.path.join(relu_dir, dir, lib)
    vlen = []
    cycles = []
    etime = []
    c, e = relu(relu_lib, n, args.num_vec_words)
    vlen = args.num_vec_words * 4
    print("results:{},{},{}".format(vlen, c, e/1000000))