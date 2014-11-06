use std::io;
 

fn main() {
    let input: String = io::stdin().read_line().unwrap();
    let nums: Vec<&str> = input.as_slice().trim().split(' ').collect();
    let k: f32 = from_str(nums[0]).unwrap();
    let m: f32 = from_str(nums[1]).unwrap();
    let n: f32 = from_str(nums[2]).unwrap();
    let inv_odds = ((n * (n - 1.)) + 0.25 * (m * (m - 1.)) + m * n) / ((k + m + n) * (k + m + n - 1.));
    let odds = 1. - inv_odds;
    println!("{}", odds);
}

//      HH   Hh   hh
// HH   0    0    0
// Hh   0    0.25 0.5
// hh   0    0.5  1
//
