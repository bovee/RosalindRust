#![feature(old_io)]
extern crate bio;

use std::old_io::{stdin};
use bio::gc;


fn main() {
    let mut max_gc: f32 = 0f32;
    let mut max_gc_name: String = String::new();
    let mut cur_name: String = String::new();
    let mut cur_seq: String = String::new();

    let mut stdin = stdin();
    for l in stdin.lock().lines().map(|x| x.unwrap()) {
        if &l[0..1] == ">" {
            if gc(&cur_seq) > max_gc {
                max_gc = gc(&cur_seq);
                max_gc_name = cur_name.to_string();
            };
            cur_name = l[1..].to_string();
            cur_seq = String::new();
        } else {
            cur_seq.push_str(l.trim());
        };
    };
    if gc(&cur_seq[..]) > max_gc {
        max_gc = gc(&cur_seq);
        max_gc_name = cur_name.to_string();
    };
    println!("{}", max_gc_name);
    println!("{}", 100f32 * max_gc);
}
