use std::{env, fs, io};
use std::error::Error;
use std::path::PathBuf;

use guarding_ident::ModelBuilder;

pub mod document;
pub mod editor;
pub mod identifier;
pub mod concept_parser;

fn main() {
    let dir = env::current_dir().unwrap();

    let filename = dir.clone().join(".forming").join("concepts.csv");
    concept_parser::parse_by_file(filename);

    let models = ModelBuilder::build_models_by_dir(dir);
}