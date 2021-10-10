use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn judge(x: char, y: char) -> (usize, usize) {
    if x == 'G' {
        if y == 'G' {
            (0, 0)
        } else if y == 'C' {
            (1, 0)
        } else {
            // P
            (0, 1)
        }
    } else if x == 'C' {
        if y == 'G' {
            (0, 1)
        } else if y == 'C' {
            (0, 0)
        } else {
            // P
            (1, 0)
        }
    } else {
        // P
        if y == 'G' {
            (1, 0)
        } else if y == 'C' {
            (0, 1)
        } else {
            // P
            (0, 0)
        }
    }
    // return (0, 0);
}
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [Chars; 2 * N]
    }

    let mut dic = HashMap::new();
    for i in 0..N {
        dic.insert(2 * i, 0);
        dic.insert(2 * i + 1, 0);
    }

    for m in 0..M {
        //
        let tmp = dic.clone();
        let mut v: Vec<_> = tmp.iter().collect();
        v.sort_by(|(&i, &x), (&j, &y)| (99 - j + 100 * y).cmp(&(99 - i + 100 * x)));

        for i in 0..N {
            let (&ai, _) = v[2 * i];
            let (&bi, _) = v[2 * i + 1];
            let (da, db) = judge(A[ai][m], A[bi][m]);
            let aa = dic.entry(ai).or_insert(0);
            *aa += da;
            let bb = dic.entry(bi).or_insert(0);
            *bb += db;
        }
    }

    let mut v: Vec<_> = dic.iter().collect();
    v.sort_by(|(&i, &x), (&j, &y)| (99 - j + 100 * y).cmp(&(99 - i + 100 * x)));

    for (&i, _) in v.iter() {
        println!("{}", i + 1);
    }
}
