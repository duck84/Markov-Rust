# Markov-Rust
A Markov chain generator written in Rust. Chat with a Shakespeare character, or add your own text file!

## Features
- Select any text file from /text folder
- Character selection
- Randomized Markov output, based on what you type

## Usage
- Add your own files to the /text folder
Parser currently assigns words to speakers based on the `^[\s]*[A-Z]*\s?[A-Z]{3,}[\.:]` regex pattern.
e.g.
- Starting at the beginning of each line
- Searches for CAPS words (up to 2)
- Ends at a period or colon
```
HAMLET.
HAMLET:
SIR ANDREW.
```

- `cargo run` to operate

## Example output
cargo run
(when in /reader/src folder)
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

## Unimplemented Features
- Loop mitigation
- Characters talking to each other (by scanning previous message for prefixes to use)
- Evaluate an input string to see which character it fits the most
- More natural start/stopping points
- Improve 'chatbot' feature by recognizing only "important" words in user speech (remove common words i.e. 'a', 'the')
- Improve filepath recognition from relative to current directory to relative to base project directory


## Issues

- parsing

Some non-character words end up matching character regex

