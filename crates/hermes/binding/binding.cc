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

  bool hermes__compile_js(const char *str, size_t code_size, const char *&data, size_t &size, bool optimize)
  {
    std::string code = std::string(str, code_size);
    std::string bytecode;

    bool result = hermes::compileJS(code, bytecode, optimize);
    if (result)
    {
      data = bytecode.data();
      size = bytecode.size();
    }

    return result;
  }

  HermesRuntime *hermes__make_hermes_runtime()
  {
    return makeHermesRuntime().release();
  }

  bool hermes__runtime_is_hermes_bytecode(const uint8_t *data, size_t len)
  {
    return HermesRuntime::isHermesBytecode(data, len);
  }

  uint32_t hermes__runtime_get_bytecode_version()
  {
    return HermesRuntime::getBytecodeVersion();
  }

  bool hermes__runtime_is_inspectable(HermesRuntime *runtime)
  {
    return runtime->isInspectable();
  }

  void hermes__runtime_delete(HermesRuntime *runtime)
  {
    delete runtime;
  }

  Value *hermes__runtime_evaluate_javascript(HermesRuntime *runtime, Buffer *buffer, const char *source_url, size_t size)
  {
    Value value = runtime->evaluateJavaScript(std::shared_ptr<Buffer>(buffer), std::string(source_url, size));
    return new Value(std::move(value));
  }

  Object *hermes__runtime_global(HermesRuntime *runtime)
  {
    Object object = runtime->global();
    return new Object(std::move(object));
  }

  StringBuffer *jsi__string_buffer_new(const char *data, size_t size)
  {
    std::string code = std::string(data, size);
    StringBuffer *buffer = new StringBuffer(code);
    return buffer;
  }

  size_t jsi__string_buffer_size(StringBuffer *buffer)
  {
    return buffer->size();
  }

  void jsi__string_buffer_delete(StringBuffer *buffer)
  {
    delete buffer;
  }

  Value *jsi__value_new_number(double value)
  {
    return new Value(value);
  }

  bool jsi__value_is_undefined(Value *value)
  {
    return value->isUndefined();
  }

  bool jsi__value_is_number(Value *value)
  {
    return value->isNumber();
  }

  bool jsi__value_is_string(Value *value)
  {
    return value->isString();
  }

  double jsi__value_as_number(Value *self)
  {
    return self->asNumber();
  }

  void jsi__value_delete(Value *value)
  {
    delete value;
  }

  std::string *jsi__value_to_bytes(Value *value, Runtime *runtime, size_t &size)
  {
    std::string string = value->toString(*runtime).utf8(*runtime);
    return new std::string(move(string));
  }

  Value *jsi__offset_from_ptr(Value *value, size_t offset)
  {
    return &value[offset];
  }

  Value *jsi__object_get_property(Object *self, Runtime *runtime, const char *name)
  {
    Value value = self->getProperty(*runtime, name);
    return new Value(std::move(value));
  }

  void jsi__object_set_property(Object *self, Runtime *runtime, const char *name, Function *value)
  {
    self->setProperty(*runtime, name, *value);
  }

  void jsi__object_delete(Object *object)
  {
    delete object;
  }

  PropNameID *jsi__prop_name_id_for_utf8(Runtime *runtime, const char *name)
  {
    return new PropNameID(std::move(PropNameID::forUtf8(*runtime, std::string(name))));
  }

  std::string *jsi__prop_name_id_utf8(PropNameID *self, Runtime *runtime)
  {
    return new std::string(std::move(self->utf8(*runtime)));
  }

  typedef Value *(*Callback)(void *closure, Runtime *runtime, const Value *thisVal, const Value *args, size_t count);

  Function *jsi__function_create_from_host_function(Runtime *runtime, PropNameID *name, unsigned int paramCount, Callback callback, void *closure)
  {
    auto cb = [callback, closure, runtime](Runtime &rt, const Value &thisVal, const Value *args, size_t count) -> Value
    {
      Value *value = callback(closure, runtime, &thisVal, args, count);
      return Value(std::move(*value));
    };
    Function fn = Function::createFromHostFunction(*runtime, *name, paramCount, cb);
    return new Function(std::move(fn));
  }
}
