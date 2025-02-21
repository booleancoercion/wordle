use std::{collections::HashMap, io::Write};

use wordle::{Hint, LetterHint, Word};

const NON_SOLUTIONS: &str = include_str!("../res/non_solutions.txt");
const SOLUTIONS: &str = include_str!("../res/solutions.txt");
const WORD_LENGTH: usize = 5;

fn main() {
    println!("Welcome to Wordle!\nWould you like to play adverswordle or wordle_bot?");
    loop {
        print!("Enter your choice: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "adverswordle" => adverswordle(),
            "wordle_bot" => wordle_bot(),
            _ => {
                println!("Invalid choice. Please enter 'adverswordle' or 'wordle_bot'.\n");
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

        let mut guesses: Vec<(
            Word<WORD_LENGTH>,
            HashMap<Hint<WORD_LENGTH>, Vec<Word<WORD_LENGTH>>>,
        )> = non_solutions
            .iter()
            .chain(solutions.iter())
            .map(|&guess| (guess, wordle::partition(&possibilities, guess)))
            .collect();

        guesses.sort_unstable_by_key(|x| x.1.len());

        println!(
            "Worst guess: {} ({} partitions)",
            guesses[0].0,
            &guesses[0].1.len()
        );

        println!(
            "Best guess: {} ({} partitions)",
            guesses.last().unwrap().0,
            &guesses.last().unwrap().1.len()
        );

        let hint = 'outer: loop {
            print!(
                "Enter hint for best guess (G for green, Y for yellow, B for black, e.g. 'GBYBB'): "
            );
            std::io::stdout().flush().unwrap();

            let mut hint = [wordle::LetterHint::Black; WORD_LENGTH];
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            for (i, c) in input.trim().chars().enumerate() {
                hint[i] = match c {
                    'G' => wordle::LetterHint::Green,
                    'Y' => wordle::LetterHint::Yellow,
                    'B' => wordle::LetterHint::Black,
                    _ => {
                        println!("Invalid hint. Please use the correct format.");
                        continue 'outer;
                    }
                };
            }
            break hint;
        };

        possibilities = guesses.pop().unwrap().1.remove(&Hint(hint)).unwrap();
        println!();
    }
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
