import numpy as np
import torch
from device import Device

dev = Device("build/librelu.so")
x = torch.tensor(np.array([[1, 2, 3], [4, 5, 6]]), dtype=torch.int8)
print(x)
dev.write_mem(0, 0, 1, x)
# dev.reset(10)
# dev.write_reg(0, 0, -3)
# dev.run(20)
# print("result:{}".format(dev.read_reg(1, 0)))
