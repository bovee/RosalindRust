#![feature(io)]
use std::io::ReadExt;
use std::ascii::AsciiExt;

fn main() {
    let stdin = std::io::stdin();
    for chr in stdin.chars() {
        let chr = chr.ok().unwrap().to_ascii_uppercase();
        let rna_chr = match chr {
            'T' => 'U',
            _ => chr
        };
        print!("{}", rna_chr);
    };
}
