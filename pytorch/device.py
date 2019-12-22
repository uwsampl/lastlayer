import torch

class Device:
    def __init__(self, lib):
        torch.ops.load_library(lib)
        self.handle = torch.ops.device.alloc()
        self.num_word = 1
        self.sel = 0  # select byte-0
        self.raddr_hid = 0
        self.waddr_hid = 1
        self.rmem_hid = 0
        self.wmem_hid = 1
    def __del__(self):
        torch.ops.device.dealloc(self.handle)
    def read_raddr(self):
        return torch.ops.device.read_reg(self.handle, self.raddr_hid, self.sel)
    def write_raddr(self, value):
        torch.ops.device.write_reg(self.handle, self.raddr_hid, self.sel, value)
    def read_waddr(self):
        return torch.ops.device.read_reg(self.handle, self.waddr_hid, self.sel)
    def write_waddr(self, value):
        torch.ops.device.write_reg(self.handle, self.waddr_hid, self.sel, value)
    def read_rmem(self, start_addr, num_elem):
        return torch.ops.device.read_mem(self.handle, self.rmem_hid, start_addr, self.num_word, num_elem)
    def write_rmem(self, start_addr, input):
        torch.ops.device.write_mem(self.handle, self.rmem_hid, start_addr, self.num_word, input)
    def read_wmem(self, start_addr, num_elem):
        return torch.ops.device.read_mem(self.handle, self.wmem_hid, start_addr, self.num_word, num_elem)
    def write_wmem(self, start_addr, input):
        torch.ops.device.write_mem(self.handle, self.wmem_hid, start_addr, self.num_word, input)
    def reset(self, cycles):
        torch.ops.device.reset(self.handle, cycles)
    def run(self, cycles):
        torch.ops.device.run(self.handle, cycles)