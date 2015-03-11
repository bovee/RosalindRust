#![feature(old_io)]
extern crate bio;

use std::old_io::{stdin};
use bio::hamming_dist;
 

fn main() {
    let mut stdin = stdin();
    let line1 = stdin.read_line().unwrap();
    let line2 = stdin.read_line().unwrap();
    println!("{}", hamming_dist(&line1, &line2));
    //println!("{}", l1.lev_distance(l2));
}
