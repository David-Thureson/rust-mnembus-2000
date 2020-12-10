use crate::words::WordList;
use ordered_float::NotNan;
use std::time::Instant;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct Mnemonic {
    phrase: String,
    word_count: usize,
    score: NotNan<f64>,
}

#[allow(dead_code)]
pub struct MnemonicRun {
    label: String,
    number: String,
    max_words: usize,
    max_rank: usize,
    // words: BTreeMap<String, Vec<(String, usize)>>,
    // start_time: Instant,
    // end_time: Option<Instant>,
    mnemonics: Vec<Mnemonic>,
}

impl MnemonicRun {
    pub fn new(word_list: &WordList, label: &str, number: &str, max_words: usize, max_rank: usize) -> Self {
        let start_time_overall = Instant::now();
        let number = digits_only(number);
        // One entry per mnemonic with multiple words possible per entry.
        let start_time_build_btree = Instant::now();
        let mut words = BTreeMap::new();
        for word in word_list.words.values().filter(|word| word.mnemonic.as_ref().is_some() && word.rank <= max_rank) {
            let entry = words.entry(word.mnemonic.as_ref().unwrap().clone()).or_insert(vec![]);
            entry.push((word.word.clone(), word.rank));
        }
        let elapsed_build_btree = Instant::now() - start_time_build_btree;
        let mut run = Self {
            label: label.to_string(),
            number: number.to_string(),
            max_words,
            max_rank,
            //words,
            //start_time,
            //end_time: None,
            mnemonics: vec![]
        };
        let partial_mnemonic = Mnemonic {
            phrase: "".to_string(),
            word_count: 0,
            score: NotNan::new(0.0).unwrap(),
        };
        let start_time_propose = Instant::now();
        let mut mnemonics = vec![];
        run.propose_mnemonics(&words, &mut mnemonics, &partial_mnemonic, &number);
        run.mnemonics = mnemonics;
        let elapsed_propose = Instant::now() - start_time_propose;
        let elapsed_overall = Instant::now() - start_time_overall;
        dbg!(run.mnemonics.iter().map(|x| x.phrase.clone()).collect::<Vec<_>>());
        dbg!(elapsed_build_btree, elapsed_propose, elapsed_overall);
        run
    }

    fn propose_mnemonics(&self, words: &BTreeMap<String, Vec<(String, usize)>>, mnemonics: &mut Vec<Mnemonic>, partial_mnemonic: &Mnemonic, remaining_number: &str) {
        //bg!(&partial_phrase, number);
        if partial_mnemonic.word_count < self.max_words {
            for length in (1..=remaining_number.len()).rev() {
                let (match_number, new_remaining_number) = remaining_number.split_at(length);
                //bg!(length, partial_number, remaining_number);
                if let Some(matching_words) = words.get(match_number) {
                    for (word, rank) in matching_words {
                        let mut new_mnemonic = partial_mnemonic.clone();
                        new_mnemonic.phrase = format!("{} {}", partial_mnemonic.phrase, word).trim().to_string();
                        new_mnemonic.word_count += 1;
                        new_mnemonic.score = new_mnemonic.score + NotNan::new(*rank as f64).unwrap();
                        if length == remaining_number.len() {
                            mnemonics.push(new_mnemonic);
                        } else {
                            self.propose_mnemonics(words, mnemonics, &new_mnemonic, new_remaining_number);
                        }
                    }
                }
            }
        }
    }
}

/*
pub fn propose_mnemonics (word_list: &WordList, label: &str, number: &str, max_words: usize, max_rank: usize) {
    let start_time = Instant::now();
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
    dbg!(Instant::now() - start_time);
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
*/

fn digits_only(value: &str) -> String {
    value.chars().filter(|char| char.is_digit(10)).collect()
}

/*
    fn propose_mnemonics(&mut self, partial_mnemonic: &Mnemonic, remaining_number: &str) {
        //bg!(&partial_phrase, number);
        if partial_mnemonic.word_count < self.max_words {
            for length in (1..=remaining_number.len()).rev() {
                let (match_number, new_remaining_number) = remaining_number.split_at(length);
                //bg!(length, partial_number, remaining_number);
                if let Some(matching_words) = self.words.get(match_number) {
                    for (word, rank) in matching_words {
                        let mut new_mnemonic = partial_mnemonic.clone();
                        new_mnemonic.phrase = format!("{} {}", partial_mnemonic.phrase, word).trim().to_string();
                        new_mnemonic.word_count += 1;
                        new_mnemonic.score = new_mnemonic.score + NotNan::new(*rank as f64).unwrap();
                        if length == remaining_number.len() {
                            self.mnemonics.push(new_mnemonic);
                        } else {
                            self.propose_mnemonics(&new_mnemonic, new_remaining_number);
                        }
                    }
                }
            }
        }
    }

 */