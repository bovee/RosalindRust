use std::io;
 

fn main() {
    let mut a_count : int = 0;
    let mut t_count : int = 0;
    let mut g_count : int = 0;
    let mut c_count : int = 0;

    for chr in io::stdin().chars() {
        let c = chr.unwrap().to_uppercase();
        if c == 'A' {
            a_count = a_count + 1;
        } else if c == 'T' {
            t_count = t_count + 1;
        } else if c == 'G' {
            g_count = g_count + 1;
        } else if c == 'C' {
            c_count = c_count + 1;
        };
    };
    print!("{} {} {} {}", a_count, c_count, g_count, t_count);
}
