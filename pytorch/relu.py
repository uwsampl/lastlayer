from device import Device

dev = Device("build/librelu.so")
dev.reset(10)
dev.write_reg(0, 0, -3)
dev.run(20)
print("result:{}".format(dev.read_reg(1, 0)))
