use std::fs::File;
use std::io::Read;

/// To run most of the examples, there needs to be a valid key.
pub fn read_key() -> String {
    let mut file = match File::open("./examples/key.txt") {
        Ok(f) => f,
        Err(_) => panic!("Could not open file."),
    };
    let mut file_contents = String::new();
    if file.read_to_string(&mut file_contents).is_err() {
        panic!("Could not read file.");
    }
    return file_contents;
}
