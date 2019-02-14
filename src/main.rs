#[macro_use]
extern crate lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;
use std::env;

const SPECIES : &str = "Mus musculus";

fn species_match(text : &str) -> Option<String> {
    lazy_static! {
        static ref RE : Regex = Regex::new("TaxName:([[:alpha:] ]+);").unwrap();
    }
    if let Some(cap) = RE.captures(text) {
        Some(cap[1].to_string())
    } else {
        None
    }
}

fn is_line_end(text : &str) -> bool {
    text.starts_with("//")
}

// essentially tagged strings
struct HmmEntry {
    raw : String,
    species : Option<String>
}

impl HmmEntry {
    fn new() -> HmmEntry {
        HmmEntry {raw: String::new(), species: None}
    }

    fn append(&mut self, line : &str) {
        if let Some(species) = species_match(line) {
            self.species = Some(species);
        }
        if !self.is_fit() {
            return;
        }
        self.raw.push_str(line);
        self.raw.push('\n');
    }

    fn is_fit(&self) -> bool {
        if let Some(s) = &self.species {
            s == SPECIES
        } else {
            true
        }
    }
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path).unwrap();
    // let mut hmms : Vec<HmmEntry> = vec![];
    let mut cur_hmm = HmmEntry::new();
    for maybe_line in BufReader::new(file).lines() {
        let line = maybe_line.unwrap();
        cur_hmm.append(&line);
        if is_line_end(&line) {
            if cur_hmm.is_fit() {
                println!("{}", cur_hmm.raw);
            }
            cur_hmm = HmmEntry::new();
        }
    }
}