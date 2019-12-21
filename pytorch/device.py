import torch

class Device:
    def __init__(self, lib):
        torch.ops.load_library(lib)
        self.handle = torch.ops.device.alloc()
    def __del__(self):
        torch.ops.device.dealloc(self.handle)
    def reset(self, cycles):
        torch.ops.device.reset(self.handle, cycles)
    def run(self, cycles):
        torch.ops.device.run(self.handle, cycles)