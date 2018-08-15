// Copyright Mike McGrath and Jesse Zhu 2018
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

extern crate rand;
//extern crate clap;
extern crate regex; // 1.0.2
extern crate inflector;
extern crate colored;

use regex::Regex;
//use clap::{Arg, App};
use std::io::Read;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::fs;
use std::collections::HashMap;
use rand::prelude::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::collections::hash_map::RandomState;
use inflector::Inflector;
use colored::*;

/// A function that prints the title screen of the program.
fn start(){
    println!("\n\n\n");
    println!("************************************************************");
    println!("***                                                      ***");
    println!("***              Shakespeare Chat Bot                    ***");
    println!("***                       by                             ***");
    println!("***            A Markov Chain Generator                  ***");
    println!("***                                                      ***");
    println!("************************************************************");
    println!("\nPick a play and character to chat with.\nWhen you are done type 'STOP' to exit the program.\n")
}

/// A function that opens a text file and reads the file to text.
/// * 'file_path' - A &str of the file path used to find the file.
/// * 'text' - A String that contains the Shakespeare play from
///            the file.
fn reader(file_path: &str) -> String {
    //****************** FILE READ **************************
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => {println!("could not open {}: {}", display, why.description());
        return reader(&play_selector());},//panic!("could not open {}: {}", display, why.description()),

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
/// It filters out non-spoken words and splits on NEWLINE.
/// * 'text' - The text you want to tokenize. Passed in at a &str.
///            It should be a Shakespeare text from a file.
/// * 'tokens' - A Vec of Strings of each LINE from the original text.
fn tokenizer(text: &str) -> Vec<String> {
    //Filtering out non-speech words i.e. _blah_ or [blah] and [SCENE I.]
    let re = Regex::new(r"\[.+\]|_.+_|SCENE.+\.|Scene.+\.|ACT[ A-Z]{1,4}\.").unwrap();
    let filtered = re.replace_all(text, "");

    let tokens: Vec<String> = filtered.split('\n')
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

    //***********SET CHARACTER ***************POLONIUS. HORATIO. HAMLET.
    //let speaker = "HORATIO."; //remove trailing period later
    let speaker2: &str;
    if speaker == "" {
        println!("Starring... {}\n", star);
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
    println!("\n{} {}", speaker2.to_string().blue(), "says...".blue());
    lines
}

/// A function that reads the string-array contents and records each word
/// into a `speaker to words` hashmap.
/// * 'tokens' - All the words of the play as a Vec of Strings.
/// * 'dict' - A hashmap containing key = speaker, value = [words]

fn lines_reader(tokens: &Vec<String>) -> (HashMap<&str, Vec<String>, RandomState>) {

    let re_char = Regex::new(r"^[\s]*[A-Z]*\s?[A-Z]{3,}[\.:]").unwrap();
//***********SORT TEXT into speaker:[words] hashmap **************
    let mut dict = HashMap::new();
    let mut key = "HEADER";
//Everything before first speaker stored in "HEADER" string
//Finds capitalized NAMES. and sets KEY
//ELSE push WORD to DICT
    for line in tokens{
        let mut splitline;
        let matches = re_char.find(line);

        match matches{
            Some(x) => {
                let speaker = x.as_str().trim();
                //println!("match found {}", speaker);
                let len = speaker.len();
                key = &speaker[0..len-1];
                //println!("key: {} line: {}", key, line);
                splitline = line.split(speaker).last().unwrap().to_owned();
                //println!("{:?}", splitline);
            },

            None => splitline = line.to_owned(),
        };
        let words = splitline.split_whitespace();

        for word in words {
            dict.entry(key).or_insert(Vec::new()).push(word.to_owned());
        }
    }
    println!("Full Character set: {:?}", dict.keys());
    dict
}

/// A function that takes a vector of Strings, windows them into groups of 4, and then starts
/// with a random starting word. The word is used as a prefix (key) in the hashmap of prefix:suffixes
/// and the last suffix is used as the new key in building the output String. The reply is parsed
/// to have the character generate a chain based on one of the words the user said.
/// * 'lines' A vector of single word Strings
/// * 'reply' A String that the user inputs. On the first call this is always empty, but afterwords
///           it is the users questions or comments to the character.
fn markov_generator(lines: &Vec<String>, reply: String) {
    let group = lines.windows(4);
    let mut histogram: HashMap<&str, Vec<(&str, &str, &str)>> = HashMap::new();
    for words in group {
        let prefix = &words[0];
        let suffix = (words[1].as_str(), words[2].as_str(), words[3].as_str());
        //histogram.insert(prefix, suffix);
        histogram.entry(&prefix).or_insert(Vec::new()).push(suffix);
    }

    let first_starts: Vec<&&str> = histogram
        .keys()
        .filter_map(|word| {
            let c = word
                .chars()
                .next()
                .unwrap();
            if c.is_ascii_uppercase() {
                Some(word)
            }
            else { None }
        })
        .collect();

    let checked_starts: Vec<&&str> = histogram.keys().collect();

    let random: usize = 1;
//since it's random anyway, might as well use 1 for fewer accidental panic
    let prefix_string;
    let mut prefix: &str = "";
    let split = reply.split_whitespace();
    let mut vec: Vec<&str> = split.collect();
//    vec.reverse();                 could reverse the vec to 'find' the object of the sentence
    thread_rng().shuffle(&mut vec); // Randomize the sentence to make the prefix more random
    if reply.is_empty() {
        prefix = first_starts[random];
    }else { for mut word in vec{
            if checked_starts.contains(&&word){
                let new_word = &word.to_title_case();
                prefix_string = new_word.to_string();
                prefix = prefix_string.as_str();
                break;
            } else {
                prefix = first_starts[random];
            }
        }
    }
    let mut result = prefix.to_string();
    let mut used = Vec::new();
//does nothing atm, just storing
    let mut suffix_index;
    let mut rng = thread_rng();
    let chain = rng.gen_range(2, 16);
    for _ in 1..chain {
        match histogram.get(prefix) {
            Some(suffixes_list) => {
                used.push(prefix);
                suffix_index = rng.gen_range(0, suffixes_list.len());
                let mut suffixes = suffixes_list[suffix_index];
                result = result + " " + suffixes.0 + " " + suffixes.1 + " " + suffixes.2;
                prefix = &suffixes.2;
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
/// * 'path' - A String of the path to the text of the plays.
fn play_selector() -> String {
    let paths = fs::read_dir("../../text/").unwrap();
    let mut allplays = Vec::new();
    for path in paths {
        if let Ok(path) = path {
            // Here, `path` is a `DirEntry`.
            allplays.push(path
                .file_name()
                .into_string()
                .expect("pathname not convertable to string")
            .split(".").next().unwrap().to_owned());
        }
        //println!("{}", allplays.join("\n"));
    }
    let plays = allplays.join("\n");

    let mut input = String::new();
    println!("\nWhich play do you want to use:");
    println!("{}", plays.to_uppercase());

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    let input = input.to_lowercase();
    let path: String;
    let pathbuilder = "../../text/".to_owned();

    if plays.contains(&input){
        path = format!("{}{}{}",pathbuilder, input, ".txt")
    }
    else{
        println!("Please select a valid play\n");
        return play_selector();
    }
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
    println!("Which character do you want to talk to? (Press 'enter' for default)\n");
    // Top 3 characters by word count
    let keys = dict.keys();
    let mut keysize: Vec<(&str, usize)> = Vec::new();
    for key in keys{
        keysize.push((&key, dict.get(key).unwrap().len()));
    }
    keysize.sort_by_key(|k| k.1);
    keysize.reverse();
    //println!("{:?}", keysize);
    for i in 0..5 {
        match keysize.get(i) {
            Some(x) => println!("{} : {} words", x.0, x.1),
            None => break,
        };
    }

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    let input = input.to_uppercase();
    if dict.contains_key(input.as_str()) || input == ""{
        input
    }else {
        character_selector(dict)
    }
}

/// A function that takes in the users input and returns it as a String.
/// This is used to set the reply variable.
/// * 'input' A cleaned up version of the user input.
fn talk()-> String{
    let mut input = String::new();
    println!("\n{}", "What do you want to say?".green());
    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid string");
    if let Some('\n') = input.chars().next_back() {
        input.pop();
    }
    input
}


fn main() {
    // let _matches = App::new("Markov Generator")
    //     .version("0.1.0")
    //     .author("Mike McGrath <mmcgrath@pdx.edu> \nJesse Zhu <jesszhu@pdx.edu>")
    //     .about("Markov generator written in Rust")
    //     .arg(Arg::with_name("Character")
    //              .required(false)
    //              .takes_value(true)
    //              .index(1)
    //              .help("Choose character to speak"))
    //     .get_matches();
    let mut reply = "".to_string();
    start();
    let path = play_selector();
    let text = reader(&path);
    let tokens = tokenizer(text.as_str());
    let dict = lines_reader(&tokens);
    let speaker = character_selector(&dict);
    let lines: Vec<String> = parser(speaker.as_str(), &dict);
    markov_generator(lines.as_ref(), reply);
    reply = talk();
    while &reply != "STOP"{
        println!("\n{} {}", speaker.blue(), "says...".blue());
        markov_generator(lines.as_ref(), reply);
        reply = talk();
    }
}

#[test]
fn test1() {
    let mystring = "SPEAKERONE. I ate a banana in CANADA.\n SPEAKERTWO. ME. TOO.";
    let tokens = tokenizer(mystring);
    println!("{:?}", tokens);

    assert_eq!(tokens, ["SPEAKERONE. I ate a banana in CANADA.", " SPEAKERTWO. ME. TOO."
    ], "tokenizer assertion");

    let mydict = lines_reader(&tokens);
    assert_eq!(2, mydict.len(), "Asserting # speakers = 2");
    println!("{:?}", mydict);

    assert_eq!(6, mydict.get("SPEAKERONE").expect("line_reader/tokenizer test").len(), "Should recognize
    CANADA. as a word, not a speaker");
}
