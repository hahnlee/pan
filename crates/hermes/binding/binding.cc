#include <hermes/hermes.h>
#include <hermes/CompileJS.h>
#include <jsi/jsi.h>

using namespace facebook::hermes;
using namespace facebook::jsi;

extern "C"
{
  void cpp_string_destroy(std::string *str)
  {
    delete str;
  }

  size_t cpp_string_size(std::string *str)
  {
    return str->size();
  }

  char *cpp_string_data(std::string *str)
  {
    return str->data();
  }

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

  void hermes__runtime_delete(HermesRuntime *runtime)
  {
    delete runtime;
  }

  Value *hermes__runtime_evaluateJavaScript(HermesRuntime *runtime, Buffer *buffer, const char *sourceURL)
  {
    Value value = runtime->evaluateJavaScript(std::shared_ptr<Buffer>(buffer), std::string(sourceURL));
    return new Value(std::move(value));
  }

  Object *hermes__runtime_global(HermesRuntime *runtime)
  {
    Object object = runtime->global();
    return new Object(std::move(object));
  }

  StringBuffer *jsi__stringBuffer_new(const char *data)
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

  Value *jsi__value_NewNumber(double value)
  {
    return new Value(value);
  }

  bool jsi__value_isUndefined(Value *value)
  {
    return value->isUndefined();
  }

  bool jsi__value_isNumber(Value *value, HermesRuntime *runtime)
  {
    return value->isNumber();
  }

  double jsi__value_asNumber(Value *self)
  {
    return self->asNumber();
  }

  void jsi__value_delete(Value *value)
  {
    delete value;
  }

  Value *jsi__object_getProperty(Object *self, Runtime *runtime, const char *name)
  {
    Value value = self->getProperty(*runtime, name);
    return new Value(std::move(value));
  }

  void jsi__object_setProperty(Object *self, Runtime *runtime, const char *name, Function *value)
  {
    self->setProperty(*runtime, name, *value);
  }

  void jsi__object_delete(Object *object)
  {
    delete object;
  }

  PropNameID *jsi__PropNameID_forUtf8(Runtime *runtime, const char *name)
  {
    return new PropNameID(std::move(PropNameID::forUtf8(*runtime, std::string(name))));
  }

  std::string *jsi__PropNameID_utf8(PropNameID *self, Runtime *runtime)
  {
    return new std::string(std::move(self->utf8(*runtime)));
  }

  typedef Value *(*Callback)(void *closure);

  Function *jsi__function_createFromHostFunction(Runtime *runtime, PropNameID *name, unsigned int paramCount, Callback callback, void *closure)
  {
    auto cb = [callback, closure](Runtime &rt, const Value &thisVal, const Value *args, size_t count) -> Value
    {
      Value *value = callback(closure);
      return Value(std::move(*value));
    };
    Function fn = Function::createFromHostFunction(*runtime, *name, paramCount, cb);
    return new Function(std::move(fn));
  }
}
