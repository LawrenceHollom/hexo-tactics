use serde_json::*;

pub fn read_json(filename: &str) -> Value {
    let filename = format!("data/{}.json", filename);
    // Read the JSON data from the file.
    let file = std::fs::File::open(&filename).unwrap();
    let json = serde_json::from_reader(file).unwrap();
    json
}