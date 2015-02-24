//#![feature(old_io)]
//// FIXME: unfinished
//use std::old_io::{BufferedReader, File};
//
fn main() {

}
//
//// #[deriving(Show)]
//pub struct FASTAReader<'a> {
//    cur_id: Option<String>,
//    cur_seq: String,
//    line_itr: &'a Iterator<Item=&'a Result>,
//    // line_itr: io::Lines<&'a str>,
//}
//
//impl<'a> FASTAReader<'a> {
//    pub fn new(file_h: &'a Path) -> FASTAReader<'a> {
//        FASTAReader {
//            cur_id: None,
//            cur_seq: String::from_str(""),
//            line_itr: &BufferedReader::new(File::open(file_h)).lines(),
//        }
//    }
//}
//
//impl<'a> Iterator for FASTAReader<'a> {
//    type Item = Option<(&'a str, &'a Option<str>)>;
//
//    fn next(&mut self) -> Option<(&'a str, &'a Option<str>)> {
//        loop {
//            match self.line_itr.next() {
//                None => {
//                    return Some((self.cur_seq.as_slice(), self.cur_id.unwrap().to_string()));
//                },
//                Some(l) => {
//                    let line = l.as_slice();
//                     
//                    if line.slice(0, 1) == ">" {
//                        return Some((self.cur_seq.as_slice(), self.cur_id.unwrap().to_string()));
//                        self.cur_id = Some(String::from_str(line.slice_from(1)));
//                        self.cur_seq = String::new();
//                    } else {
//                        self.cur_seq.push_str(line.trim());
//                    };
//                },
//            };
//        };
//    }
//}
