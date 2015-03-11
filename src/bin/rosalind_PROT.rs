#![feature(io)]
extern crate bio;

use std::io::BufReadExt;
use std::io;

use bio::rna_to_prot;

 
fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", rna_to_prot(input.trim()));
}
