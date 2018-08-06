extern crate rand;
extern crate clap;

use clap::{Arg, App};
use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use rand::prelude::*;

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Mike McGrath <mmcgrath@pdx.edu> \nJesse Zhu <jesszhu@pdx.edu>")
        .about("Markov generator written in Rust")
        .arg(Arg::with_name("Character")
                 .required(false)
                 .takes_value(true)
                 .index(1)
                 .help("Choose character to speak"))
        .get_matches();

    //****************** FILE READ **************************
    let path = Path::new("../../text/hamlet.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(_) => println!("File read success\n"),//print!("{} contains:\n{}", display, text),
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

    let lines: Vec<_>;// = Vec::new();
    //***********SORT TEXT into speaker:[words] hashmap **************
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
    println!("Character set: {:?}", dict.keys());

    //**********Find default character (most lines) **********
    let cast = dict.keys();
    let star = cast.max_by_key(|key| dict.get(key.to_owned()).expect("no characters found").len()).unwrap();
    println!("Starring... {}\n", star);

    //***********SET CHARACTER ***************POLONIUS. HORATIO. HAMLET.
    //let speaker = "HORATIO."; //remove trailing period later
    let speaker = matches.value_of("Character").unwrap_or(star);
    match dict.get(speaker){
        Some(vocab) => lines = vocab.to_vec(),
        None => panic!("No character named: {}", speaker),
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
    let random: usize = 1; //since it's random anyway, might as well use 1 for fewer accidental panic

    let mut prefix: &str = potential_starts[random];

    let mut result = prefix.to_string();
    let mut used = Vec::new(); //does nothing atm, just storing
    let mut suffix_index;
    let mut rng = thread_rng();

    for _ in 1 ..30 {
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
