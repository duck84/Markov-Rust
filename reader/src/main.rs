use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::fs::File;


fn main() {
    let path = Path::new("../../text/hamlet.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, text),
    }
}
