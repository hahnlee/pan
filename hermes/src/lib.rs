include!("bindings.rs");

#[cfg(test)]
mod tests {
    use crate::get_bytecode_version;

    #[test]
    fn check_version() {
        assert_eq!(get_bytecode_version(), 89);
    }
}
