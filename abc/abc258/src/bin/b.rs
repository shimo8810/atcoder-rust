use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: isize,
    }
    let mut A = vec![];
    for _ in 0..N {
        input! {s: Chars}
        A.push(
            s.into_iter()
                .map(|c| c as u64 - '0' as u64)
                .collect::<Vec<u64>>(),
        );
    }
    let moves = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            for (dx, dy) in &moves {
                let mut y = i;
                let mut x = j;
                let mut s = A[y as usize][x as usize];
                for _ in 0..(N - 1) {
                    x = (x + N + dx) % N;
                    y = (y + N + dy) % N;
                    s = s * 10 + A[y as usize][x as usize];
                }
                ans = ans.max(s);
                //
            }
        }
    }
    println!("{}", ans);
}
