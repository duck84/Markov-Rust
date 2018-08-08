// Copyright Mike McGrath and Jesse Zhu 2018
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rand;
extern crate clap;

use clap::{Arg, App};
use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use rand::prelude::*;

/// A function that opens a text file and reads the file to text.
/// * 'file_path' - A &str of the file path used to find the file.
/// * 'text' - A String that contains the Shakespeare play from
///            the file.
fn reader(file_path: &str) -> String {
    //****************** FILE READ **************************
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Err(why) => panic!("could not open {}: {}", file_path, why.description()),
        Ok(_) => println!("File read success\n"),//print!("{} contains:\n{}", display, text),
    }
    text
}
/// A function that takes a &str of text and returns a Vec of Strings.
/// It filters out no alphabetic words and splits on whitespace.
/// * 'text' - The text you want to tokenize. Passed in at a &str.
///            It should be a Shakespeare text from a file.
/// * 'tokens' - A Vec of Strings of each word from the original text.
fn tokenizer(text: &str) -> Vec<String> {

    //Filtering out non-speech words that don't start/end with valid chars
    //Doesn't catch stuff like "_Hic et ubique?_"
    let textstring = text.replace("’", "'"); //terminal output can't print the first one

    let tokens: Vec<String> = textstring.split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|word| word.chars()
            .next()
            .unwrap()
            .is_alphabetic()
            &&word.chars()
            .last()
            .unwrap() != '_')
        .map(|x| x.to_string())
        .collect();

    tokens
}

fn main() {
    let matches = App::new("Markov Generator")
        .version("0.1.0")
        .author("Mike McGrath <mmcgrath@pdx.edu> \nJesse Zhu <jesszhu@pdx.edu>")
        .about("Markov generator written in Rust")
        .arg(Arg::with_name("Character")
                 .required(false)
                 .takes_value(true)
                 .index(1)
                 .help("Choose character to speak"))
        .get_matches();

    let path = "../../text/twelfth.txt";
    let text = reader(path);
    let tokens = tokenizer(text.as_str());


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

    let group = lines.windows(4);

    let mut histogram: HashMap<&str, Vec<(&str, &str, &str)>> = HashMap::new();

    for words in group {
        let prefix = words[0].as_str();
        let suffix = (words[1].as_str(), words[2].as_str(), words[3].as_str());
        //histogram.insert(prefix, suffix);
        histogram.entry(&prefix).or_insert(Vec::new()).push(suffix);
    }


    let potential_starts: Vec<&&str> = histogram.keys().collect();
    let random: usize = 1; //since it's random anyway, might as well use 1 for fewer accidental panic

    let mut prefix: &str = potential_starts[random];

    let mut result = prefix.to_string();
    let mut used = Vec::new(); //does nothing atm, just storing
    let mut suffix_index;
    let mut rng = thread_rng();

    for _ in 1 ..10 {
        match histogram.get(&prefix) {
            Some(suffixes_list) => {
                used.push(prefix);

                suffix_index = rng.gen_range(0, suffixes_list.len());
                //println!("length:random {}:{}", suffixes_list.len(), suffix_index);
                let mut suffixes = suffixes_list[suffix_index];
                result = result + " " + suffixes.0 + " " + suffixes.1 + " " + suffixes.2;
                prefix = suffixes.2;
            },
            None => {
                break;
            }

        }
    }
    print!("{}\n", result);
//    for (key, value) in &histogram{
//        println!("{}, {:?}", key, value);
//    }
    //println!("{:?}", used);
    //println!("{:?}", histogram);
}
