use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
#[allow(dead_code)]
struct Opts {
    #[clap(short, long, default_value = "forming")]
    input: String,
}

fn main() {
    // let opts: Opts = Opts::parse();
    println!("hello, world!");
}
