use proconio::{fastout, input};
use std::collections::HashMap;

fn dfs(s: usize, t: usize, a: &[usize], memo: &mut HashMap<(usize, usize), usize>) -> usize {
    println!("{}-{} {:?}", s, t, &a[s..t]);
    if let Some(&ret) = memo.get(&(s, t)) {
        println!("{}-{} {:?} {}", s, t, &a[s..t], ret);
        ret
    } else {
        let ret = if t - s == 1 {
            a[s]
        } else {
            let mut ret = usize::MAX;
            for i in (s + 1)..t {
                ret = ret.min(dfs(s, i, a, memo) + dfs(i, t, a, memo));
            }
            ret
        };
        memo.insert((s, t), ret);
        println!("{}-{} {:?} {}", s, t, &a[s..t], ret);

        ret
    }
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [usize; N]
    }

    let mut memo = HashMap::new();

    let ans = dfs(0, N, &A, &mut memo);

    println!("{}", ans);
}
