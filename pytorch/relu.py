import numpy as np
import torch
from device import Device

n = 8
minv = -64
maxv = 64
max_cycle = 1000

nx = np.random.randint(minv, maxv, size=n, dtype="int8")
tx = torch.tensor(nx, dtype=torch.int8)

dev = Device("build/librelu.so")

dev.reset(3)
dev.set_raddr(0)
dev.set_waddr(0)
dev.set_length(n)
dev.write_mem(0, tx)
dev.launch()
dev.run(max_cycle)
finish = dev.finish()

if finish:
    ty = dev.read_mem(0, n)
    cycle_counter = dev.get_cycle_counter()
    print("finish:", finish)
    print("cycles:", cycle_counter)
    print("input:", tx)
    print("output:", ty)
else:
    print("Still not finished in {} cycles".format(cycle_counter))