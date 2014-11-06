extern crate bio;
use bio::rna_to_prot;

use std::io;
 
fn main() {
    let input: String = io::stdin().read_line().unwrap();
    println!("{}", rna_to_prot(input.as_slice().trim()));
}
