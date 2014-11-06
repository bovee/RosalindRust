use std::collections::TreeMap;
use std::str;
use std::io;
 

fn main() {
    let input: String = io::stdin().read_line().unwrap();
    println!("{}", rna_to_prot(input.as_slice().trim()));
}

#[test]
fn test_rna_to_prot() {
    assert!(rna_to_prot("AGAUUU").as_slice() == "RF");
    assert!(rna_to_prot("AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGG").as_slice() == "MAMAPRTEINSTRING");
}

fn rna_to_prot(rna_seq: &str) -> String {
    let mut codon_table = TreeMap::new();
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
        let aa = match codon_table.find(&c) {
            Some(&aa) => aa, 
            None => "?"
        };
        s.push_str(aa);
    };
    return s;
}
