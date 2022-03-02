use itertools::Itertools;
use num::integer::gcd;
use proconio::{fastout, input};
use std::collections::HashSet;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
      N: usize,
      P: [(i64, i64); N]
    }

    let mut set = HashSet::new();
    for comb in P.iter().combinations(2) {
        //
        let &(x1, y1) = comb[0];
        let &(x2, y2) = comb[1];
        let dx = x2 - x1;
        let dy = y2 - y1;
        let z = gcd(dx, dy);

        if dx == 0 {
            set.insert((0, 1));
        } else if dy == 0 {
            set.insert((1, 0));
        } else if dx > 0 {
            set.insert((dx / z, dy / z));
        } else {
            set.insert((-dx / z, -dy / z));
        }
    }
    println!("{}", set.len() * 2);
}
