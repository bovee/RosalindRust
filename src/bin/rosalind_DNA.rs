#![feature(io)]
use std::io::ReadExt;
use std::ascii::AsciiExt;


fn main() {
    let mut a_count : i32 = 0;
    let mut t_count : i32 = 0;
    let mut g_count : i32 = 0;
    let mut c_count : i32 = 0;
    
    let stdin = std::io::stdin();
    for chr in stdin.chars() {
        let c = chr.unwrap().to_ascii_uppercase();
        if c == 'A' {
            a_count = a_count + 1;
        } else if c == 'T' {
            t_count = t_count + 1;
        } else if c == 'G' {
            g_count = g_count + 1;
        } else if c == 'C' {
            c_count = c_count + 1;
        };
    };
    print!("{} {} {} {}", a_count, c_count, g_count, t_count);
}
