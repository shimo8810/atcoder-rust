use proconio::input;
use std::collections::HashSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: [[u8;4];4]
    }
    let mut ans = 0;

    let n: u8 = A.iter().flatten().sum(); // number of village

    for bit in 1..(1 << 16usize) {
        let moats: Vec<_> = (0..16usize)
            .filter(|x| bit & (1 << x) != 0)
            .map(|x| (x / 4, x % 4))
            .collect();

        let mut stack = Vec::new();
        let mut set = HashSet::new();

        // check 2
        let m = moats.iter().filter(|(x, y)| A[*y][*x] == 1).count() as u8;
        if m != n {
            continue;
        }

        //
        stack.push(moats[0]);
        set.insert(moats[0]);
        while let Some((x, y)) = stack.pop() {
            for &(t, u) in moats.iter() {
                if set.contains(&(t, u)) {
                    continue;
                }

                if ((x == t) && (y == 1 + u || y + 1 == u))
                    || ((y == u) && (x == 1 + t || x + 1 == t))
                {
                    stack.push((t, u));
                    set.insert((t, u));
                }
            }
        }

        if set.len() != moats.len() {
            continue;
        }

        ans += 1;
    }

    println!("{}", ans);
}
