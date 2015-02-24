#![feature(old_io)]
#![feature(unicode)]
use std::old_io::{stdin};

fn main() {
    let mut stdin = stdin();
    for chr in stdin.lock().chars() {
        let chr = chr.ok().unwrap().to_uppercase();
        let rna_chr = match chr {
            'T' => 'U',
            _ => chr
        };
        print!("{}", rna_chr);
    };
}
