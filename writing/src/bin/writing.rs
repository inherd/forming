use std::fs;

use clap::Parser;

use writing::Writing;

#[derive(Parser)]
#[clap(version = "0.1", author = "Inherd <forming@inherd.org>")]
struct Opts {
    #[clap(short, long, default_value = "README.md")]
    path: String,
    #[clap(short, long, default_value = "out.md")]
    output: String,
}

fn main() {
    let opts: Opts = Opts::parse();


    let result = match Writing::process_file(opts.path) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    };

    match fs::write(opts.output, result) {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
            return;
        }
    }
}
