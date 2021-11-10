use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use clap::Parser;
use walkdir::WalkDir;

use forming::parser::parse;

#[derive(Parser)]
#[clap(version = "0.1", author = "Inherd <forming@inherd.org>")]
struct Opts {
    #[clap(short, long, default_value = "default.forming")]
    input: String,

    #[clap(short, long, default_value = "src")]
    dir: String,

    #[clap(short, long, default_value = "default.forming")]
    // auto crate a basic
    generate: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let content = fs::read(opts.input).unwrap();
    let str = String::from_utf8_lossy(&*content);
    let unit = parse(str.to_string().as_str());
    println!("{:?}", unit);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileCache {
    pub path: PathBuf,
    pub created: SystemTime,
    pub modified: SystemTime,
}

pub fn store<P: AsRef<Path>>(buf: P, suffix: &str) -> Vec<FileCache> {
    let walk_dir = WalkDir::new(buf);
    let vec = walk_dir.into_iter()
        .map(|entry| entry.expect("error dir"))
        .filter(|entry| {
            if let Some(ext) = entry.path().extension() {
                return ext.to_string_lossy().to_string().eq(suffix);
            }

            false
        })
        .map(|entry| {
            let metadata = entry.metadata().unwrap();
            FileCache {
                path: entry.into_path(),
                created: metadata.created().unwrap(),
                modified: metadata.modified().unwrap(),
            }
        })
        .collect::<Vec<FileCache>>();

    vec
}

#[cfg(test)]
mod tests {
    use crate::store;

    #[test]
    fn store_file_info() {
        let vec = store("src", "rs");
        println!("{:?}", vec);
    }
}