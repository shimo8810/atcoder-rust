use proconio::{fastout, input};
use std::collections::HashMap;
use std::io::{stdout, Write};

const N: usize = 400;
const M: usize = 1995;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {_coord: [(u32, u32); N]}
    let mut graph = HashMap::new();
    for _ in 0..M {
        input! {x: usize, y: usize}
        graph.entry(x).or_insert_with(Vec::new).push(y);
        graph.entry(y).or_insert_with(Vec::new).push(x);
    }

    for _ in 0..M {
        input! {_l: u32}
        println!("{}", 1);
        stdout().flush().unwrap();
    }
}
