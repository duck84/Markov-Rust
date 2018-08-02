use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;


fn main() {
    let path = Path::new("../../text/test.txt");
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

    let tokens: Vec<&str> = text.split_whitespace().collect();

    let mut parse = false;
    let mut lines = Vec::new();

    for word in tokens{
        if word == "MATT."{
            parse = false;
            lines.push("END");
        }
        if parse{
            lines.push(word);
        }
        if word == "MIKE."{
            parse = true;
            lines.push("START");
        }
    }

    let group = lines.windows(3);

    let mut histogram: HashMap<&str, (&str, &str)> = HashMap::new();

    for words in group {

        let prefix = words[0];
        let suffix = (words[1], words[2]);
        histogram.insert(prefix, suffix);
    }

    for(key, value) in histogram{
        println!("{} / {:?}", key, value)
    }

// To parse just Hamlet's lines we will need to match this pattern:
// go through all the words.
// if the word is HAMLET. then have a start token,
// collect all the words until double new line,
// add an end token.


}
