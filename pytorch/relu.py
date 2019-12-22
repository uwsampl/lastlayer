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
dev.write_rmem(0, tx)
dev.write_raddr(1)
dev.write_waddr(3)
dev.run(5)
ty = dev.read_wmem(0, tx.numel())
print("input:", tx)
print("output:", ty)
