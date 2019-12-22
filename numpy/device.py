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
        self.sel = 0
    def __del__(self):
        self.lib.LastLayerDealloc(self.handle)
    def read_reg_a(self):
        return int.from_bytes(self.lib.LastLayerReadReg(self.handle, 0, self.sel), byteorder="big")
    def read_reg_b(self):
        return int.from_bytes(self.lib.LastLayerReadReg(self.handle, 1, self.sel), byteorder="big")
    def read_reg_y(self):
        return int.from_bytes(self.lib.LastLayerReadReg(self.handle, 2, self.sel), byteorder="big")
    def write_reg_a(self, value):
        self.lib.LastLayerWriteReg(self.handle, 0, self.sel, int(value))
    def write_reg_b(self, value):
        self.lib.LastLayerWriteReg(self.handle, 1, self.sel, int(value))
    def write_reg_y(self, value):
        self.lib.LastLayerWriteReg(self.handle, 2, self.sel, int(value))
    def reset(self, cycles):
        self.lib.LastLayerReset(self.handle, cycles)
    def run(self, cycles):
        self.lib.LastLayerRun(self.handle, cycles)
