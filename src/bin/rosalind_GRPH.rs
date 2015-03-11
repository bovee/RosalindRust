#![feature(path)]
extern crate bio;

use std::path::Path;
use std::collections::HashMap;

fn main() {
    let mut endings: HashMap<String, Vec<String>> = HashMap::new();
    let path = Path::new("rosalind_grph.txt");
    for (seq, id) in bio::fasta::FASTAReader::new(&path) {
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
    for (seq, id) in bio::fasta::FASTAReader::new(&path) {
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

