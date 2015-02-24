#![feature(old_io)]
//extern crate regex;
extern crate bio;

use std::old_io::{stdin};

use bio::MovingWindow;
//use regex::Regex;

fn main () {
    let fullstr: String = stdin().read_line().unwrap();
    let substr: String = stdin().read_line().unwrap();

    for i in substr_locs(fullstr.trim(), substr.trim()).iter() {
        print!("{} ", i);
    };
}

fn substr_locs(fullstr: &str, substr: &str) -> Vec<usize> {
    let mw = MovingWindow::new(fullstr, substr.len());
    let mut locs = vec![];
    for (i, win) in mw.enumerate() {
        //if std::str::eq_slice(win, substr) {
        if win == substr {
            // do i + 1 to keep it at 1-based indexing for rosalind answer
            locs.push(i + 1);
        };
    };
    locs
}

//// unsuccessful regex attempt -> doens't return overlapping windows
//fn substr_locs(fullstr: &str, substr: &str) -> Vec<int> {
//    // TODO: doesn't return overlapping substrings
//    let re = Regex::new(regex::quote(substr).as_slice()).unwrap();
//    for (st, en) in re.find_iter(fullstr) {
//        println!("{}", st);
//    };
//}
