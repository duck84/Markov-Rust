// Copyright Mike McGrath and Jesse Zhu 2018
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rand;
extern crate clap;
extern crate regex; // 1.0.2

use regex::Regex;
use clap::{Arg, App};
use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use rand::prelude::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::collections::hash_map::RandomState;

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
/// It filters out non-spoken words and splits on whitespace.
/// * 'text' - The text you want to tokenize. Passed in at a &str.
///            It should be a Shakespeare text from a file.
/// * 'tokens' - A Vec of Strings of each word from the original text.
fn tokenizer(text: &str) -> Vec<String> {
    //Filtering out non-speech words i.e. _blah_ or [blah] and [SCENE I.]
    let re = Regex::new(r"\[.+\]|_.+_|SCENE.+\.|Scene.+\.").unwrap();
    let filtered = re.replace_all(text, "");

    let tokens: Vec<String> = filtered.split_whitespace()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    tokens
}

/// A function that goes through the text and finds every character
/// to create a character set. From this set, the function finds
/// all the lines for each character. Then, the function finds
/// the star of the play (the person with the most lines). Finally,
/// the function returns the stats lines, or another character's
/// line as entered on the command line.
/// * 'tokens' - All the words of the play as a Vec of Strings.
/// * 'speaker' - The character entered on the command line. If
///               none, then the star.
/// * 'lines' - A Vec of Strings of all the lines of the character
///             selected.
fn parser<'a>(speaker: &'a str, dict: &HashMap<&str, Vec<String>, RandomState>) -> Vec<String>{
    let lines;

    //**********Find default character (most lines) **********
    let cast = dict.keys();
    let star = cast.max_by_key(|key| dict.get(key.to_owned()).expect("no characters found").len()).unwrap();
    println!("Starring... {}\n", star);
    //***********SET CHARACTER ***************POLONIUS. HORATIO. HAMLET.
    //let speaker = "HORATIO."; //remove trailing period later
    let speaker2: &str;
    if speaker == "" {
        speaker2 = star;
        match dict.get(star){
            Some(vocab) => lines = vocab.to_vec(),
            None => panic!("No character named: {}", speaker),
        }
    } else {
        speaker2 = speaker;
        match dict.get(speaker) {
            Some(vocab) => lines = vocab.to_vec(),
            None => panic!("No character named: {}", speaker),
        }
    }
    println!("{} says...", speaker2);
    lines
}

fn lines_reader(tokens: &Vec<String>) -> (HashMap<&str, Vec<String>, RandomState>) {
    let re_char = Regex::new(r"^[A-Z]+[\.:]$").unwrap();
//    let lines: Vec<_>;
//***********SORT TEXT into speaker:[words] hashmap **************
    let mut dict = HashMap::new();
    let mut key = "";
//Everything before first speaker stored in empty string
    for word in tokens {
        //Finds capitalized NAMES. and sets KEY
        if re_char.is_match(word) {
            key = word;
        }
            //ELSE push WORD to DICT else {
            dict.entry(key).or_insert(Vec::new()).push(word.to_owned());
        }
    println!("Character set: {:?}", dict.keys());
    dict
}


fn markov_generator(lines: Vec<String>) {
    let group = lines.windows(4);
    let mut histogram: HashMap<&str, Vec<(&str, &str, &str)>> = HashMap::new();
    for words in group {
        let prefix = &words[0];
        let suffix = (words[1].as_str(), words[2].as_str(), words[3].as_str());
        //histogram.insert(prefix, suffix);
        histogram.entry(&prefix).or_insert(Vec::new()).push(suffix);
    }
    let potential_starts: Vec<&&str> = histogram.keys().collect();
    let random: usize = 1;
//since it's random anyway, might as well use 1 for fewer accidental panic
    let mut prefix: &str = potential_starts[random];
    let mut result = prefix.to_string();
    let mut used = Vec::new();
//does nothing atm, just storing
    let mut suffix_index;
    let mut rng = thread_rng();
    for _ in 1..12 {
        match histogram.get(&prefix) {
            Some(suffixes_list) => {
                used.push(prefix);
                suffix_index = rng.gen_range(0, suffixes_list.len());
                let mut suffixes = suffixes_list[suffix_index];
                result = result + " " + suffixes.0 + " " + suffixes.1 + " " + suffixes.2;
                prefix = suffixes.2;
                if prefix.contains(".") | prefix.contains("!") | prefix.contains("?") {
                    break;
                }
            },
            None => {
                break;
            }
        }
    }
    print!("{}\n", result);
}

/// A function that takes no arguments and returns a path to the text of a play.
/// The function runs in the beginning of the problem and asks the user
/// for the play they would like to use. The user selects the play from the choices
/// and the function matches the input with lower case names of plays.
///               none, then the star.
/// * 'path' - A &str of the path to the text of the plays.
fn play_selector() -> &'static str {
    let mut input = String::new();
    println!("Which play do you want to use:\nHamlet\nRomeo\nTwelfth Night\n");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    let input = input.to_lowercase();
    let path: &str;
    match input.as_ref() {
        "hamlet" => path = "../../text/hamlet.txt",
        "romeo" => path = "../../text/romeo.txt",
        "twelfth night" => path = "../../text/twelfth.txt",
        _ => {println!("Please select a valid play\n"); return play_selector()}
    };
    path
}
/// A function that takes the hash map of characters and lines and asks the user
/// to pick a character from the play to talk to. A potential refactor would to
/// pass just the key values to the function.
/// * 'dict' - A reference to a hash map of the lines in the play as strings.
/// * 'input' - A string that the user inputs. It has to match the character choice
/// exactly.
fn character_selector(dict: &HashMap<&str, Vec<String>, RandomState>) -> String {
    let mut input = String::new();
    println!("Which character do you want to talk to?\n");
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
//    let input = input.to_lowercase();
    if dict.contains_key(input.as_str()){
        input
    }else {
        return character_selector(dict);
    }
}

fn main() {
    let _matches = App::new("Markov Generator")
        .version("0.1.0")
        .author("Mike McGrath <mmcgrath@pdx.edu> \nJesse Zhu <jesszhu@pdx.edu>")
        .about("Markov generator written in Rust")
        .arg(Arg::with_name("Character")
                 .required(false)
                 .takes_value(true)
                 .index(1)
                 .help("Choose character to speak"))
        .get_matches();
    let path = play_selector();
    let text = reader(path);
    let tokens = tokenizer(text.as_str());
    let dict = lines_reader(&tokens);
    let speaker = character_selector(&dict);
    let lines: Vec<String> = parser(speaker.as_str(), &dict);
    markov_generator(lines);
}