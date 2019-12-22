import numpy as np
import torch
from device import Device

N = 8
MIN = -64
MAX = 64

dev = Device("build/librelu.so")
np_x = np.random.randint(MIN, MAX, size=N, dtype="int8")
torch_x = torch.tensor(np_x, dtype=torch.int8)
print(torch_x)
dev.write_mem(0, 0, 1, torch_x)
# dev.reset(10)
# dev.write_reg(0, 0, -3)
# dev.run(20)
# print("result:{}".format(dev.read_reg(1, 0)))
