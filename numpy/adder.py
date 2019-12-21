from device import Device

dev = Device("build/libadder.so")
dev.reset(20)
# dev.write(0, 0, 3)
dev.run(100)
# dev.run(100)
print(dev.read(0, 0))