use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn parse_by_dir(_filename: PathBuf) {

}

pub fn parse_by_file(filename: PathBuf) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    parse_csv(contents.as_bytes());
}

fn parse_csv(data: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(data);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
