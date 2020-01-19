#ifdef __cplusplus
extern "C" {
#endif

/* device handle */
typedef void* LastLayerHandle;

/* allocate device */
LastLayerHandle LastLayerAlloc();

/* deallocate device */
void LastLayerDealloc(LastLayerHandle handle);

/* read a register */
int LastLayerReadReg(LastLayerHandle handle, int hid, int sel);

/* write a register */
void LastLayerWriteReg(LastLayerHandle handle,
    int hid, int sel, int value);

/* read a memory */
int LastLayerReadMem(LastLayerHandle handle,
    int hid, int addr, int sel);

/* write a memory */
void LastLayerWriteMem(LastLayerHandle handle,
    int hid, int addr, int sel, int value);

/* reset for n clock cycles */
void LastLayerReset(LastLayerHandle handle, int n);

/* run for n clock cycles */
void LastLayerRun(LastLayerHandle handle, int n);

#ifdef __cplusplus
}
#endif
