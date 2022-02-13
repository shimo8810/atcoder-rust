use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N]
    }
    // aのn回目の引数が入っている
    let mut map = std::collections::HashMap::new();
    for (i, &a) in A.iter().enumerate() {
        map.entry(a).or_insert_with(Vec::new).push(i);
    }

    for _ in 0..Q {
        input! {x: usize, k: usize}
        if let Some(x) = map.get(&x) {
            if x.len() >= k {
                println!("{}", x[k - 1] + 1);
            } else {
                println!("-1");
            }
        } else {
            println!("-1");
        }
    }
}
