import ctypes

class Device:
    def __init__(self, lib):
        self.lib = ctypes.CDLL(lib)
        self.lib.alloc.restype = ctypes.c_void_p
        self.lib.dealloc.argtypes = [ctypes.c_void_p]
        self.lib.reset.argtypes = [ctypes.c_void_p, ctypes.c_int]
        self.lib.run.argtypes = [ctypes.c_void_p, ctypes.c_int]
        self.handle = self.lib.alloc()
    def __del__(self):
        self.lib.dealloc(self.handle)
    def reset(self, cycles):
        self.lib.reset(self.handle, cycles)
    def run(self, cycles):
        self.lib.run(self.handle, cycles)
