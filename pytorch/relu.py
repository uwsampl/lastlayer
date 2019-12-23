import numpy as np
import torch
from time import perf_counter_ns
from device import Device

n = 16384
minv = -64
maxv = 64
max_cycle = 1000000

nx = np.random.randint(minv, maxv, size=n, dtype="int8")
tx = torch.tensor(nx, dtype=torch.int8)

dev = Device("build/librelu.so")
dev.reset(3)
dev.set_raddr(0)
dev.set_waddr(0)
dev.set_length(n)
dev.write_mem(0, tx)
dev.launch()
t1_start = perf_counter_ns()
dev.run(max_cycle)
t1_stop = perf_counter_ns()
finish = dev.finish()
tz = tx.clamp(min=0)

if finish:
    ty = dev.read_mem(0, n)
    cycle_counter = dev.get_cycle_counter()
    if torch.all(torch.eq(ty, tz)):
        print("status: PASS")
        print("cycles:", cycle_counter)
        print("Time:", t1_stop-t1_start)
    else:
        print("status: FAIL")
else:
    print("Still not finished in {} cycles".format(cycle_counter))