use std::env;

use pan::initialize;
use pan::run_js;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let runtime = initialize();
    run_js(&runtime, file_path);
}
