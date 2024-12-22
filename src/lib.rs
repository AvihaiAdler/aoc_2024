use std::fs::File;
use std::io::BufReader;

pub fn get_reader(path: &str) -> std::io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
