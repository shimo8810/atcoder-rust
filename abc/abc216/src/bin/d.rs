use proconio::{fastout, input, marker::Usize1};

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let mut A = Vec::new();
    let mut tops = vec![vec![]; N];

    for i in 0..M {
        input! {
            k: usize,
            mut a: [Usize1; k]
        }
        let top = a.pop().unwrap();
        tops[top].push(i);
        A.push(a);
    }

    let mut list = vec![];
    for top in &tops {
        if top.len() == 2 {
            list.push(top.to_vec());
        }
    }
    while let Some(top) = list.pop() {
        for &p in &top {
            if let Some(x) = A[p].pop() {
                tops[x].push(p);
                if tops[x].len() == 2 {
                    list.push(tops[x].to_vec());
                }
            }
        }
    }

    let ans = if A.into_iter().all(|a| a.is_empty()) {
        YES
    } else {
        NO
    };
    println!("{}", ans);
}
