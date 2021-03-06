use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufReadExt, Read, Lines};

pub struct FASTAReader<T> {
    cur_id: Option<String>,
    cur_seq: String,
    line_itr: Lines<BufReader<T>>,
}

impl<T: Read> FASTAReader<T> {
    pub fn new(r: T) -> FASTAReader<T> {
        let br = BufReader::new(r);
        FASTAReader {
            cur_id: None,
            cur_seq: String::new(),
            line_itr: br.lines(),
        }
    }
    pub fn from_path(file_h: &Path) -> FASTAReader<File> {
        let br = BufReader::new(File::open(file_h).unwrap());
        FASTAReader {
            cur_id: None,
            cur_seq: String::new(),
            line_itr: br.lines(),
        }
    }
}

impl<T: Read> Iterator for FASTAReader<T> {
    type Item = (String, Option<String>);

    fn next(&mut self) -> Option<(String, Option<String>)> {
        loop {
            match self.line_itr.next() {
                None => {
                    if self.cur_seq.len() == 0 {
                        return None
                    }
                    let ret = Some((self.cur_seq.clone(), self.cur_id.clone()));
                    self.cur_seq = String::new();
                    return ret;
                },
                Some(Ok(l)) => {
                    if &l[..1] == ">" {
                        let mut ret = None;
                        if self.cur_seq.len() != 0 {
                            ret = Some((self.cur_seq.clone(), self.cur_id.clone()));
                        }
                        self.cur_id = Some(l[1..].to_string());
                        self.cur_seq = String::new();
                        if ret != None {
                            return ret;
                        }
                    } else {
                        self.cur_seq.push_str(l.trim());
                    };
                },
                Some(Err(_)) => {
                    return None;
                }
            };
        };
    }
}
