pub mod runtime;
pub mod support;
pub mod jsi;

#[cfg(test)]
mod tests {
    use crate::runtime::get_bytecode_version;

    #[test]
    fn check_version() {
        assert_eq!(get_bytecode_version(), 89);
    }
}
