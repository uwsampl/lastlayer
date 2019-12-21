import ctypes

class Device:
    def __init__(self, lib):
        self.lib = ctypes.CDLL(lib)
        self.lib.LastLayerAlloc.restype = ctypes.c_void_p
        self.lib.LastLayerDealloc.argtypes = [ctypes.c_void_p]
        self.lib.LastLayerReset.argtypes = [ctypes.c_void_p, ctypes.c_int]
        self.lib.LastLayerRun.argtypes = [ctypes.c_void_p, ctypes.c_int]
        self.handle = self.lib.LastLayerAlloc()
    def __del__(self):
        self.lib.LastLayerDealloc(self.handle)
    def reset(self, cycles):
        self.lib.LastLayerReset(self.handle, cycles)
    def run(self, cycles):
        self.lib.LastLayerRun(self.handle, cycles)
