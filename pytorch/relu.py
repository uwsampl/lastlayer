import numpy as np
import torch
from device import Device

n = 8
minv = -64
maxv = 64

dev = Device("build/librelu.so")
nx = np.random.randint(minv, maxv, size=n, dtype="int8")
tx = torch.tensor(nx, dtype=torch.int8)
print(tx)
dev.write_mem(0, 0, tx)
ty = dev.read_mem(0, 0, tx.numel())
print(ty)
# dev.reset(10)
# dev.write_reg(0, 0, -3)
# dev.run(20)
# print("result:{}".format(dev.read_reg(1, 0)))
