from device import Device

dev = Device("build/libadder.so")
dev.reset(10)
dev.run(10)