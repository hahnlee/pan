use crate::support::Opaque;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Runtime(Opaque);

impl Runtime {}
