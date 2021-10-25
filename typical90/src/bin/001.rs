use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
        K: usize,
        A: [usize; N]
    }
    let mut len = L / (K + 1);
    let mut left = 0;
    let mut right = len;

    let mut c = 0;
    loop {
        c += 1;
        if c > 20 {
            break;
        }
        let mut offset = 0;
        let mut min = L;
        let mut k = 0;
        for &a in A.iter() {
            if a - offset >= len {
                min = std::cmp::min(a - offset, min);
                offset = a;
                k += 1;
            }
        }

        min = std::cmp::min(L - offset, min);
        println!("{}-{}-{}: {}-{}", left, len, right, min, k);

        if min >= len && k >= K {
            left = len;
            len = (right + len) / 2;
        } else {
            right = len;
            len = (left + len) / 2;
        }
    }
}
