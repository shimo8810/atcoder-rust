use proconio::input;

const YES: &str = "Yes";
const NO: &str = "No";

#[allow(non_snake_case)]
// #[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N]
    }
    let N = N.min(8);
    let mut v = vec![0; 200];
    for bits in 0..(1 << N) {
        let x = (0..N)
            .filter(|&i| (bits >> i) & 1 == 1)
            .fold(0, |x, i| (x + A[i]) % 200);

        if v[x as usize] == 0 {
            v[x as usize] = bits;
        } else {
            let f = |b| {
                let mut cnt = 0;
                let mut list = vec![];
                for i in 0..N {
                    if b >> i & 1 == 1 {
                        cnt += 1;
                        list.push(i + 1);
                    }
                }
                print!("{} ", cnt);
                for i in list.into_iter() {
                    print!("{} ", i);
                }
                println!();
            };
            println!("{}", YES);

            f(v[x as usize]);
            f(bits);
            return;
        }
    }
    println!("{}", NO);
}
