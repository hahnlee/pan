use crate::jsi::pointer::PropNameID;
use crate::jsi::runtime::Runtime;
use crate::jsi::value::Value;
use crate::support::Opaque;

type Callback = unsafe extern "C" fn(*mut libc::c_void) -> *const Value;

extern "C" {
    fn jsi__function_createFromHostFunction(
        runtime: *const libc::c_void,
        name: *const libc::c_void,
        param_count: u32,
        cb: Callback,
        closure: *const libc::c_void,
    ) -> *mut InternalFunction;
}

#[repr(C)]
struct InternalFunction(Opaque);

#[repr(C)]
#[derive(Debug)]
pub struct Function(*mut InternalFunction);

fn to_c_callback<F>(_closure: &F) -> Callback
where
    F: FnMut() -> *const Value,
{
    c_callback::<F>
}

unsafe extern "C" fn c_callback<F>(closure: *mut libc::c_void) -> *const Value
where
    F: FnMut() -> *const Value,
{
    let closure = &mut *(closure as *mut F);
    closure()
}

impl Function {
    pub fn from_host_function<T: Runtime, P>(
        runtime: &T,
        name: &str,
        param_count: u32,
        closure: &mut P,
    ) -> Function
    where
        P: Fn() -> *const Value,
    {
        let name = PropNameID::from_str(runtime, name);
        let callback = to_c_callback(&closure);

        unsafe {
            Function(jsi__function_createFromHostFunction(
                &*runtime as *const _ as *const libc::c_void,
                name.to_ptr(),
                param_count,
                callback,
                closure as *mut _ as *mut libc::c_void,
            ))
        }
    }

    pub fn to_ptr(&self) -> *const libc::c_void {
        self.0 as *const libc::c_void
    }
}
