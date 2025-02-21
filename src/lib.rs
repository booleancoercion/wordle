use std::collections::HashMap;

pub const WORD_LENGTH: usize = 5;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LetterHint {
    Green,
    Yellow,
    Black,
}

pub type Hint = [LetterHint; WORD_LENGTH];

pub type Word = [u8; WORD_LENGTH];

#[macro_export]
macro_rules! word {
    ($string:literal) => {{
        const _LEN: usize = $string.len();
        const _: [(); _LEN] = [(); WORD_LENGTH];
        unsafe {
            <$crate::Word as ::std::convert::TryFrom<&[::core::primitive::u8]>>::try_from(
                ::core::primitive::str::as_bytes($string),
            )
            .unwrap_unchecked()
        }
    }};
}

pub fn generate_hint(base_word: Word, guess: Word) -> Hint {
    let mut hint = [LetterHint::Black; 5];

    for (i, &l) in guess.iter().enumerate() {
        if base_word[i] == l {
            hint[i] = LetterHint::Green;
            continue;
        }

        let count = base_word.into_iter().filter(|x| *x == l).count();
        if count == 0 {
            hint[i] = LetterHint::Black;
        } else {
            let guess_count_so_far = guess[..i].iter().filter(|x| **x == l).count();

            if guess_count_so_far < count {
                hint[i] = LetterHint::Yellow;
            } else {
                hint[i] = LetterHint::Black;
            }
        }
    }

    hint
}

pub fn partition(dictionary: &[Word], guess: Word) -> HashMap<Hint, Vec<Word>> {
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
        assert_eq!(hint, [LetterHint::Green, LetterHint::Green, LetterHint::Yellow, LetterHint::Black, LetterHint::Black]);
    }

    #[test]
    fn generate_hint_multiples() {
        let word = word!("boots");
        let guess = word!("ovolo");
        let hint = generate_hint(word, guess);

        #[rustfmt::skip]
        assert_eq!(hint, [LetterHint::Yellow, LetterHint::Black, LetterHint::Green, LetterHint::Black, LetterHint::Black]);
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

        let hints = guesses.map(|guess| generate_hint(word, guess));
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
}
