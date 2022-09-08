#include <hermes/hermes.h>
#include <hermes/CompileJS.h>
#include <jsi/jsi.h>

using namespace facebook::hermes;
using namespace facebook::jsi;

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

  HermesRuntime *hermes__makeHermesRuntime()
  {
    return makeHermesRuntime().release();
  }

  bool hermes__runtime_isHermesBytecode(const uint8_t *data, size_t len)
  {
    return HermesRuntime::isHermesBytecode(data, len);
  }

  uint32_t hermes__runtime_getBytecodeVersion()
  {
    return HermesRuntime::getBytecodeVersion();
  }

  bool hermes__runtime_isInspectable(HermesRuntime *runtime)
  {
    return runtime->isInspectable();
  }

  StringBuffer *jsi__stringBuffer_New(const char *data)
  {
    std::string code = std::string(data);
    StringBuffer *buffer = new StringBuffer(code);
    return buffer;
  }

  size_t jsi__stringBuffer_size(StringBuffer *buffer)
  {
    return buffer->size();
  }

  void jsi__stringBuffer_delete(StringBuffer *buffer)
  {
    delete buffer;
  }
}
