#ifdef __cplusplus
extern "C" {
#endif

typedef void* LastLayerHandle;

LastLayerHandle LastLayerAlloc();
void LastLayerDealloc(LastLayerHandle handle);

void LastLayerReset(LastLayerHandle handle, int cycles);
void LastLayerRun(LastLayerHandle handle, int cycles);

#ifdef __cplusplus
}
#endif