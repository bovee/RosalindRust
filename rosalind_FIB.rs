use std::io;


fn main() {
    loop {
        let input: String = io::stdin().read_line().unwrap();
        let nums: Vec<&str> = input.as_slice().split(' ').collect();
        let opt_n: Option<int> = from_str(nums[0].trim());
        let opt_k: Option<int> = from_str(nums[1].trim());
        match (opt_n, opt_k) {
            (Some(n), Some(k)) => {
                println!("{}", fib(n, k));
            },
            (_, _) => {
                println!("Invalid Input");
                break;
            }
        }
    }
}

fn fib(n: int, k: int) -> int {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1, k) + k * fib(n - 2, k),
    }
}
