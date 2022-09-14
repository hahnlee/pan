use hermes::jsi::buffer::StringBuffer;
use hermes::jsi::function::Function;
use hermes::jsi::runtime::Runtime;
use hermes::jsi::value::Value;
use hermes::runtime::HermesRuntime;
use std::ops::Deref;

fn main() {
    let runtime = HermesRuntime::new();

    let number = 10.0;

    let function =
        Function::from_host_function::<HermesRuntime, _>(&runtime, "required", 0, move || {
            let value = Value::from_number(number);
            value.deref()
        });

    runtime
        .global()
        .set_property::<HermesRuntime>(&runtime, "required", function.to_ptr());

    let output =
        runtime.evaluate_javascript::<StringBuffer>(StringBuffer::new("required()").deref(), "");

    println!("{}", output.is_number(&*runtime));
    println!("{}", output.as_number() == number);
}
