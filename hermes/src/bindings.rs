extern "C" {
    fn getBytecodeVersion() -> u32;
}

pub fn get_bytecode_version() -> u32 {
    unsafe { getBytecodeVersion() }
}
