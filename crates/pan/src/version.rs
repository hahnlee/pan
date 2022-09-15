use hermes::runtime::HermesRuntime;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn get_bytecode_version() -> u32 {
    HermesRuntime::get_bytecode_version()
}
