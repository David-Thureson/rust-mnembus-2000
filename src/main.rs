// use mnembus_2000_rust::*;
use mnembus_2000_rust::{mnemonic, words};
use util_rust::log;

fn main() {
    println!("Mnembus 2000 - Start");
    log::clear();

    // words::survey_words();
    // let words = words::WordList::fill();
    // words::survey_pronunciations();
    //bg!(words::Pronunciation::fill(Some(words)).iter().take(20).collect::<Vec<_>>());
    // try_read_pronunciations();
    try_propose_mnemonics();

    dbg!(log::get());
    println!("Mnembus 2000 - Done");
}

fn try_propose_mnemonics() {
    let mut words = words::WordList::fill();
    words::Pronunciation::fill(Some(&mut words));
    //mnemonic::propose_mnemonics(&words, "Test", "70718", 3, 1_000);
    //mnemonic::propose_mnemonics(&words, "Brian", "206-890-9233");
    //mnemonic::MnemonicRun::new(&words, "Test", "70718", 3, 2_000);
    //nemonic::MnemonicRun::new(&words, "Test", "890-9233", 4, 5_000);
    //mnemonic::MnemonicRun::new(&words, "Brian", "206-890-9233", 5, 5_000);
    mnemonic::main();
}

