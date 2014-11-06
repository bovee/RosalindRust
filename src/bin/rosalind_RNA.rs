use std::io;

fn main() {
    for chr in io::stdin().chars() {
        let chr = chr.ok().unwrap().to_uppercase();
        let rna_chr = match chr {
            'T' => 'U',
            _ => chr
        };
        print!("{}", rna_chr);
    };
}
