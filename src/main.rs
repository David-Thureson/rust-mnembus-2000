// use mnembus_2000_rust::*;
use mnembus_2000_rust::words;
use util_rust::log;

fn main() {
    println!("Mnembus 2000 - Start");
    log::clear();

    // words::survey_words();
    let words = words::WordList::fill();
    // words::survey_pronunciations();
    dbg!(words::Pronunciation::fill(Some(words)).iter().take(20).collect::<Vec<_>>());
    // try_read_pronunciations();

    dbg!(log::get());
    println!("Mnembus 2000 - Done");
}

