import torch

class Device:
    def __init__(self, lib):
        torch.ops.load_library(lib)
        self.handle = torch.ops.device.alloc()
        self.wordsize = 1
    def __del__(self):
        torch.ops.device.dealloc(self.handle)
    def read_reg(self, hid, sel):
        return torch.ops.device.read_reg(self.handle, hid, sel)
    def write_reg(self, hid, sel, value):
        torch.ops.device.write_reg(self.handle, hid, sel, value)
    def read_mem(self, hid, addr, numel):
        return torch.ops.device.read_mem(self.handle, hid, addr, self.wordsize, numel)
    def write_mem(self, hid, addr, input):
        torch.ops.device.write_mem(self.handle, hid, addr, self.wordsize, input)
    def reset(self, cycles):
        torch.ops.device.reset(self.handle, cycles)
    def run(self, cycles):
        torch.ops.device.run(self.handle, cycles)