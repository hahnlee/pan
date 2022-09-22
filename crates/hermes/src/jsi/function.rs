use crate::handle::Local;
use crate::jsi::pointer::PropNameID;
use crate::jsi::runtime::Runtime;
use crate::jsi::value::Value;
use crate::support::Opaque;

type Callback = unsafe extern "C" fn(
    *mut libc::c_void,
    runtime: *const libc::c_void,
    this: *const Value,
    args: *const Value,
    count: usize,
) -> *const Value;

extern "C" {
    fn jsi__function_create_from_host_function(
        runtime: *const libc::c_void,
        name: *const libc::c_void,
        param_count: u32,
        cb: Option<Callback>,
        closure: *const libc::c_void,
    ) -> *const InternalFunction;
    fn jsi__function_call(
        function: *const InternalFunction,
        runtime: *const libc::c_void,
        args: *const *const Value,
        count: usize,
    ) -> *const Value;
}

#[repr(C)]
pub struct InternalFunction(Opaque);

#[repr(C)]
#[derive(Debug)]
pub struct Function(*const InternalFunction);

extern "C" fn c_callback(
    closure: *mut libc::c_void,
    runtime: *const libc::c_void,
    this: *const Value,
    args: *const Value,
    count: usize,
) -> *const Value {
    let closure: &mut Box<
        dyn FnMut(
            *const libc::c_void,
            &Local<'_, Value>,
            &Vec<Local<'_, Value>>,
            usize,
        ) -> *const Value,
    > = unsafe { std::mem::transmute(closure) };

    let this = Value::from_raw(this);
    let raw_args = Value::from_raw(args);

    let mut args: Vec<Local<'_, Value>> = Vec::with_capacity(count);
    args.push(raw_args);

    for i in 1..=count {
        args.push(raw_args.offset(i));
    }

    closure(runtime, &this, &args, count)
}

// TODO: (@hahnlee) provide high level bind API
impl Function {
    pub fn from_raw(ptr: *const InternalFunction) -> Function {
        Function(ptr)
    }

    pub fn from_host_function<T: Runtime, F>(
        runtime: &T,
        name: &str,
        param_count: u32,
        closure: F,
    ) -> Function
    where
        F: FnMut(*const T, &Local<'_, Value>, &Vec<Local<'_, Value>>, usize) -> *const Value,
    {
        let name = PropNameID::from_str(runtime, name);
        let cb: Box<
            Box<
                dyn FnMut(
                    *const T,
                    &Local<'_, Value>,
                    &Vec<Local<'_, Value>>,
                    usize,
                ) -> *const Value,
            >,
        > = Box::new(Box::new(closure));

        unsafe {
            Function::from_raw(jsi__function_create_from_host_function(
                &*runtime as *const _ as *const libc::c_void,
                name.to_ptr(),
                param_count,
                Some(c_callback),
                Box::into_raw(cb) as *mut _ as *mut libc::c_void,
            ))
        }
    }

    pub fn call<'s, T: Runtime>(&self, runtime: &T, args: &[Local<Value>]) -> Local<'s, Value> {
        let args = Local::slice_into_raw(args);
        let value = unsafe {
            Value::from_raw(jsi__function_call(
                self.0,
                &*runtime as *const _ as *const libc::c_void,
                args.as_ptr(),
                args.len(),
            ))
        };
        return value;
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
    use crate::runtime::{HermesRuntime, OwnedHermesRuntime};

    use std::ops::Deref;

    #[test]
    fn check_function_with_pointer() {
        let runtime = OwnedHermesRuntime::new();

        let function = Function::from_host_function::<HermesRuntime, _>(
            &runtime,
            "add",
            2,
            move |runtime_ptr, _, args, count| {
                assert_eq!(count, 2);

                let runtime = HermesRuntime::from_raw(runtime_ptr);

                let first = &args[0];
                let second = &args[1];

                assert_eq!(runtime.is_inspectable(), true);

                assert_eq!(10.0, first.as_number());
                assert_eq!(20.0, second.as_number());

                let value = Value::from_number(first.as_number() + second.as_number());
                value.deref()
            },
        );

        runtime
            .global()
            .set_function::<HermesRuntime>(&runtime, "add", function);

        let output = runtime
            .evaluate_javascript::<StringBuffer>(StringBuffer::new("add(10, 20)").deref(), "");

        assert_eq!(output.as_number(), 30.0);
    }
}
