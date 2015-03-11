#![feature(io)]
extern crate bio;

use std::io::BufReadExt;
use bio::hamming_dist;
 

fn main() {
    let stdin = std::io::stdin();
    let line1 = stdin.lock().lines().next().unwrap().unwrap();
    let line2 = stdin.lock().lines().next().unwrap().unwrap();
    println!("{}", hamming_dist(&line1, &line2));
    //println!("{}", l1.lev_distance(l2));
}
