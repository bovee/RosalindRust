#![feature(path)]
extern crate bio;

use std::path::Path;
use std::collections::HashMap;
use bio::fasta::FASTAReader;
use std::fs::File;

fn main() {
    let mut endings: HashMap<String, Vec<String>> = HashMap::new();
    let path = Path::new("test_files/rosalind_grph.txt");
    for (seq, id) in FASTAReader::<File>::from_path(&path) {
        let id = id.unwrap();
        let se = seq.len() - 3;
        if endings.contains_key(&seq[se..]) {
            let mut values = endings.get_mut(&seq[se..]).unwrap();
            values.push(id);
            //endings.insert(seq[-3..].to_string(), values);
        } else {
            endings.insert(seq[se..].to_string(), vec![id]);
        }
    }
    for (seq, id) in FASTAReader::<File>::from_path(&path) {
        let id = id.unwrap();
        match endings.get(&seq[..3]) {
            Some(first_ids) => {
                for first_id in first_ids.iter() {
                    if first_id != &id {
                        println!("{} {}", first_id, id);
                    }
                }
            },
            None => {}
        }
    }
}

