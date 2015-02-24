#![feature(old_io)]
use std::old_io::{stdin};
 

fn main() {
    let input: String = stdin().read_line().unwrap();
    let nums: Vec<&str> = input.trim().split(' ').collect();
    let k: f32 = nums[0].parse().unwrap();
    let m: f32 = nums[1].parse().unwrap();
    let n: f32 = nums[2].parse().unwrap();
    let inv_odds = ((n * (n - 1.)) + 0.25 * (m * (m - 1.)) + m * n) / ((k + m + n) * (k + m + n - 1.));
    let odds = 1. - inv_odds;
    println!("{}", odds);
}

//      HH   Hh   hh
// HH   0    0    0
// Hh   0    0.25 0.5
// hh   0    0.5  1
//
