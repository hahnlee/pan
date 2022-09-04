#include <hermes/hermes.h>
#include <jsi/jsi.h>

using namespace facebook;
using namespace facebook::hermes;

extern "C"
{
  uint32_t hermes__getBytecodeVersion()
  {
    return HermesRuntime::getBytecodeVersion();
  }

  HermesRuntime *hermes__makeHermesRuntime()
  {
    return makeHermesRuntime().release();
  }

  bool hermes__runtime_isInspectable(HermesRuntime *runtime)
  {
    return runtime->isInspectable();
  }
}
