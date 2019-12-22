#ifdef __cplusplus
extern "C" {
#endif

typedef void* LastLayerHandle;

LastLayerHandle LastLayerAlloc();
void LastLayerDealloc(LastLayerHandle handle);

char LastLayerReadReg(LastLayerHandle handle, int hid, int sel);
void LastLayerWriteReg(LastLayerHandle handle, int hid, int sel, char value);

char LastLayerReadMem(LastLayerHandle handle, int hid, int addr, int sel);
void LastLayerWriteMem(LastLayerHandle handle, int hid, int addr, int sel, char value);

void LastLayerReset(LastLayerHandle handle, int cycles);
void LastLayerRun(LastLayerHandle handle, int cycles);

#ifdef __cplusplus
}
#endif