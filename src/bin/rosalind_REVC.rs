extern crate bio;

use std::io;
use bio::revc;
 

fn main() {
    let data: String = io::stdin().chars().map(|x| x.unwrap()).collect();
    revc(data.as_slice());
}
