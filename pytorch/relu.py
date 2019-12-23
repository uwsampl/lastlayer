import numpy as np
import torch
from device import Device

n = 8
minv = -64
maxv = 64

nx = np.random.randint(minv, maxv, size=n, dtype="int8")
tx = torch.tensor(nx, dtype=torch.int8)

dev = Device("build/librelu.so")

dev.reset(3)
dev.set_raddr(0)
dev.set_waddr(0)
dev.set_length(n)
dev.write_mem(0, tx)
dev.launch()
dev.run(20)
finish = dev.finish()
ty = dev.read_mem(0, n)

print("finish:", finish)
print("input:", tx)
print("output:", ty)
