use clap::Parser;

#[derive(Parser)]
#[clap(version = "0.1", author = "Inherd <forming@inherd.org>")]
struct Opts {
    #[clap(short, long, default_value = "forming")]
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("opts: {:?}", opts.input);
}
