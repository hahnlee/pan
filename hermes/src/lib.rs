pub mod runtime;
pub mod support;
pub mod jsi;

#[cfg(test)]
mod tests {
    use crate::runtime;

    #[test]
    fn check_version() {
        assert_eq!(runtime::get_bytecode_version(), 89);
    }

    #[test]
    fn check_runtime() {
        let runtime = runtime::HermesRuntime::new();
        assert_eq!(runtime.is_inspectable(), true);
    }
}
