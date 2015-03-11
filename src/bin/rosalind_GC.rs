#![feature(io)]
#![feature(path)]
extern crate bio;

use bio::gc;
use bio::fasta::FASTAReader;


fn main() {
    let mut max_gc: f32 = 0f32;
    let mut max_gc_name: String = String::new();

    let stdin = std::io::stdin();
    for (seq, id) in FASTAReader::new(stdin.lock()) {
        if gc(&seq) > max_gc {
            max_gc = gc(&seq);
            max_gc_name = id.unwrap();
        };
    }

    println!("{}", max_gc_name);
    println!("{}", 100f32 * max_gc);
}
