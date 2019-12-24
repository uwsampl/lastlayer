import numpy as np
import torch
from time import perf_counter_ns
from device import Device

n = 16384
minv = -64
maxv = 64
max_cycle = 1000000
num_vec_words = 2

a = np.random.randint(minv, maxv, size=n, dtype="int8")
x = torch.tensor(a, dtype=torch.int8)

dev = Device("build/librelu.so", n, num_vec_words)
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

if finish:
    y = dev.read_mem(n)
    cycle_counter = dev.get_cycle_counter()
    if torch.all(torch.eq(y, z)):
        print("status: PASS")
        print("cycles:", cycle_counter)
        print("Time:", stop-start)
    else:
        print("status: FAIL")
        print("input:", x)
        print("output:", y)

else:
    print("Still not finished in {} cycles".format(cycle_counter))