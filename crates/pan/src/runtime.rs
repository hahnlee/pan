use std::collections::HashMap;
use std::path::PathBuf;

use hermes::handle::Local;
use hermes::jsi::value::Value;
use hermes::runtime::OwnedHermesRuntime;

use crate::module::{bind_require, evaluate_module};

#[derive(Debug)]
pub struct PanRuntime {
    pub hermes: OwnedHermesRuntime,
    pub stack: Vec<PathBuf>,
    pub modules: HashMap<String, Local<'static, Value>>,
}

impl PanRuntime {
    pub fn new() -> Self {
        Self {
            hermes: OwnedHermesRuntime::new(),
            stack: Vec::new(),
            modules: HashMap::new(),
        }
    }

    pub fn initialize(&mut self) {
        bind_require(self);
    }

    pub fn run(&mut self, file_path: &str) -> *const Value {
        let absolute_path = PathBuf::from(file_path).canonicalize().unwrap();
        evaluate_module(
            &self.hermes,
            absolute_path,
            &mut self.stack,
            &mut self.modules,
        )
    }
}
