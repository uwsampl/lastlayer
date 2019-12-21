from device import Device

dev = Device("build/librelu.so")
dev.reset(10)
dev.run(10)
