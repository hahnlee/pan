pub mod handle;
pub mod jsi;
pub mod runtime;
pub mod support;

#[cfg(test)]
mod tests {
    use crate::runtime;

    #[test]
    fn check_version() {
        assert_eq!(runtime::HermesRuntime::get_bytecode_version(), 89);
    }

    #[test]
    fn check_runtime() {
        let runtime = runtime::HermesRuntime::new();
        assert_eq!(runtime.is_inspectable(), true);
    }

    #[test]
    fn check_bytecode() {
        let invalid: [u8; 0] = [];
        assert_eq!(runtime::HermesRuntime::is_hermes_bytecode(&invalid), false);
    }
}
