use crate::words::WordList;
use ordered_float::NotNan;
use std::cmp::Reverse;

#[derive(Clone, Debug)]
struct Mnemonic {
    phrase: String,
    word_count: usize,
    score: NotNan<f64>,
}

impl Mnemonic {

}

pub fn propose_mnemonics (word_list: &WordList, label: &str, number: &str, max_words: usize) {
    let number = digits_only(number);
    //bg!(&label, &number);
    let mut mnemonics = vec![];
    let partial_mnemonic = Mnemonic {
        phrase: "".to_string(),
        word_count: 0,
        score: NotNan::new(0.0).unwrap(),
    };
    fill_mnemonics(word_list, &mut mnemonics, &partial_mnemonic, &number, max_words);
    mnemonics.sort_by_key(|m| (m.word_count, m.score));
    dbg!(&mnemonics.iter().map(|m| m.phrase.clone()).collect::<Vec<_>>());
}

fn fill_mnemonics(word_list: &WordList, mnemonics: &mut Vec<Mnemonic>, partial_mnemonic: &Mnemonic, number: &str, max_words: usize) {
    //bg!(&partial_phrase, number);
    if partial_mnemonic.word_count < max_words {
        for length in (1..=number.len()).rev() {
            let (partial_number, remaining_number) = number.split_at(length);
            //bg!(length, partial_number, remaining_number);
            for word in word_list.words
                .values()
                .filter(|word| word.mnemonic.as_ref().map_or(false, |mnemonic| mnemonic.eq(partial_number))) {
                //bg!(&word.word);
                let mut new_mnemonic = partial_mnemonic.clone();
                new_mnemonic.phrase = format!("{} {}", partial_mnemonic.phrase, word.word).trim().to_string();
                new_mnemonic.word_count += 1;
                new_mnemonic.score = new_mnemonic.score + NotNan::new(word.frequency as f64).unwrap();
                if length == number.len() {
                    mnemonics.push(new_mnemonic);
                } else {
                    fill_mnemonics(word_list, mnemonics, &new_mnemonic, remaining_number, max_words);
                }
            }
        }
    }
}

fn digits_only(value: &str) -> String {
    value.chars().filter(|char| char.is_digit(10)).collect()
}