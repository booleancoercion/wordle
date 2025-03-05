use std::{collections::HashMap, io::Write};

use wordle::{Hint, LetterHint, Word};

const NON_SOLUTIONS: &str = include_str!("../res/non_solutions.txt");
const SOLUTIONS: &str = include_str!("../res/solutions.txt");
const WORD_LENGTH: usize = 5;

fn main() {
    println!("Welcome to Wordle!\nWhich games are we playing? (wordle/adverswordle)");
    loop {
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "adverswordle" => adverswordle(),
            "wordle" => wordle_bot(),
            _ => {
                println!("Invalid choice. Please enter 'adverswordle' or 'wordle'.\n");
                continue;
            }
        };
        break;
    }
}

fn adverswordle() {
    let (mut possibilities, solutions) = process_words();
    possibilities.extend_from_slice(&solutions);

    loop {
        let word = loop {
            print!("Enter your word: ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let Ok(word) = input.trim().parse::<Word<WORD_LENGTH>>() else {
                println!("Invalid word. Please enter a word of length 5.");
                continue;
            };
            break word;
        };

        let partitions = wordle::partition(&possibilities, word);
        let worst_partition = partitions
            .into_iter()
            .max_by_key(|x| {
                if x.0.0 == [LetterHint::Green; WORD_LENGTH] {
                    0
                } else {
                    x.1.len()
                }
            })
            .unwrap();
        println!(
            "Worst partition: {} ({} possibilities)\n",
            worst_partition.0,
            worst_partition.1.len()
        );

        possibilities = worst_partition.1;

        if possibilities.len() == 1 {
            println!("Game over. The word is {}", possibilities[0]);
            break;
        }
    }
}

fn wordle_bot() {
    let (non_solutions, solutions) = process_words();

    let mut possibilities = solutions.clone();
    loop {
        if possibilities.is_empty() {
            println!(
                "All of the words have been eliminated! Either one of the inputs is incorrect or the word is not in the dictionary."
            );
            break;
        } else if possibilities.len() == 1 {
            println!("The word is {}", possibilities[0]);
            break;
        }

        let guesses: Vec<(
            Word<WORD_LENGTH>,
            HashMap<Hint<WORD_LENGTH>, Vec<Word<WORD_LENGTH>>>,
        )> = non_solutions
            .iter()
            .chain(solutions.iter())
            .map(|&guess| (guess, wordle::partition(&possibilities, guess)))
            .collect();

        let mut best_guess = guesses.into_iter().min_by_key(|x| (partition_score(&x.1), !possibilities.contains(&x.0))).unwrap();

        println!(
            "Best guess: {} ({} partitions)",
            best_guess.0,
            best_guess.1.len()
        );

        let hint = 'outer: loop {
            print!(
                "Enter hint for best guess (G for green, Y for yellow, B for black, e.g. 'GBYBB'): "
            );
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<Hint<WORD_LENGTH>>() {
                Ok(hint) => {
                    if best_guess.1.contains_key(&hint) {
                        break 'outer hint;
                    } else {
                        println!(
                            "Invalid hint. This hint is inconsistent with the previous hints."
                        );
                        continue;
                    }
                }
                Err(_) => {
                    println!("Invalid hint. Please enter a hint of length 5.");
                    continue;
                }
            }
        };

        possibilities = best_guess.1.remove(&hint).unwrap();
        println!();
    }
}

fn partition_score(partition: &HashMap<Hint<WORD_LENGTH>, Vec<Word<WORD_LENGTH>>>) -> usize {
    partition.values().map(|x| x.len().pow(2)).sum()
}

fn process_words() -> (Vec<Word<WORD_LENGTH>>, Vec<Word<WORD_LENGTH>>) {
    (
        NON_SOLUTIONS
            .lines()
            .filter(|x| x.len() == WORD_LENGTH)
            .map(|x| x.parse().unwrap())
            .collect(),
        SOLUTIONS
            .lines()
            .filter(|x| x.len() == WORD_LENGTH)
            .map(|x| x.parse().unwrap())
            .collect(),
    )
}
