# Markov-Rust
A Markov chain generator written in Rust

## Features
- Read from file
- Generalized Parsing
- Character selection
- Histogram prefix-suffix storage (1 prefix: 2 suffix)
- Randomized Markov output

## Example output
cargo run HORATIO.
```
Character set: ["LORD.", "LORDS.", "HORATIO.", "CAPTAIN.", "V.", "FRANCISCO.", "PRIEST.", "HAMLET.", "FORTINBRAS.", "OSRIC.", "GUILDENSTERN.", "SCENE.", "OPHELIA.", "SERVANT.", "CLOWN.", "LUCIANUS.", "AMBASSADOR.", "BARNARDO.", "PROLOGUE.", "III.", "LAERTES.", "II.", "REYNALDO.", "MESSENGER.", "MARCELLUS.", "IV.", "", "ROSENCRANTZ.", "PLAYER.", "VOLTEMAND.", "VI.", "I.", "POLONIUS.", "BOTH.", "SAILOR.", "DANES.", "GHOST.", "KING.", "VII.", "QUEEN.", "ALL.", "GENTLEMAN."]

Starring... HAMLET.

HORATIO. says...
view, And let us hear Of carnal, bloody and you from our sight. As I beseech you. Ay, good turn for upon my lord? Good my lord, the King your father. Season your father; These are but wild and ordnance shot off within._] What does the drum come hither? within._] Enter Fortinbras, the night, Been thus encounter'd. A figure like your search. Not when I boarded them. SCENE Custom hath made probation. So Guildenstern and speak not from Lord Hamlet. Enter Osric. No, by no ghost, my lord. My lord, I Truly deliver. Of that I shall have also
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
