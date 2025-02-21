use std::{collections::HashMap, io::Write};

use wordle::{Hint, LetterHint, Word};

const NON_SOLUTIONS: &str = include_str!("../res/non_solutions.txt");
const SOLUTIONS: &str = include_str!("../res/solutions.txt");

fn main() {
    println!("Welcome to Wordle!\nWould you like to play adverswordle or wordle_bot?");
    print!("Enter your choice: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "adverswordle" => adverswordle(),
        "wordle_bot" => wordle_bot(),
        _ => panic!("Invalid choice"),
    }
}

fn adverswordle() {
    let (mut possibilities, solutions) = process_words();
    possibilities.extend_from_slice(&solutions);

    loop {
        print!("Enter your word: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let word: Word = input.trim().parse().unwrap();

        let partitions = wordle::partition(&possibilities, word);
        let worst_partition = partitions
            .into_iter()
            .max_by_key(|x| {
                if x.0.0 == [LetterHint::Green; 5] {
                    0
                } else {
                    x.1.len()
                }
            })
            .unwrap();
        println!(
            "Worst partition: {} ({} possibilities)",
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
            unreachable!("Somehow eliminated all of the words!")
        } else if possibilities.len() == 1 {
            println!("The word is {}", possibilities[0]);
            break;
        }

        let mut guesses: Vec<(Word, HashMap<Hint, Vec<Word>>)> = non_solutions
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

        possibilities = guesses.pop().unwrap().1.remove(&Hint(hint)).unwrap();
    }
}

fn process_words() -> (Vec<Word>, Vec<Word>) {
    (
        NON_SOLUTIONS.lines().map(|x| x.parse().unwrap()).collect(),
        SOLUTIONS.lines().map(|x| x.parse().unwrap()).collect(),
    )
}
