use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut S: Chars,
        Q: usize,
    }

    let mut flip = false;
    for _ in 0..Q {
        input! {T: u8, mut A: usize, mut B: usize}

        if T == 1 {
            A -= 1;
            B -= 1;
            if flip {
                A = (A + N) % (2 * N);
                B = (B + N) % (2 * N);
            }
            S.swap(A, B);
        } else {
            flip = !flip;
        }
    }

    let ans = if flip {
        S[N..].iter().collect::<String>() + &S[..N].iter().collect::<String>()
    } else {
        S.iter().collect::<String>()
    };

    println!("{}", ans);
}
