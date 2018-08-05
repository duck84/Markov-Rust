# Markov-Rust
A Markov chain generator written in Rust

## Features
- Read from file
- Generalized Parsing
- Character selection
- Histogram prefix-suffix storage (1 prefix: 2 suffix)
- Randomized Markov output

## Unimplemented Features
- Loop mitigation
- Characters talking to each other (by scanning previous message for prefixes to use)
- Evaluate an input string to see which character it fits the most
- Menu selection system
- More natural start/stopping points

## Change log

8/3/18
-Parsing
--parsing is now generalized to record all words for each character.
--Hashmap of CHARACTER : vector of strings


8/4/18
-Loop Issues
--Previously histogram was overwriting values as a `HashMap<&str, (&str, &str)>`
  now a `HashMap<&str, Vec<(&str, &str)>>`
--Now randomly selects a suffix tuple if more than one is present.


## Issues

parsing
```
blow them to lament with you: And you, my cause, And with such ambiguous giving out, to know himself. What's his fines, and a Queen. PLAYER Wormwood, wormwood. PLAYER Wormwood, wormwood. PLAYER Ophelia._]
```
