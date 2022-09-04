#include <hermes/hermes.h>
#include <jsi/jsi.h>

using namespace facebook;
using namespace facebook::hermes;

extern "C"
{
  uint32_t getBytecodeVersion()
  {
    return HermesRuntime::getBytecodeVersion();
  }
}
