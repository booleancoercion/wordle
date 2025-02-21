use std::{collections::HashMap, io::Write};

use wordle::{Hint, Word};

const WORDS: &str = include_str!("../res/valid-wordle-words.txt");

fn main() {
    let words = process_words();

    let mut possibilities = words.clone();
    // let mut current_hints: Vec<(Hint, Word)> = vec![];
    loop {
        if possibilities.is_empty() {
            unreachable!("Somehow eliminated all of the words!")
        } else if possibilities.len() == 1 {
            println!(
                "The word is {}",
                String::from_utf8_lossy(&possibilities[0])
            );
            break;
        }

        let mut guesses: Vec<(Word, HashMap<Hint, Vec<Word>>)> = words
            .iter()
            .map(|&guess| (guess, wordle::partition(&possibilities, guess)))
            .collect();

        guesses.sort_unstable_by_key(|x| x.1.len());

        println!(
            "Worst guess: {} ({} partitions)",
            String::from_utf8_lossy(&guesses[0].0),
            &guesses[0].1.len()
        );

        println!(
            "Best guess: {} ({} partitions)",
            String::from_utf8_lossy(&guesses.last().unwrap().0),
            &guesses.last().unwrap().1.len()
        );

        print!("Enter hint for best guess: ");
        std::io::stdout().flush().unwrap();
        let hint = {
            let mut hint = [wordle::LetterHint::Black; wordle::WORD_LENGTH];
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            for (i, c) in input.trim().chars().enumerate() {
                hint[i] = match c {
                    'G' => wordle::LetterHint::Green,
                    'Y' => wordle::LetterHint::Yellow,
                    'B' => wordle::LetterHint::Black,
                    _ => panic!("Invalid hint character"),
                };
            }
            hint
        };

        possibilities = guesses.pop().unwrap().1.remove(&hint).unwrap();
    }
}

fn process_words() -> Vec<Word> {
    WORDS
        .lines()
        .map(|x| x.as_bytes().try_into().unwrap())
        .collect()
}
