import torch

class Device:

    def __init__(self, lib, num_array_bytes, num_vec_words):
        torch.ops.load_library(lib)
        self.vec_word_bytes = 4
        self.handle = torch.ops.device.alloc()
        self.length = int(num_array_bytes / (num_vec_words * self.vec_word_bytes))
        self.num_vec_words = num_vec_words
        self.start_addr = 0
        self.sel = 0
        self.raddr_id = 0
        self.waddr_id = 1
        self.launch_id = 2
        self.finish_id = 3
        self.length_id = 4
        self.cycle_id = 5
        self.rmem_id = 0
        self.wmem_id = 1

    def __del__(self):
        torch.ops.device.dealloc(self.handle)

    def get_raddr(self):
        return torch.ops.device.read_reg(self.handle, self.raddr_id, self.sel)

    def set_raddr(self):
        torch.ops.device.write_reg(self.handle, self.raddr_id, self.sel, self.start_addr)

    def get_waddr(self):
        return torch.ops.device.read_reg(self.handle, self.waddr_id, self.sel)

    def set_waddr(self):
        torch.ops.device.write_reg(self.handle, self.waddr_id, self.sel, self.start_addr)

    def launch(self):
        torch.ops.device.write_reg(self.handle, self.launch_id, self.sel, 1)

    def finish(self):
        return torch.ops.device.read_reg(self.handle, self.finish_id, self.sel)

    def get_length(self):
        return torch.ops.device.read_reg(self.handle, self.length_id, self.sel)

    def set_length(self):
        torch.ops.device.write_reg(self.handle, self.length_id, self.sel, self.length)

    def get_cycle_counter(self):
        return torch.ops.device.read_reg(self.handle, self.cycle_id, self.sel)

    def write_mem(self, input):
        torch.ops.device.write_mem(self.handle, self.rmem_id, self.start_addr, self.num_vec_words, input)

    def read_mem(self, num_elem):
        return torch.ops.device.read_mem(self.handle, self.wmem_id, self.start_addr, self.num_vec_words, num_elem)

    def reset(self, cycles):
        torch.ops.device.reset(self.handle, cycles)

    def run(self, cycles):
        torch.ops.device.run(self.handle, cycles)