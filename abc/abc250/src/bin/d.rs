use proconio::{fastout, input};

const M: u128 = 1_000_000;

#[allow(non_snake_case)]
fn sieve() -> Vec<u128> {
    let mut nums = vec![];
    let mut isp = vec![true; M as usize];
    isp[0] = false;
    isp[1] = false;

    for n in 2..M {
        if isp[n as usize] {
            nums.push(n);
            let mut i = 2;
            while n * i < M {
                isp[(n * i) as usize] = false;
                i += 1;
            }
        }
    }
    nums
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u128,
    }

    let mut ans = 0;
    let nums = sieve();

    for (i, &p) in nums.iter().enumerate() {
        let mut ng = nums.len() - 1;
        let mut ok = i;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            let q = nums[mid];

            if p * q * q * q <= N {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        if i != ok {
            ans += ok - i;
        }
    }
    println!("{}", ans);
}
