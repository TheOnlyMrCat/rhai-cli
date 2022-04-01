use std::path::PathBuf;

use clap::Parser;
use exitcode::ExitCode;
use rhai::Engine;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The file to run
    #[clap(parse(from_os_str))]
    filename: PathBuf,
    /// Don't optimise scripts before running them
    #[clap(long)]
    no_optimise: bool,
}

fn main() {
    std::process::exit(run());
}

fn run() -> ExitCode {
    let args = Cli::parse();
    let file = match std::fs::read_to_string(&args.filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("{}", e);
            return exitcode::USAGE;
        }
    };

    let file = if file.starts_with("#!") {
        // Skip shebang, because rhai uses C-style comments
        &file[file.find('\n').unwrap_or(0)..]
    } else {
        &file
    };
    
    let mut engine = Engine::new();
    if !args.no_optimise {
        engine.set_optimization_level(rhai::OptimizationLevel::Full);
    }
    let mut ast = match engine.compile(file) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("{}", e);
            return exitcode::DATAERR;
        }
    };
    ast.set_source(format!("{}", args.filename.display()));
    match engine.run_ast(&ast) {
        Ok(_) => exitcode::OK,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}
