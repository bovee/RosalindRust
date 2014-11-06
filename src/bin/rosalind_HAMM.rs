extern crate bio;

use std::io;
use bio::hamming_dist;
 

fn main() {
    let mut stdin = io::stdin();
    let line1 = stdin.read_line().unwrap();
    let line2 = stdin.read_line().unwrap();
    let l1 = line1.as_slice();
    let l2 = line2.as_slice();
    println!("{}", hamming_dist(l1, l2));
    //println!("{}", l1.lev_distance(l2));
}
