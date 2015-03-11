#![feature(io)]
use std::io::BufReadExt;


fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                let nums: Vec<&str> = input.split(' ').collect();
                let opt_n = nums[0].trim().parse::<i32>();
                let opt_k = nums[1].trim().parse::<i32>();
                match (opt_n, opt_k) {
                    (Ok(n), Ok(k)) => {
                        println!("{}", fib(n, k));
                    },
                    (_, _) => {
                        println!("Invalid Input");
                        break;
                    }
                }
            },
            Err(_) => {
                break;
            }
        }
    }
}

fn fib(n: i32, k: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1, k) + k * fib(n - 2, k),
    }
}
