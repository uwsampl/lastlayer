from ctypes import CDLL, c_void_p, c_char, c_int

class Device:
    def __init__(self, lib):
        self.lib = CDLL(lib)
        self.lib.LastLayerAlloc.restype = c_void_p
        self.lib.LastLayerDealloc.argtypes = [c_void_p]
        self.lib.LastLayerReadReg.restype = c_char
        self.lib.LastLayerReadReg.argtypes = [c_void_p, c_int, c_int]
        self.lib.LastLayerWriteReg.argtypes = [c_void_p, c_int, c_int, c_char]
        self.lib.LastLayerReset.argtypes = [c_void_p, c_int]
        self.lib.LastLayerRun.argtypes = [c_void_p, c_int]
        self.handle = self.lib.LastLayerAlloc()
    def __del__(self):
        self.lib.LastLayerDealloc(self.handle)
    def read(self, hid, sel):
        return int.from_bytes(self.lib.LastLayerReadReg(self.handle, hid, sel), byteorder="big")
    def write(self, hid, sel, value):
        self.lib.LastLayerWriteReg(self.handle, hid, sel, value)
    def reset(self, cycles):
        self.lib.LastLayerReset(self.handle, cycles)
    def run(self, cycles):
        self.lib.LastLayerRun(self.handle, cycles)
