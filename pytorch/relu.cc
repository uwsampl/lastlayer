#include <torch/script.h>
#include <vector>
#include "lastlayer.h"

typedef int64_t TorchDeviceHandle;

TorchDeviceHandle alloc() {
    return reinterpret_cast<TorchDeviceHandle>(LastLayerAlloc());
}

void dealloc(TorchDeviceHandle handle) {
    LastLayerDealloc(reinterpret_cast<LastLayerHandle>(handle));
}

int64_t read_reg(TorchDeviceHandle handle, int64_t hid, int64_t sel) {
    return LastLayerReadReg(reinterpret_cast<LastLayerHandle>(handle), hid, sel);
}

void write_reg(TorchDeviceHandle handle, int64_t hid, int64_t sel, int64_t value) {
    return LastLayerWriteReg(reinterpret_cast<LastLayerHandle>(handle), hid, sel, value);
}

void reset(TorchDeviceHandle handle, int64_t cycles) {
    LastLayerReset(reinterpret_cast<LastLayerHandle>(handle), cycles);
}

void run(TorchDeviceHandle handle, int64_t cycles) {
    LastLayerRun(reinterpret_cast<LastLayerHandle>(handle), cycles);
}

std::vector<torch::RegisterOperators> register_device_api() {
    std::vector<torch::RegisterOperators> registeredOps;
    registeredOps.push_back(
        torch::RegisterOperators().op("device::alloc", &alloc));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::dealloc", &dealloc));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::read_reg", &read_reg));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::write_reg", &write_reg));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::reset", &reset));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::run", &run));
  return registeredOps;
}

static auto registry = register_device_api();