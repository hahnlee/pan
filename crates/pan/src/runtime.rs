use std::fs;
use std::path::PathBuf;

use hermes::buffer::MemoryBuffer;
use hermes::jsi::runtime::Runtime;
use hermes::runtime::OwnedHermesRuntime;

use crate::module::bind_require;

pub struct PanRuntime {
    pub hermes: OwnedHermesRuntime,
    pub stack: Vec<PathBuf>,
}

impl PanRuntime {
    pub fn new() -> Self {
        Self {
            hermes: OwnedHermesRuntime::new(),
            stack: Vec::new(),
        }
    }

    pub fn initialize(&mut self) {
        bind_require(self);
    }

    pub fn run(&mut self, file_path: &str) {
        let file_path = PathBuf::from(file_path).canonicalize().unwrap();

        let file = fs::read(&file_path).unwrap();
        let buffer = MemoryBuffer::from_bytes(&file);

        let source_url = format!("file://{}", file_path.to_str().unwrap());

        self.stack.push(file_path);
        self.hermes
            .evaluate_javascript(&buffer, source_url.as_str());
        self.stack.pop();
    }
}
