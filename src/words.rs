use crate::*;
use util_rust::group::{Grouper, count_distinct, list_duplicates};

const WORD_FILE_NAME: &str = "English Words Top 5000.txt";
const PRONUNCIATION_FILE_NAME: &str = "CMU Pronouncing Dictionary.txt";

#[derive(Debug)]
pub struct Word {
    word: String,
    rank: usize,
    frequency: usize,
    dispersion: f64,
    part_of_speech: String,
}

impl Word {
    pub fn fill_words() -> Vec<Self> {
        let mut v = vec![];
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
            v.push(Self {
                word,
                rank,
                frequency,
                dispersion,
                part_of_speech,
            })
        }
        v
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

