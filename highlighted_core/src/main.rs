use guarding_ident::ModelBuilder;
use std::path::PathBuf;
use std::env;

pub mod document;
pub mod editor;
pub mod identifier;

fn main() {
    let dir = env::current_dir().unwrap();
    println!("{:?}", dir.display());
    let models = ModelBuilder::build_models_by_dir(dir);
}
