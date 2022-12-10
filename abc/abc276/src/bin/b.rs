use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut g = vec![vec![]; N];
    for _ in 0..M {
        input! {A: Usize1, B: Usize1}
        g[A].push(B);
        g[B].push(A);
    }
    for s in g.iter_mut() {
        s.sort_unstable();
        print!("{} ", s.len());
        for x in s.iter() {
            print!("{} ", x + 1);
        }
        println!();
    }
}
