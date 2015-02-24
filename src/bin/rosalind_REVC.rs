#![feature(old_io)]
extern crate bio;

use std::old_io::{stdin};
use bio::revc;
 

fn main() {
    let mut stdin = stdin();
    let data: String = stdin.lock().chars().map(|x| x.unwrap()).collect();
    revc(&data);
}
