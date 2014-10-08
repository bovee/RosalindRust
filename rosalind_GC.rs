use std::io;


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

fn gc(dna: &str) -> f32 {
    let mut gc_count: int = 0;
    let mut num_chars: int = 0;
    for chr in dna.trim().chars() {
        gc_count += match chr {
            'G' | 'C' | 'g' | 'c' => 1,
            _ => 0
        };
        num_chars += 1;
    };
    return (gc_count as f32) / (num_chars as f32);
}

//pub struct FASTAReader {
//    name: str,
//}
//
//pub impl<R: Reader> FASTAReader<R> {
//    fn new(inner: R) -> FASTAReader<R> {
//
//
//    }
//}
