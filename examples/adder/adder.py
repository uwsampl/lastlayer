import numpy as np
from device import Device


def adder(lib):
    dev = Device(lib) # lastlayer generated
    dev.reset(10) # reset for 10 cycles

    n = 16
    maxv = 64

    a = np.random.randint(maxv, size=n, dtype="uint8")
    b = np.random.randint(maxv, size=n, dtype="uint8")
    c = np.zeros(n, dtype="uint8")

    # [hardware op] y = a + b
    for i, d in enumerate(zip(a, b)):
        dev.write_reg_a(d[0])
        dev.write_reg_b(d[1])
        dev.run(3) # run for 3 cycles
        c[i] = dev.read_reg_y()

    y = np.add(a, b)
    np.testing.assert_array_equal(c, y)