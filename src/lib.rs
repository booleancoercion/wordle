use core::fmt;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LetterHint {
    Green,
    Yellow,
    Black,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hint<const WORD_LENGTH: usize>(pub [LetterHint; WORD_LENGTH]);

impl<const WORD_LENGTH: usize> Display for Hint<WORD_LENGTH> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for &hint in &self.0 {
            let c = match hint {
                LetterHint::Green => 'G',
                LetterHint::Yellow => 'Y',
                LetterHint::Black => 'B',
            };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl<const WORD_LENGTH: usize> FromStr for Hint<WORD_LENGTH> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != WORD_LENGTH {
            return Err(());
        }

        let mut hint = [LetterHint::Black; WORD_LENGTH];
        for (i, c) in s.to_ascii_uppercase().chars().enumerate() {
            hint[i] = match c {
                'G' => LetterHint::Green,
                'Y' => LetterHint::Yellow,
                'B' => LetterHint::Black,
                _ => return Err(()),
            };
        }

        Ok(Hint(hint))
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Word<const WORD_LENGTH: usize>([u8; WORD_LENGTH]);

impl<const WORD_LENGTH: usize> Display for Word<WORD_LENGTH> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for &c in &self.0 {
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

impl<const WORD_LENGTH: usize> fmt::Debug for Word<WORD_LENGTH> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "\"")?;
        for &c in &self.0 {
            write!(f, "{}", c as char)?;
        }
        write!(f, "\"")?;
        Ok(())
    }
}

impl<const WORD_LENGTH: usize> FromStr for Word<WORD_LENGTH> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != WORD_LENGTH {
            return Err(());
        }

        let mut word = [0; WORD_LENGTH];
        for (i, c) in s.to_ascii_lowercase().chars().enumerate() {
            word[i] = c as u8;
        }

        Ok(Word(word))
    }
}

#[macro_export]
macro_rules! word {
    ($string:literal) => {{
        const _LEN: usize = $string.len();
        unsafe {
            Word(
                <[::core::primitive::u8; _LEN] as ::std::convert::TryFrom<
                    &[::core::primitive::u8],
                >>::try_from(::core::primitive::str::as_bytes($string))
                .unwrap_unchecked(),
            )
        }
    }};
}

pub fn generate_hint<const WORD_LENGTH: usize>(
    base_word: Word<WORD_LENGTH>,
    guess: Word<WORD_LENGTH>,
) -> Hint<WORD_LENGTH> {
    let mut hint = [LetterHint::Black; WORD_LENGTH];

    for (i, (x, y)) in base_word.0.iter().zip(guess.0.iter()).enumerate() {
        if x == y {
            hint[i] = LetterHint::Green;
        }
    }

    for (i, &l) in guess.0.iter().enumerate() {
        if hint[i] != LetterHint::Black {
            continue;
        }

        let count = base_word
            .0
            .iter()
            .enumerate()
            .filter(|&(i, &x)| x == l && hint[i] != LetterHint::Green)
            .count();
        let mut current_count = 0;
        for (j, hint) in hint.iter_mut().enumerate() {
            if guess.0[j] == l {
                if *hint == LetterHint::Green {
                    continue;
                }
                current_count += 1;
                if current_count > count {
                    break;
                }
                *hint = LetterHint::Yellow;
            }
        }
    }

    Hint(hint)
}

pub fn partition<const WORD_LENGTH: usize>(
    dictionary: &[Word<WORD_LENGTH>],
    guess: Word<WORD_LENGTH>,
) -> HashMap<Hint<WORD_LENGTH>, Vec<Word<WORD_LENGTH>>> {
    let mut output = HashMap::new();

    for &word in dictionary {
        let hint = generate_hint(word, guess);
        output.entry(hint).or_insert_with(Vec::new).push(word);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_hint_simple() {
        let word = word!("spoke");
        let guess = word!("spear");
        let hint = generate_hint(word, guess);

        #[rustfmt::skip]
        assert_eq!(hint.0, [LetterHint::Green, LetterHint::Green, LetterHint::Yellow, LetterHint::Black, LetterHint::Black]);
    }

    #[test]
    fn generate_hint_multiples() {
        let word = word!("boots");
        let guess = word!("ovolo");
        let hint = generate_hint(word, guess);

        #[rustfmt::skip]
        assert_eq!(hint.0, [LetterHint::Yellow, LetterHint::Black, LetterHint::Green, LetterHint::Black, LetterHint::Black]);
    }

    #[test]
    fn generate_hint_extended() {
        let word = word!("clove");
        let guesses = [
            word!("pound"),
            word!("plead"),
            word!("uvula"),
            word!("carat"),
            word!("power"),
            word!("clove"),
        ];

        let hints = guesses.map(|guess| generate_hint(word, guess).0);
        #[rustfmt::skip]
        assert_eq!(hints, [
            [LetterHint::Black, LetterHint::Yellow, LetterHint::Black, LetterHint::Black, LetterHint::Black],
            [LetterHint::Black, LetterHint::Green, LetterHint::Yellow, LetterHint::Black, LetterHint::Black],
            [LetterHint::Black, LetterHint::Yellow, LetterHint::Black, LetterHint::Yellow, LetterHint::Black],
            [LetterHint::Green, LetterHint::Black, LetterHint::Black, LetterHint::Black, LetterHint::Black],
            [LetterHint::Black, LetterHint::Yellow, LetterHint::Black, LetterHint::Yellow, LetterHint::Black],
            [LetterHint::Green, LetterHint::Green, LetterHint::Green, LetterHint::Green, LetterHint::Green],
        ]);
    }

    #[test]
    fn generate_hint_multiples2() {
        let word = word!("zooks");
        let guess = word!("kooks");
        let hint = generate_hint(word, guess);

        #[rustfmt::skip]
        assert_eq!(hint.0, [LetterHint::Black, LetterHint::Green, LetterHint::Green, LetterHint::Green, LetterHint::Green]);
    }
}
