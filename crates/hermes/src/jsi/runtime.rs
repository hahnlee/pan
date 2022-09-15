use crate::handle::Local;
use crate::jsi::buffer::Buffer;
use crate::jsi::value::Value;

pub trait Runtime {
    fn evaluate_javascript<T: Buffer>(&self, buffer: &T, source_url: &str) -> Local<'_, Value>;
}
