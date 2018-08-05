extern crate rand;
use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use rand::prelude::*;

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
        Ok(_) => println!("File read success"),//print!("{} contains:\n{}", display, text),
    }

    //Filtering out non-speech words that don't start/end with valid chars
    //Doesn't catch stuff like "_Hic et ubique?_"
    let textstring = text.replace("’","'"); //terminal output can't print the first one
    let tokens: Vec<&str> = textstring.split_whitespace()
                                        .collect::<Vec<_>>()
                                        .into_iter()
                                        .filter(|word| word.chars()
                                                            .next()
                                                            .unwrap()
                                                            .is_alphabetic()
                                                        &&word.chars()
                                                                .last()
                                                                .unwrap() != '_')
                                        .collect();

    let mut lines = Vec::new();

    let mut dict = HashMap::new();
    let mut key = "";
    for word in &tokens{
        //Finds capitalized NAMES. and sets KEY
        if word == &word.to_uppercase()
            && word.ends_with("."){
            key = word;
        }
        //ELSE push WORD to DICT
        else{
            dict.entry(key).or_insert(Vec::new()).push(word.to_owned());
        }
    }


    //***********SET CHARACTER ***************POLONIUS. HORATIO. HAMLET.
    let speaker = "HORATIO."; //remove trailing period later
    match dict.get(speaker){
        Some(vocab) => lines = vocab.to_vec(),
        None => println!("No character named: {}", speaker),
    }
    println!("{} says...", speaker);

    //***********HISTOGRAM ***************

    let group = lines.windows(3);

    let mut histogram: HashMap<&str, Vec<(&str, &str)>> = HashMap::new();

    for words in group {
        let prefix = words[0];
        let suffix = (words[1], words[2]);
        //histogram.insert(prefix, suffix);
        histogram.entry(prefix).or_insert(Vec::new()).push(suffix);
    }


    let potential_starts: Vec<&&str> = histogram.keys().collect();
    let random: usize = 45;

    let mut prefix: &str = potential_starts[random];

    let mut result = prefix.to_string();
    let mut used = Vec::new(); //does nothing atm, just storing
    let mut suffix_index;
    let mut rng = thread_rng();

    for _ in 1 ..50 {
        match histogram.get(&prefix) {
            Some(suffixes_list) => {
                used.push(prefix);

                suffix_index = rng.gen_range(0, suffixes_list.len());
                //println!("length:random {}:{}", suffixes_list.len(), suffix_index);
                let mut suffixes = suffixes_list[suffix_index];

                result = result + " " + suffixes.0 + " " + suffixes.1;
                prefix = suffixes.1;
            },
            None => {
                break;
            }

        }
    }
    print!("{}\n", result);
    //println!("{:?}", used);
    //println!("{:?}", histogram);
}
