#![feature(io)]
extern crate regex;
extern crate hyper;
extern crate bio;

use std::io::{Read, Cursor, BufReadExt};
//use regex::Regex;
use hyper::Client;
use hyper::header::Connection;
use hyper::header::ConnectionOption;
use bio::fasta::FASTAReader;
use bio::MovingWindow;

fn get_uniprot_seq(seq_id: &str) -> String {
    let mut client = Client::new();

    let path = format!("http://www.uniprot.org/uniprot/{}.fasta", seq_id);
    let mut res = client.get(&path[..])
        .header(Connection(vec![ConnectionOption::Close]))
        .send().unwrap();
    // TODO: there's probably a way to not unwrap into a string we're just going
    // to create a cursor on anyways
    let mut seq = String::new();
    res.read_to_string(&mut seq).unwrap();
    seq
}


fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let seq_id = l.trim();
        let fasta = get_uniprot_seq(seq_id);
        let mut locs = Vec::new();

        for (seq, _) in FASTAReader::new(Cursor::new(fasta.into_bytes())) {
            let mw = MovingWindow::new(&seq[..], 4);
            for (i, w) in mw.enumerate() {
                let win: Vec<char> = w.chars().collect();
                if win[0] == 'N' && win[1] != 'P' && (win[2] == 'S' || win[2] == 'T') && win[3] != 'P' {
                    locs.push(i + 1);
                }
            }
        }

        //// This doesn't work -> Regexes is non-overlapping
        //// when there's lookahead support, we can turn this back on
        //let re = Regex::new(r"N[^P][ST][^P]").unwrap();
        //for (seq, _) in FASTAReader::new(Cursor::new(seq.into_bytes())) {
        //    for (st_pos, _) in re.find_iter(&seq[..]) {
        //        locs.push(st_pos + 1);
        //    }
        //}
        
        // print out the positions
        if locs.len() != 0 {
            println!("{}", seq_id);
            // TODO: find a a better way to do " ".join(pos as str)
            let pos_str = locs.iter().map(|n| n.to_string())
                .fold(String::new(), |mut a, b| {
                    a.push_str(" ");
                    a.push_str(&b[..]);
                    a.trim().to_string()
                });
            println!("{}", pos_str);
        }
    }
}
