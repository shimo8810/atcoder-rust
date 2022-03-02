use proconio::{input, marker::Usize1};
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(non_snake_case)]
fn main() {
    input! { M: usize }

    let mut list = HashMap::new();
    for i in 0..9 {
        list.insert(i, Vec::new());
    }
    for _ in 0..M {
        input! {u1: Usize1, u2: Usize1}
        list.get_mut(&u1).unwrap().push(u2);
        list.get_mut(&u2).unwrap().push(u1);
    }

    let e = 8; // 空
    let crr = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    let mut p = [e; 9];
    for i in 0..8 {
        input! {mut x : Usize1}
        p[x] = i;
    }

    let mut que = VecDeque::new();
    let mut checked = HashSet::new();

    que.push_back((p, 0));
    checked.insert(p);

    while let Some((p, n)) = que.pop_front() {

        if p == crr {
            println!("{}", n);
            return;
        }

        let i = p.iter().position(|&x| x == e).unwrap();
        for &j in list.get(&i).unwrap() {
            let mut tmp = p;
            tmp.swap(i, j);
            if !checked.contains(&tmp) {
                checked.insert(tmp);
                que.push_back((tmp, n + 1));
            }
        }
    }

    println!("-1");
}
