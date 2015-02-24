#![feature(old_io)]
extern crate bio;

use std::old_io::{stdin};

use bio::rna_to_prot;

 
fn main() {
    let input: String = stdin().read_line().unwrap();
    println!("{}", rna_to_prot(input.as_slice().trim()));
}
