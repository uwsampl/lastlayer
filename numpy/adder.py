import numpy as np
from device import Device

dev = Device("build/libadder.so") # lastlayer generated
dev.reset(10) # reset for 10 cycles

N = 16
MAX = 64

a = np.random.randint(MAX, size=N, dtype="uint8")
b = np.random.randint(MAX, size=N, dtype="uint8")
c = np.zeros(N, dtype="uint8")

for i, d in enumerate(zip(a, b)):
    dev.write_reg(0, d[0]) # hid=0
    dev.write_reg(1, d[1]) # hid=1
    dev.run(3) # run for 3 cycles
    c[i] = dev.read_reg(2) # hid=2

y = np.add(a, b)
np.testing.assert_array_equal(c, y)