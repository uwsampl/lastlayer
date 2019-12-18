#ifdef __cplusplus
extern "C" {
#endif

typedef void* LLDeviceHandle;

LLDeviceHandle alloc();
void dealloc(LLDeviceHandle handle);

void reset(LLDeviceHandle handle, int cycles);
void run(LLDeviceHandle handle, int cycles);

#ifdef __cplusplus
}
#endif