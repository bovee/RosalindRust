use std::io;
 

fn main() {
    let mut stdin = io::stdin();
    let line1 = stdin.read_line().unwrap();
    let line2 = stdin.read_line().unwrap();
    let l1 = line1.as_slice();
    let l2 = line2.as_slice();
    println!("{}", hamming_dist(l1, l2));
    //println!("{}", l1.lev_distance(l2));
}

fn hamming_dist(str1: &str, str2: &str) -> int {
    let mut dif_chars: int = 0;
    for (c1, c2) in str1.chars().zip(str2.chars()) {
        if c1 != c2 {
            dif_chars += 1;
        }
    }
    return dif_chars;
}
