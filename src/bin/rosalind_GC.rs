#![feature(path)]
extern crate bio;

use std::path::Path;
use bio::gc;


fn main() {
    let mut max_gc: f32 = 0f32;
    let mut max_gc_name: String = String::new();

    let path = Path::new("test_files/rosalind_gc.txt");

    for (seq, id) in bio::fasta::FASTAReader::new(&path) {
        if gc(&seq) > max_gc {
            max_gc = gc(&seq);
            max_gc_name = id.unwrap();
        };
    }

    println!("{}", max_gc_name);
    println!("{}", 100f32 * max_gc);
}
