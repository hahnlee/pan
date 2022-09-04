pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn get_bytecode_version() -> u32 {
    hermes::runtime::get_bytecode_version()
}
