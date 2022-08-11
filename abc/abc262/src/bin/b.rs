use proconio::{fastout, input, marker::Usize1};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
    }

    let mut graph = vec![std::collections::HashSet::new(); N];
    for _ in 0..M {
        input! {U: Usize1, V: Usize1}
        graph[U].insert(V);
    }

    let mut ans = 0;

    for a in 0..N {
        for b in (a + 1)..N {
            for c in (b + 1)..N {
                if graph[a].contains(&b) && graph[b].contains(&c) && graph[a].contains(&c) {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
