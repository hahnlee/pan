#include <hermes/hermes.h>

using namespace facebook::hermes;

uint32_t version = HermesRuntime::getBytecodeVersion();

extern "C" uint32_t getBytecodeVersion() {
  return version;
}
