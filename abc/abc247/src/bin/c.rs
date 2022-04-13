use proconio::{fastout, input};

fn dfs(n: u32) {
    if n == 1 {
        print!("1 ");
        return;
    }

    dfs(n - 1);
    print!("{} ", n);
    dfs(n - 1);
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u32,
    }

    dfs(N);
}
