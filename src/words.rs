use crate::*;

const WORD_FILE_NAME: &str = "English Words Top 5000.txt";

pub struct Word {
    word: String,
}

pub fn survey_words() {
    let mut ranks = vec![];
    let mut words = vec![];
    let mut part_of_speech_grouper = util_rust::group::Grouper::new("Part of Speech");
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
    dbg!(ranks.len(), ranks.iter().min(), ranks.iter().max());
    ranks.dedup();
    dbg!(ranks.len());
    dbg!(words.len(), words.iter().min(), words.iter().max());
    words.dedup();
    dbg!(words.len());
    part_of_speech_grouper.print_by_count(0, None);
    dbg!(frequencies.len(), frequencies.iter().min(), frequencies.iter().max());
    frequencies.dedup();
    dbg!(frequencies.len());
    dbg!(dispersion_min, dispersion_max);
}

pub fn try_read_words() {

}

