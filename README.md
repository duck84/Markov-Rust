# Markov-Rust
A Markov chain generator written in Rust

## Features
- Read from file
- Generalized Parsing and reading from file source
- Character selection
- Histogram prefix-suffix storage (1 prefix: 2 suffix)
- Randomized Markov output

## Usage
- Add your own files to the /text folder
Parser currently assigns words to speakers based on the `^\n[A-Z]{3,}[.:]$` regex pattern.

- cargo run

## Example output
cargo run
```
Which play do you want to use:
hamlet
romeo
trump
twelfth
hamlet
File read success

Full Character set: ["HEADER", "QUEEN", "MESSENGER", "LUCIANUS", "CAPTAIN", "ALL", "HAMLET", "BARNARDO", "LORD", "PRIEST", "SERVANT", "MARCELLUS", "HORATIO", "KING", "DANES", "PROLOGUE", "GENTLEMAN", "POLONIUS", "GUILDENSTERN", "BOTH", "ROSENCRANTZ", "OPHELIA", "GHOST", "REYNALDO", "OSRIC", "LAERTES", "FORTINBRAS", "FRANCISCO", "VOLTEMAND"]
Which character do you want to talk to? (Press 'enter' for default)

HAMLET : 11956 words
KING : 4323 words
POLONIUS : 2783 words
HORATIO : 1959 words
LAERTES : 1375 words

Starring... HAMLET

HAMLET says...
makes them Like wonder-wounded hearers? This is th'imposthume of much offence Touching this employment. They are no tongues else to fat us, till ground, Singeing his pate against the burning zone, Make Ossa like Mars, to and GUILDENSTERN. We'll wait upon you. No traveller returns, puzzles

```

## Unimplemented Features / TODO
- Loop mitigation
- Characters talking to each other (by scanning previous message for prefixes to use)
- Evaluate an input string to see which character it fits the most
- Menu selection system
- More natural start/stopping points
- Code cleanup to functions for legibility
- tests for each function

## Change log

8/3/18
- Parsing
-- parsing is now generalized to record all words for each character.
-- Hashmap of CHARACTER : vector of strings


8/4/18
- Loop Issues
-- Previously histogram was overwriting values as a `HashMap<&str, (&str, &str)>`
  now a `HashMap<&str, Vec<(&str, &str)>>`
-- Now randomly selects a suffix tuple if more than one is present.


8/5/18
- Command line character selection
  - Using Clap crate
  - Defaults to character with most lines if argument not given


## Issues

- parsing

Non-speech words entered into vocab (PLAYER)
```
blow them to lament with you: And you, my cause, And with such ambiguous giving out, to know himself. What's his fines, and a Queen. PLAYER Wormwood, wormwood. PLAYER Wormwood, wormwood. PLAYER Ophelia._]
```

Scene, empty, and acts (I, II, III...) are recognized as characters
```
"MARCELLUS.", "", "SCENE.", "GUILDENSTERN.", "PLAYER.", "I."
```
