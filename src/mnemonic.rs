use ordered_float::NotNan;
use std::time::Instant;
use std::collections::BTreeMap;
use crate::words::WordList;
use crate::itertools::Itertools;
use util_rust::parse;

pub fn main() {
    let words = WordList::fill_with_pronunciation();
    //bg!(gen_paths("123456",4));
    // propose_mnemonics_path(&words, "Executive", "70718", 5_000);
    // propose_mnemonics_path(&words, "Executive Plus", "3707184", 5_000);
    // propose_mnemonics_path(&words, "Brian", "206-890-9233", 5_000);
    propose_mnemonics_path(&words, "Mom (cell)", "206-406-9701", 5_000);
}

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
        let number = parse::digits_only(number);
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

pub fn propose_mnemonics_path(word_list: &WordList, _label: &str, match_numbers: &str, max_rank: usize) {
    let start_time_overall = Instant::now();
    let match_numbers = parse::digits_only(match_numbers);

    // One entry per mnemonic with multiple words possible per entry.
    let start_time_build_btree = Instant::now();
    let mut words = BTreeMap::new();
    for word in word_list.words.values().filter(|word| word.mnemonic.as_ref().is_some() && word.rank <= max_rank) {
        let entry = words.entry(word.mnemonic.as_ref().unwrap().clone()).or_insert(vec![]);
        entry.push((word.word.clone(), word.rank));
    }
    let elapsed_build_btree = Instant::now() - start_time_build_btree;

    let start_time_propose = Instant::now();

    // Try to find mnemonics with the least possible number of words.
    // The most possible words is the length of the match number, with a single phone per word.
    let mut found = false;
    for path_length in 1..=match_numbers.len() {
        let paths = gen_paths(&match_numbers, path_length);
        //bg!(path_length, &paths);
        for path in paths.iter() {
            // See if we have at least one matching word for each step in the path.
            if path.iter().all(|key| words.contains_key(key)) {
                found = true;
                println!("{}", path.iter().join("-"));
                for key in path.iter() {
                    let found_words = words.get(key).unwrap().iter().map(|(word, _)| word).join(" ");
                    println!("\t{}", found_words);
                }
            }
        }
        if found {
            break;
        }
    }

    let elapsed_propose = Instant::now() - start_time_propose;
    let elapsed_overall = Instant::now() - start_time_overall;
    dbg!(elapsed_build_btree, elapsed_propose, elapsed_overall);
}

fn gen_paths(match_numbers: &str, path_length: usize) -> Vec<Vec<String>> {
    let mut paths = vec![];
    let partial_path = vec![];
    gen_path_internal(&mut paths, partial_path, match_numbers, path_length);
    //bg!(&paths);

    // Test the paths.
    for path in paths.iter(){
        let reconstructed_numbers = path.iter().join("");
        //rintln!("{}", reconstructed_numbers);
        debug_assert_eq!(reconstructed_numbers, match_numbers);
    }
    paths
}

fn gen_path_internal(paths: &mut Vec<Vec<String>>, partial_path: Vec<String>, remaining_match_numbers: &str, remaining_path_length: usize) {
    //bg!(&paths, &partial_path, remaining_match_numbers, remaining_path_length);
    if remaining_path_length == 1 {
        // We have only one path step remaining so we must use all of the remainingc match numbers.
        // Shadow partial_path as completed_path to make the dbg! output clearer.
        let mut completed_path = partial_path;
        completed_path.push(remaining_match_numbers.to_string());
        //bg!(&completed_path);
        paths.push(completed_path);
    } else {
        let remaining_match_numbers_len = remaining_match_numbers.len();
        let length_range = 1..=(remaining_match_numbers_len - remaining_path_length) + 1;
        //bg!(remaining_match_numbers_len, &length_range);
        for length in length_range.rev() {
            let (these_match_numbers, new_remaining_match_numbers) = remaining_match_numbers.split_at(length);
            //bg!(length, these_match_numbers, new_remaining_match_numbers);
            let mut new_path = partial_path.clone();
            new_path.push(these_match_numbers.to_string());
            gen_path_internal(paths, new_path, &new_remaining_match_numbers, remaining_path_length - 1);
        }
    }
}

/*
pub fn propose_mnemonics (word_list: &WordList, label: &str, number: &str, max_words: usize, max_rank: usize) {
    let start_time = Instant::now();
    let number = parse::digits_only(number);
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