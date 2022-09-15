use crate::jsi::pointer::PropNameID;
use crate::jsi::runtime::Runtime;
use crate::jsi::value::Value;
use crate::support::Opaque;

type Callback = unsafe extern "C" fn(
    *mut libc::c_void,
    runtime: *const libc::c_void,
    args: *const Value,
    count: libc::size_t,
) -> *const Value;

extern "C" {
    fn jsi__function_createFromHostFunction(
        runtime: *const libc::c_void,
        name: *const libc::c_void,
        param_count: u32,
        cb: Option<Callback>,
        closure: *const libc::c_void,
    ) -> *mut InternalFunction;
}

#[repr(C)]
struct InternalFunction(Opaque);

#[repr(C)]
#[derive(Debug)]
pub struct Function(*mut InternalFunction);

extern "C" fn c_callback(
    closure: *mut libc::c_void,
    runtime: *const libc::c_void,
    args: *const Value,
    count: libc::size_t,
) -> *const Value {
    let closure: &mut Box<
        dyn FnMut(*const libc::c_void, *const Value, libc::size_t) -> *const Value,
    > = unsafe { std::mem::transmute(closure) };
    closure(runtime, args, count)
}

impl Function {
    pub fn from_host_function<T: Runtime, F>(
        runtime: &T,
        name: &str,
        param_count: u32,
        closure: F,
    ) -> Function
    where
        F: FnMut(*const libc::c_void, *const Value, libc::size_t) -> *const Value,
        F: 'static,
    {
        let name = PropNameID::from_str(runtime, name);
        let cb: Box<
            Box<dyn FnMut(*const libc::c_void, *const Value, libc::size_t) -> *const Value>,
        > = Box::new(Box::new(closure));

        unsafe {
            Function(jsi__function_createFromHostFunction(
                &*runtime as *const _ as *const libc::c_void,
                name.to_ptr(),
                param_count,
                Some(c_callback),
                Box::into_raw(cb) as *mut _ as *mut libc::c_void,
            ))
        }
    }

    pub fn to_ptr(&self) -> *const libc::c_void {
        self.0 as *const libc::c_void
    }
}

#[cfg(test)]
mod test {
    use crate::jsi::buffer::StringBuffer;
    use crate::jsi::function::Function;
    use crate::jsi::runtime::Runtime;
    use crate::jsi::value::Value;
    use crate::runtime::HermesRuntime;

    use std::ops::Deref;

    #[test]
    fn check_function_with_pointer() {
        let runtime = HermesRuntime::new();

        let number = 10.0;

        let function = Function::from_host_function::<HermesRuntime, _>(
            &runtime,
            "required",
            0,
            move |_, _, _| {
                let value = Value::from_number(number);
                value.deref()
            },
        );

        runtime
            .global()
            .set_property::<HermesRuntime>(&runtime, "required", function.to_ptr());

        let output = runtime
            .evaluate_javascript::<StringBuffer>(StringBuffer::new("required()").deref(), "");

        assert_eq!(output.as_number(), number);
    }
}
