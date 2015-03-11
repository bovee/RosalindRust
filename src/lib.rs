#![feature(io)]
#![feature(path)]

#![crate_name = "bio"]

#[allow(unused_attributes)]

use std::collections::HashMap;
use std::str;
use std::ascii::AsciiExt;

pub mod fasta;

#[test]
fn test_gc() {
    assert_eq!(gc("AAGC"), 0.5);
}

pub fn gc(dna: &str) -> f32 {
    let mut gc_count: i32 = 0;
    let mut num_chars: i32 = 0;
    for chr in dna.trim().chars() {
        gc_count += match chr {
            'G' | 'C' | 'g' | 'c' => 1,
            _ => 0
        };
        num_chars += 1;
    };
    return (gc_count as f32) / (num_chars as f32);
}

#[test]
fn test_hamming_dist() {
    assert_eq!(hamming_dist("AAGC", "AAGT"), 1);
}

pub fn hamming_dist(str1: &str, str2: &str) -> i32 {
    let mut dif_chars: i32 = 0;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            dif_chars += 1;
        }
    }
    return dif_chars;
}


#[test]
fn test_rna_to_prot() {
    assert!(rna_to_prot("AGAUUU").as_slice() == "RF");
    assert!(rna_to_prot("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGG").as_slice() == "MAMAPRTEINSTRING");
}

pub fn rna_to_prot(rna_seq: &str) -> String {
    let mut codon_table = HashMap::new();
    // TODO: load this from an external file
    codon_table.insert("UUU", "F");
    codon_table.insert("CUU", "L");
    codon_table.insert("AUU", "I");
    codon_table.insert("GUU", "V");
    codon_table.insert("UUC", "F");
    codon_table.insert("CUC", "L");
    codon_table.insert("AUC", "I");
    codon_table.insert("GUC", "V");
    codon_table.insert("UUA", "L");
    codon_table.insert("CUA", "L");
    codon_table.insert("AUA", "I");
    codon_table.insert("GUA", "V");
    codon_table.insert("UUG", "L");
    codon_table.insert("CUG", "L");
    codon_table.insert("AUG", "M");
    codon_table.insert("GUG", "V");
    codon_table.insert("UCU", "S");
    codon_table.insert("CCU", "P");
    codon_table.insert("ACU", "T");
    codon_table.insert("GCU", "A");
    codon_table.insert("UCC", "S");
    codon_table.insert("CCC", "P");
    codon_table.insert("ACC", "T");
    codon_table.insert("GCC", "A");
    codon_table.insert("UCA", "S");
    codon_table.insert("CCA", "P");
    codon_table.insert("ACA", "T");
    codon_table.insert("GCA", "A");
    codon_table.insert("UCG", "S");
    codon_table.insert("CCG", "P");
    codon_table.insert("ACG", "T");
    codon_table.insert("GCG", "A");
    codon_table.insert("UAU", "Y");
    codon_table.insert("CAU", "H");
    codon_table.insert("AAU", "N");
    codon_table.insert("GAU", "D");
    codon_table.insert("UAC", "Y");
    codon_table.insert("CAC", "H");
    codon_table.insert("AAC", "N");
    codon_table.insert("GAC", "D");
    codon_table.insert("UAA", "*");
    codon_table.insert("CAA", "Q");
    codon_table.insert("AAA", "K");
    codon_table.insert("GAA", "E");
    codon_table.insert("UAG", "*");
    codon_table.insert("CAG", "Q");
    codon_table.insert("AAG", "K");
    codon_table.insert("GAG", "E");
    codon_table.insert("UGU", "C");
    codon_table.insert("CGU", "R");
    codon_table.insert("AGU", "S");
    codon_table.insert("GGU", "G");
    codon_table.insert("UGC", "C");
    codon_table.insert("CGC", "R");
    codon_table.insert("AGC", "S");
    codon_table.insert("GGC", "G");
    codon_table.insert("UGA", "*");
    codon_table.insert("CGA", "R");
    codon_table.insert("AGA", "R");
    codon_table.insert("GGA", "G");
    codon_table.insert("UGG", "W");
    codon_table.insert("CGG", "R");
    codon_table.insert("AGG", "R");
    codon_table.insert("GGG", "G");
    
    let mut s = String::new();
    // TODO: this should be iterating over unicode points instead?
    let codons = rna_seq.as_bytes();

    for codon in codons.chunks(3) {
        let c = str::from_utf8(codon).unwrap();
        let aa = match codon_table.get(&c) {
            Some(&aa) => aa, 
            None => "?"
        };
        s.push_str(aa);
    };
    return s;
}

#[test]
fn test_revc() {

}

pub fn revc(dna: &str) {
    for chr in dna.chars().rev() {
        let rev_comp = match chr.to_ascii_uppercase() {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => chr
        };
        print!("{}", rev_comp);
    };
}


#[derive(Debug)]
pub struct MovingWindow<'a> {
    wrapped_str: &'a str,
    loc: usize,
    size: usize,
}

impl<'a> MovingWindow<'a> {
    pub fn new(wrapped_str: &'a str, size: usize) -> MovingWindow<'a> {
        MovingWindow {
            wrapped_str: wrapped_str,
            loc: 0,
            size: size,
        }
    }
}

impl<'a> Iterator for MovingWindow<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.loc += 1;
        if self.loc + self.size - 1 <= self.wrapped_str.len() {
            Some(&self.wrapped_str[(self.loc - 1)..(self.loc + self.size - 1)])
        } else {
            None
        }
    }
}
