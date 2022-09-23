use proconio::{fastout, input, marker::Chars};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        S: Chars,
        W: [u32; N]
    }

    let mut WS: Vec<_> = S
        .into_iter()
        .zip(W.into_iter())
        .map(|(s, w)| (w, s))
        .collect();

    let mut ans = WS.iter().filter(|(_, s)| *s == '1').count();
    WS.sort_unstable();
    let mut tmp = ans;
    for (i, &(_, s)) in WS.iter().enumerate() {
        if s == '1' {
            tmp -= 1;
        } else {
            tmp += 1;
        }

        if i < N - 1 {
            if WS[i].0 != WS[i + 1].0 {
                ans = ans.max(tmp);
            }
        } else {
            ans = ans.max(tmp);
        }
    }

    println!("{}", ans);
}
