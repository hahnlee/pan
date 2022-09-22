use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use pan::compile;
use pan::initialize;
use pan::run_js;

#[derive(Parser)]
struct Cli {
    #[clap(value_parser)]
    input: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Compile {
        #[clap(value_parser)]
        input: PathBuf,

        #[clap(long)]
        optimize: bool,

        #[clap(short, long, value_parser)]
        out: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(input_path) = cli.input.as_deref() {
        let mut runtime = initialize();
        run_js(&mut runtime, &input_path.to_str().unwrap());
        return;
    }

    match &cli.command {
        Some(Commands::Compile { input, optimize, out  }) => {
            let file = fs::read(&input).unwrap();
            let bytecode = compile(&file, *optimize);

            let output_path = match out {
                Some(output) => {output.clone()},
                None => {
                    let absolute_path = input.canonicalize().unwrap();
                    let parent = absolute_path.parent().unwrap();
                    let file_stem = input.file_stem().unwrap().to_str().unwrap();
                    let mut output = parent.to_path_buf();
                    output.push(format!("{}.hbc", file_stem));

                    output
                }
            };

            fs::write(&output_path, &bytecode).unwrap();
        }
        None => {}
    }
}
