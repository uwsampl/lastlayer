#include <torch/script.h>
#include <vector>
#include "lastlayer.h"

// unlikely to change, used in write_mem and read_mem
#define WORD_BYTES 4

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
    LastLayerWriteReg(reinterpret_cast<LastLayerHandle>(handle), hid, sel, value);
}

void write_mem(TorchDeviceHandle handle,
               int64_t hid,
               int64_t start_addr,
               int64_t num_word,
               torch::Tensor input) {
  TORCH_CHECK(input.is_contiguous());
  TORCH_CHECK(input.numel() >= WORD_BYTES && input.numel() % WORD_BYTES == 0)
  int8_t* a = (int8_t*)input.data_ptr();
  int32_t saddr = start_addr;
  uint32_t wdata;
  for (auto i = 0; i < input.numel() / WORD_BYTES; i = i + num_word) {
    for (auto j = 0; j < num_word; j++) {
        wdata = 0;
        for (auto k = 0; k < WORD_BYTES; k++) {
            wdata = wdata | ((*a++ << 8*k) & (0xff << 8*k));
        }
        LastLayerWriteMem(reinterpret_cast<LastLayerHandle>(handle), hid, saddr, j, wdata);
    }
    saddr++;
  }
}

torch::Tensor read_mem(TorchDeviceHandle handle,
                       int64_t hid,
                       int64_t start_addr,
                       int64_t num_word,
                       int64_t num_elem) {
    torch::Tensor output = torch::ones(num_elem, torch::kInt8);
    TORCH_CHECK(output.is_contiguous());
    TORCH_CHECK(num_elem >= WORD_BYTES && num_elem % WORD_BYTES == 0)
    int8_t* a = (int8_t*)output.data_ptr();
    int32_t saddr = start_addr;
    uint32_t rdata;
    for (auto i = 0; i < num_elem / WORD_BYTES; i = i + num_word) {
        for (auto j = 0; j < num_word; j++) {
            rdata = LastLayerReadMem(reinterpret_cast<LastLayerHandle>(handle), hid, saddr, j);
            for (auto k = 0; k < WORD_BYTES; k++) {
                *a++ = rdata >> 8*k;
            }
        }
        saddr++;
    }
    return output;
}

void reset(TorchDeviceHandle handle, int64_t n) {
    LastLayerReset(reinterpret_cast<LastLayerHandle>(handle), n);
}

void run(TorchDeviceHandle handle, int64_t n) {
    LastLayerRun(reinterpret_cast<LastLayerHandle>(handle), n);
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
        torch::RegisterOperators().op("device::read_mem", &read_mem));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::write_mem", &write_mem));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::reset", &reset));
    registeredOps.push_back(
        torch::RegisterOperators().op("device::run", &run));
  return registeredOps;
}

static auto registry = register_device_api();