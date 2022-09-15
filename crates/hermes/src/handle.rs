use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

#[repr(C)]
#[derive(Debug)]
pub struct Local<'s, T>(NonNull<T>, PhantomData<&'s ()>);

impl<'s, T> Local<'s, T> {
    pub(crate) unsafe fn from_raw(ptr: *const T) -> Option<Self> {
        NonNull::new(ptr as *mut _).map(|nn| Self::from_non_null(nn))
    }

    pub(crate) unsafe fn from_non_null(nn: NonNull<T>) -> Self {
        Self(nn, PhantomData)
    }
}

impl<'s, T> Copy for Local<'s, T> {}

impl<'s, T> Clone for Local<'s, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'s, T> Deref for Local<'s, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.0.as_ref() }
    }
}
