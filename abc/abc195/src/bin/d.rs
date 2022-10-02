use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        Q: u32,
        mut WV: [(u32, u32); N],
        mut X: [u32; M]
    }

    for _ in 0..Q {
        input! {L: Usize1, R: usize}

        let mut Z: Vec<_> = X[..L].iter().chain(X[R..].iter()).collect();
        Z.sort_unstable();

        let mut mrk = vec![false; N];

        let mut ans = 0;
        for &x in Z.into_iter() {
            let mut mx = 0;
            let mut mx_idx = None;
            for (j, &(w, v)) in WV.iter().enumerate() {
                if !mrk[j] && w <= x && mx < v {
                    mx_idx = Some(j);
                    mx = v;
                }
            }

            if let Some(idx) = mx_idx {
                mrk[idx] = true;
                ans += mx;
            }
        }
        println!("{}", ans);
    }
}
