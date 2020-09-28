use proconio::input;
use proconio::marker::{Chars, Isize1};
use std::collections::{HashSet, VecDeque};

#[allow(non_snake_case)]
fn main() {
    input! {
        H: isize,
        W: isize,
        ch: Isize1,
        cw: Isize1,
        dh: Isize1,
        dw: Isize1,
        S: [Chars; H]
    }
    let mut set = HashSet::new();
    let mut deque = VecDeque::new();

    deque.push_front((ch, cw, 0));
    while let Some((y, x, d)) = deque.pop_front() {
        // 終了条件
        if y == dh && x == dw {
            println!("{}", d);
            return;
        }

        if set.contains(&(y, x)) {
            continue;
        }
        set.insert((y, x));

        for dx in -2..=2isize {
            for dy in -2..=2isize {
                // 中心は除外
                if dx == 0 && dy == 0 {
                    continue;
                }
                // 座標決め
                let h = y + dy;
                let w = x + dx;
                // 範囲外は除外
                if h < 0 || H <= h || w < 0 || W <= w {
                    continue;
                }
                // 移動可能でなければ除外
                if S[h as usize][w as usize] != '.' {
                    continue;
                }
                if dx.abs() + dy.abs() == 1 {
                    if !set.contains(&(h, w)) {
                        deque.push_front((h, w, d));
                    }
                } else {
                    if !set.contains(&(h, w)) {
                        deque.push_back((h, w, d + 1));
                    }
                }
            }
        }
    }

    println!("-1");
}
