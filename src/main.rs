use pan::version;

fn main() {
    println!("{}", version::get_bytecode_version());

    hermes::runtime::make_hermes_runtime();
}
