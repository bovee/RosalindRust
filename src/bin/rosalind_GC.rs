extern crate bio;

use std::io;
use bio::gc;


fn main() {
    let mut max_gc: f32 = 0f32;
    let mut max_gc_name: String = String::new();
    let mut cur_name: String = String::new();
    let mut cur_seq: String = String::new();

    for l in io::stdin().lines().map(|x| x.unwrap()) {
        let line = l.as_slice();
         
        if line.slice(0, 1) == ">" {
            if gc(cur_seq.as_slice()) > max_gc {
                max_gc = gc(cur_seq.as_slice());
                max_gc_name = cur_name.to_string();
            };
            cur_name = String::from_str(line.slice_from(1));
            cur_seq = String::new();
        } else {
            cur_seq.push_str(line.trim());
        };
    };
    if gc(cur_seq.as_slice()) > max_gc {
        max_gc = gc(cur_seq.as_slice());
        max_gc_name = cur_name.to_string();
    };
    println!("{}", max_gc_name);
    println!("{}", 100f32 * max_gc);
}
