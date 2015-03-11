#![feature(io)]

use std::io::BufReadExt;
use std::collections::HashSet;

fn main() {
    let stdin = std::io::stdin();
    let r = stdin.lock();
    let mut r_lines = r.lines();

    // first line is number of nodes in the graph
    let n_nodes = r_lines.next().unwrap().unwrap().parse::<usize>();
    let n_nodes = n_nodes.unwrap();

    // create an array where each node is its own subgraph
    let mut graphs: Vec<u32> = (1..(n_nodes + 1) as u32).collect();
    for l in r_lines {
        let input = l.unwrap();
        let nums: Vec<usize> = input.split(' ').map(std::str::StrExt::parse::<usize>).map(|i| i.unwrap()).collect();
        let old_subg = graphs[nums[1] - 1];
        for i in (0..n_nodes) {
            if graphs[i] == old_subg {
                graphs[i] = graphs[nums[0] - 1];
            }
        }
        //println!("{:?}", graphs);
    }
    let g_set: HashSet<&u32> = graphs.iter().collect();
    //println!("{:?}", g_set);
    println!("{}", g_set.len() - 1);
}
