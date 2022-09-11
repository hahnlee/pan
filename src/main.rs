use hermes::runtime::HermesRuntime;

use pan::version;

fn main() {
    println!("{}", version::get_bytecode_version());

    let runtime = HermesRuntime::new();
    println!("{}", runtime.is_inspectable());

    let mut id = hermes::jsi::pointer::PropNameID::from_str::<HermesRuntime>(&runtime, "test");
    println!("{}", id.to_string::<HermesRuntime>(&runtime));
}
