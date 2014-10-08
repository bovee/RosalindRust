use std::io;
 

fn main() {
    let data: String = io::stdin().chars().map(|x| x.unwrap()).collect();
    revc(data.as_slice());
}

fn revc(dna: &str) {
    for chr in dna.chars().rev() {
        let rev_comp = match chr.to_uppercase() {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => chr
        };
        print!("{}", rev_comp);
    };
}
