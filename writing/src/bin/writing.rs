use clap::Parser;
use writing::Writing;

#[derive(Parser)]
#[clap(version = "0.1", author = "Inherd <forming@inherd.org>")]
struct Opts {
    #[clap(short, long, default_value = "README.md")]
    path: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    match Writing::process_file(opts.path) {
        Ok(some) => {
            println!("{}", some);
        }
        Err(error) => {
            println!("{:?}", error)
        }
    }
}
