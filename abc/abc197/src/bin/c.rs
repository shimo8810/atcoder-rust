use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N]
    }

    let mut ans = std::u64::MAX;

    for bit in 0..(1 << (N - 1)) {
        let mut list: Vec<_> = (0..N - 1)
            .filter(|x| bit & (1 << x) != 0)
            .map(|x| x + 1)
            .collect();
        list.insert(0, 0);
        list.push(N);

        let mut tmp = 0;
        for w in list.windows(2) {
            let (h, t) = (w[0], w[1]);
            tmp ^= A[h..t].iter().fold(0, |acc, x| acc | x);
        }

        ans = ans.min(tmp);
        //
    }
    println!("{}", ans);
}
