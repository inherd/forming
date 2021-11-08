use std::fs;
use clap::Parser;
use forming::parser::parse;

#[derive(Parser)]
#[clap(version = "0.1", author = "Inherd <forming@inherd.org>")]
struct Opts {
    #[clap(short, long, default_value = "forming")]
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let content = fs::read(opts.input).unwrap();
    let str = String::from_utf8_lossy(&*content);
    let unit = parse(str.to_string().as_str());
    println!("{:?}", unit);
}
