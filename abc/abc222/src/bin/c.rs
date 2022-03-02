use proconio::{input, marker::Chars};

fn judge(x: char, y: char) -> Option<usize> {
    if x == y {
        None
    } else if (x == 'G' && y == 'C') || (x == 'C' && y == 'P') || (x == 'P' && y == 'G') {
        Some(0)
    } else {
        Some(1)
    }
}

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [Chars; 2 * N]
    }

    let mut r: Vec<_> = (0..(2 * N)).map(|x| (0, x)).collect();

    for m in 0..M {
        for i in 0..N {
            //
            let a = r[2 * i];
            let b = r[2 * i + 1];

            if let Some(j) = judge(A[a.1][m], A[b.1][m]) {
                r[2 * i + j].0 -= 1;
            }
        }
        r.sort_unstable();
    }

    for (_, i) in r.iter() {
        println!("{}", i + 1);
    }
}
