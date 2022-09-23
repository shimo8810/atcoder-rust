use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        mut N: u64,
        C: [u64; 9]
    }
    // 桁数を求める
    let min = C.iter().min().unwrap();
    let k = N / min;

    let mut ans = vec![];
    for i in 0..k {
        for j in (0..9).rev() {
            if min * (k - i - 1) + C[j] <= N {
                N -= C[j];
                ans.push((b'0' + j as u8 + 1) as char);
                break;
            }
        }
    }

    println!("{}", ans.into_iter().collect::<String>());

    //     if((ll)mn * (length - 1 - i) + c[j] <= n) {
    //         n -= c[j];
    //         ans.push_back((char)('0' + j));
    //         break;
    //     }
    // }
}
