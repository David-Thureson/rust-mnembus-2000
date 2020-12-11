use crate::*;
use util_rust::group::{Grouper, count_distinct, list_duplicates};
use util_rust::log;
use std::collections::BTreeMap;

const WORD_FILE_NAME: &str = "English Words Top 5000.txt";
const PRONUNCIATION_FILE_NAME: &str = "CMU Pronouncing Dictionary.txt";

#[derive(Debug)]
pub struct WordList {
    pub words: BTreeMap<String, Word>,
}

#[derive(Debug)]
pub struct Word {
    pub word: String,
    pub rank: usize,
    pub frequency: usize,
    pub dispersion: f64,
    pub part_of_speech: String,
    pub mnemonic: Option<String>,
}

#[derive(Debug)]
pub struct Pronunciation {
    word: String,
    mnemonic: String,
    phones: Vec<String>,
}

impl WordList {
    pub fn new() -> Self {
        Self {
            words: BTreeMap::new(),
        }
    }

    pub fn fill() -> Self {
        let mut words = BTreeMap::new();
        let lines = util_rust::parse::read_file_as_lines(WORD_FILE_NAME)
            .iter()
            .skip(1)
            .map(|line| line.trim().to_string())
            .collect::<Vec<_>>();
        for line in lines.iter() {
            let splits = line.split("\t").collect::<Vec<_>>();
            let rank: usize = splits[0].trim().parse().unwrap();
            let word = splits[1].trim().to_string();
            let part_of_speech = splits[2].trim().to_string();
            let frequency: usize = splits[3].trim().parse().unwrap();
            let dispersion: f64 = splits[4].trim().parse().unwrap();
            words.insert(word.to_lowercase().to_string(), Word {
                word,
                rank,
                frequency,
                dispersion,
                part_of_speech,
                mnemonic: None,
            });
        }
        Self {
            words,
        }
    }

    pub fn fill_with_pronunciation() -> Self {
        let mut words = Self::fill();
        Pronunciation::fill(Some(&mut words));
        words
    }

    pub fn contains_word(&self, word: &str) -> bool {
        self.words.contains_key(&word.to_lowercase())
    }

    pub fn set_mnemonic(&mut self, word: &str, mnemonic: &str) {
        let mnemonic = Some(mnemonic.to_string());
        let word = self.words.get_mut(&word.to_lowercase());
        if let Some(word) = word {
            word.mnemonic = mnemonic;
        }
    }
}

impl Word {
}

impl Pronunciation {
    pub fn fill(mut words: Option<&mut WordList>) -> Vec<Self> {
        //bg!(&words);
        let mut v = vec![];
        let lines = util_rust::parse::read_file_as_lines(PRONUNCIATION_FILE_NAME)
            .iter()
            .map(|line| line.trim().to_string())
            .collect::<Vec<_>>();
        //ebug_assert!(lines.len() >= 1000);
        // Ignore the alternate pronunciations with a parenthetical number like:
        //   DROP     D R AA1 P
        //   DROP(1)  D R AO1 P
        // These variations seem to be in vowels and weak sounds like "H", so they won't affect the
        // mnemonics.
        for line in lines.iter().filter(|line| !line.contains("(")) {
            let mut splits = line
                .split(" ")
                .filter(|&x| !x.is_empty())
                .map(|split| split.trim().to_string())
                .collect::<Vec<_>>();
            debug_assert!(splits.len() > 1);
            let word = splits.remove(0);
            let use_this_word = words.as_ref().map_or(true, |words| words.contains_word(&word));
            //bg!(&word, use_this_word);
            if use_this_word {
                let phones = splits;
                match Self::phones_to_mnemonic(&phones) {
                    Ok(mnemonic) => {
                        if mnemonic.len() > 0 {
                            words.as_mut().and_then(|words| Some(words.set_mnemonic(&word, &mnemonic)));
                            // if let Some(ref mut w) = words {
                            //     w.set_mnemonic(&word, &mnemonic);
                            //}
                            v.push(Self {
                                word,
                                mnemonic,
                                phones,
                            });
                        }
                    },
                    Err(message) => {
                        dbg!(&message);
                        log::log(&format!("{} in {}", message, line));
                    },
                };
            }
        }
        //bg!(v.len());
        v
    }

    fn phones_to_mnemonic(phones: &[String]) -> Result<String, String> {
        debug_assert!(phones.len() > 0);
        /*
        Ok(phones
            .iter()
            .map(|phone| phone_to_mnemonic_number(phone).ok()?)
            .map(|opt| opt.map_or("".to_string(), |x| x.to_string()))
            .collect())
         */
        let mut mnemonic = "".to_string();
        for phone in phones.iter() {
            match phone_to_mnemonic_number(phone) {
                Ok(opt) => {
                    if let Some(n) = opt {
                        mnemonic.push_str(&n.to_string());
                    }
                },
                Err(message) => {
                    return Err(message);
                }
            }
        }
        Ok(mnemonic)
    }
}

pub fn survey_words() {
    let mut ranks = vec![];
    let mut words = vec![];
    let mut part_of_speech_grouper = Grouper::new("Part of Speech");
    let mut frequencies = vec![];
    let mut dispersion_min = f64::MAX;
    let mut dispersion_max = f64::MIN;
    let lines = util_rust::parse::read_file_as_lines(WORD_FILE_NAME)
            .iter()
            .skip(1)
            .map(|line| line.trim().to_string())
            .collect::<Vec<_>>();
        //bg!(&line);
    for line in lines.iter() {
        let splits = line.split("\t").collect::<Vec<_>>();
        //bg!(&splits);
        let rank: usize = splits[0].trim().parse().unwrap();
        dbg!(rank);
        ranks.push(rank);
        let word = splits[1].trim();
        dbg!(word);
        words.push(word);
        let part_of_speech = splits[2].trim();
        dbg!(part_of_speech);
        part_of_speech_grouper.record_entry(&part_of_speech);
        let frequency: usize = splits[3].trim().parse().unwrap();
        dbg!(frequency);
        frequencies.push(frequency);
        let dispersion: f64 = splits[4].trim().parse().unwrap();
        dbg!(dispersion);
        dispersion_min = dispersion_min.min(dispersion);
        dispersion_max = dispersion_max.min(dispersion);
    }
    dbg!(ranks.len(), count_distinct(&ranks), ranks.iter().min(), ranks.iter().max());
    dbg!(words.len(), count_distinct(&words), words.iter().min(), words.iter().max());
    part_of_speech_grouper.print_by_count(0, None);
    dbg!(frequencies.len(), count_distinct(&frequencies), frequencies.iter().min(), frequencies.iter().max());
    dbg!(dispersion_min, dispersion_max);
}

pub fn survey_pronunciations() {
    let mut words = vec![];
    let mut exception_words = vec![];
    let mut phone_count_grouper = Grouper::new("Phone Counts");
    let mut phone_grouper = Grouper::new("Phones");
    let lines = util_rust::parse::read_file_as_lines(PRONUNCIATION_FILE_NAME)
        .iter()
        .map(|line| line.trim().to_string())
        .collect::<Vec<_>>();
    //bg!(&line);
    // Ignore the alternate pronunciations with a parenthetical number like:
    //   DROP     D R AA1 P
    //   DROP(1)  D R AO1 P
    // These variations seem to be in vowels and weak sounds like "H", so they won't affect the
    // mnemonics.
    for line in lines.iter().filter(|line| !line.contains("(")) {
        //bg!(&line);
        let splits = line.split(" ").filter(|&x| !x.is_empty()).collect::<Vec<_>>();
        //bg!(&splits);
        let word = splits[0].trim();
        //bg!(word);
        let first_char: char = word.chars().next().unwrap();
        if !(first_char.is_ascii_uppercase() || first_char.is_ascii_digit()) {
            exception_words.push(word.clone());
        }
        words.push(word);
        let phone_count = splits.len() - 1;
        // if phone_count >= 18 {
        //     dbg!(&line);
        // }
        phone_count_grouper.record_entry(&phone_count);
        for phone in splits.iter().skip(1).map(|phone| phone.to_string()) {
            phone_grouper.record_entry(&phone);
        }
    }
    dbg!(words.len(), count_distinct(&words), words.iter().min(), words.iter().max());
    phone_count_grouper.list_by_key();
    phone_grouper.print_by_count(0, None);
    dbg!(&exception_words);
    dbg!(list_duplicates(&words));
}

fn phone_to_mnemonic_number(phone: &str) -> Result<Option<u8>, String> {
    //bg!(&phone);
    let phone = if phone.len() == 3 { &phone[..2] } else { phone };
    match phone {
        "AA" => Ok(None),
        "AE" => Ok(None),
        "AH" => Ok(None),
        "AO" => Ok(None),
        "AW" => Ok(None),
        "AY" => Ok(None),
        "B" => Ok(Some(9)),
        "CH" => Ok(Some(6)),
        "D" => Ok(Some(1)),
        "DH" => Ok(None),
        "EH" => Ok(None),
        "ER" => Ok(Some(4)),
        "EY" => Ok(None),
        "F" => Ok(Some(8)),
        "G" => Ok(Some(7)),
        "HH" => Ok(None),
        "IH" => Ok(None),
        "IY" => Ok(None),
        "JH" => Ok(Some(6)),
        "K" => Ok(Some(7)),
        "L" => Ok(Some(5)),
        "M" => Ok(Some(3)),
        "N" => Ok(Some(2)),
        "NG" => Ok(Some(2)),
        "OW" => Ok(None),
        "OY" => Ok(None),
        "P" => Ok(Some(9)),
        "R" => Ok(Some(4)),
        "S" => Ok(Some(0)),
        "SH" => Ok(Some(6)),
        "T" => Ok(Some(1)),
        "TH" => Ok(Some(1)),
        "UH" => Ok(None),
        "UW" => Ok(None),
        "V" => Ok(Some(8)),
        "W" => Ok(None),
        "Y" => Ok(None),
        "Z" => Ok(Some(0)),
        "ZH" => Ok(Some(6)),
        _ => Err(format!("Unexpected phone = {}", phone)),
    }
}
