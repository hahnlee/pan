#include <hermes/hermes.h>
#include <hermes/CompileJS.h>
#include <jsi/jsi.h>

using namespace facebook::hermes;

extern "C"
{
  bool hermes__compileJS(const char *str, const char *&data, size_t &size, bool optimize)
  {
    std::string code = std::string(str);
    std::string bytecode;

    bool result = hermes::compileJS(code, bytecode, optimize);
    if (result)
    {
      data = bytecode.data();
      size = bytecode.size();
    }

    return result;
  }

  bool hermes__isHermesBytecode(const uint8_t *data, size_t len)
  {
    return HermesRuntime::isHermesBytecode(data, len);
  }

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
