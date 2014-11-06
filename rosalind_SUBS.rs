extern crate regex;

use std::io;
//use regex::Regex;

fn main () {
    let fullstr: String = io::stdin().read_line().unwrap();
    let substr: String = io::stdin().read_line().unwrap();

    for i in substr_locs(fullstr.trim(), substr.trim()).iter() {
        print!("{} ", i);
    };
    //println!("{}", );
}

#[test]
fn test_substr_locs() {
    assert_eq!(substr_locs("GATATATGCATATACTT", "ATAT"), vec![2, 4, 10]);
    assert_eq!(substr_locs("ATATGCGC", "ATGC"), vec![3]);
}

#[deriving(Show)]
pub struct MovingWindow<'a> {
    wrapped_str: &'a str,
    loc: uint,
    size: uint,
}

impl<'a> MovingWindow<'a> {
    pub fn new(wrapped_str: &'a str, size: uint) -> MovingWindow<'a> {
        MovingWindow {
            wrapped_str: wrapped_str,
            loc: 0,
            size: size,
        }
    }
}

impl<'a> Iterator<&'a str> for MovingWindow<'a> {
    fn next(&mut self) -> Option<&'a str> {
        self.loc += 1;
        if self.loc + self.size - 1 <= self.wrapped_str.len() {
            Some(self.wrapped_str.slice(self.loc - 1, self.loc + self.size - 1))
        } else {
            None
        }
    }
}

fn substr_locs(fullstr: &str, substr: &str) -> Vec<uint> {
    let mw = MovingWindow::new(fullstr, substr.len());
    let mut locs = vec![];
    for (i, win) in mw.enumerate() {
        if std::str::eq_slice(win, substr) {
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
