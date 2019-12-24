import numpy as np
import torch
from time import perf_counter_ns
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
    assert finish == int(1), "Relu accelerator did not finish"
    y = dev.read_mem(n)
    cycle_counter = dev.get_cycle_counter()
    assert torch.all(torch.eq(y, z)), "Relu fail, mismatch"
    elapsed = stop-start
    return cycle_counter, elapsed