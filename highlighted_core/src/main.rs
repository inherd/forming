use guarding_ident::ModelBuilder;
use std::path::PathBuf;
use std::{env, io, fs};
use std::error::Error;

pub mod document;
pub mod editor;
pub mod identifier;

fn parse_csv(data: &[u8]) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(data);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let dir = env::current_dir().unwrap();

    let filename = dir.clone().join(".forming").join("concepts.csv");
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    parse_csv(contents.as_bytes());

    let models = ModelBuilder::build_models_by_dir(dir);
}
