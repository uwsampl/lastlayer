from device import Device

dev = Device("build/libadder.so")
dev.reset(10)
dev.write(0, 0, 3)
dev.run(10)
print(dev.read(0, 0))