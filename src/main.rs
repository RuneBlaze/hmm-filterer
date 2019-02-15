#![warn(clippy)]
#[macro_use]
extern crate lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;
use std::env;
use std::collections::HashSet;

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

fn name_match(text : &str) -> Option<String> {
    lazy_static! {
        static ref RE : Regex = Regex::new("NAME  ([[:alnum:]_-]+)").unwrap();
    }

    if let Some(cap) = RE.captures(text) {
        // println!("{:?}", cap[1].to_string());
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
    species : Option<String>,
    name : Option<String>,
}

impl HmmEntry {
    fn new() -> HmmEntry {
        HmmEntry {raw: String::new(), species: None, name: None}
    }

    fn append(&mut self, line : &str) {
        if let Some(species) = species_match(line) {
            self.species = Some(species);
        }
        if let Some(name) = name_match(line) {
            self.name = Some(name);
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

    let mut models = HashSet::new();
    let model_names = vec![
        String::from("ERVB7_1-LTR_MM"),
        String::from("ERVB4_2-LTR_MM"),
        String::from("RLTRETN_Mm"),
        String::from("IAPLTR1_Mm"),
        String::from("MT2_Mm"),
        String::from("IAPEY3C_LTR"),
        String::from("L1MdA_I_5end"),
        String::from("L1MdTf_I_5end"),
    ];

    for n in model_names.iter() {
        models.insert(n);
    }
    // models.insert(value: T)
    // let mut hmms : Vec<HmmEntry> = vec![];
    let mut cur_hmm = HmmEntry::new();
    // let mut cnt : u32 = 0;
    for maybe_line in BufReader::new(file).lines() {
        let line = maybe_line.unwrap();
        cur_hmm.append(&line);
        if is_line_end(&line) {
            // if cur_hmm.is_fit() {
                if models.contains(&cur_hmm.name.unwrap()) {
                    println!("{}", cur_hmm.raw);
                }
                
                // cnt += 1;
                // if cnt >= 500 {
                //     break;
                // }
            // }
            cur_hmm = HmmEntry::new();
        }
    }
    // println!("cnt: {}", cnt);
}