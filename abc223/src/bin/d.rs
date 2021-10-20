use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize
    }

    let mut incnt = vec![0; N];
    let mut out = vec![Vec::new(); N];

    for _ in 0..M {
        input! { a: Usize1, b: Usize1 }
        incnt[b] += 1;
        out[a].push(b);
    }

    let mut heap: BinaryHeap<_> = incnt
        .iter()
        .enumerate()
        .filter(|(_, &n)| n == 0)
        .map(|(i, _)| Reverse(i))
        .collect();

    let mut ans = Vec::new();
    while let Some(Reverse(n)) = heap.pop() {
        ans.push(n + 1);
        for &x in out[n].iter() {
            incnt[x] -= 1;
            if incnt[x] == 0 {
                heap.push(Reverse(x));
            }
        }
    }

    if ans.is_empty() {
        println!("-1");
    } else {
        for (i, x) in ans.iter().enumerate() {
            print!("{}{}", x, if i <= N { " " } else { "\n" });
        }
    }
}
