use hermes::runtime::HermesRuntime;
use pan::version;

fn main() {
    println!("{}", version::get_bytecode_version());

    let runtime = HermesRuntime::new();
    println!("{}", runtime.is_inspectable());
}
