### Wordy
Your friendly text processor. a command line tool to process text in a variety of ways. under current version we are only supporting the following features:
- Replace a word with another word

In the future we will be adding more features such as:
- Count the number of words in a text
- Count the number of characters in a text
- Process a text to remove specific group of words, and so much more.

## Installation
- Clone the repo
- Run `Cargo build --release`
- Run `./target/release/wordy --help` to see the available commands

## Usage
- `cargo r -- -o "original_word" -r "new_or_replaced_word" -f "file_name"`
-  -o: original word, the word you want to replace
- -r: new or replaced word, the word you want to replace the original word with
- -f: file name, the name of the file you want to process
- -s: safe mode, this will create a new file with the replaced words and keep the original file intact, if not used the original file will be replaced with the new file