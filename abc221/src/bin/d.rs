use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! { N: usize }

    let mut d = Vec::new();
    for _ in 0..N {
        input! { a: u32, b: u32 }
        d.push((a, 1)); // login
        d.push((a + b, -1)); // logout
    }
    d.sort_unstable();
    let mut ans = vec![0; N + 1];
    let mut cnt = 0;
    for i in 0..(d.len() - 1) {
        let (a1, b1) = d[i];
        let (a2, _) = d[i + 1];
        cnt += b1;
        ans[cnt as usize] += a2 - a1;
    }

    for a in ans.iter().skip(1) {
        print!("{} ", a);
    }
}
