use proconio::{fastout, input};

#[allow(non_snake_case)]
fn dfs(i: usize, bx: &mut Vec<Vec<bool>>, A: usize, B: usize, H: usize, W: usize) -> usize {
    // 終了条件
    if i == H * W {
        return 1;
    }

    // 座標取得
    let x = i % W;
    let y = i / W;
    // 次のiがすでに埋まっている
    if bx[y][x] {
        return dfs(i + 1, bx, A, B, H, W);
    }

    let mut ans = 0;
    // Bを埋める
    if B > 0 {
        bx[y][x] = true;
        ans += dfs(i + 1, bx, A, B - 1, H, W);
        bx[y][x] = false;
    }

    // A を埋める
    if A > 0 {
        if x + 1 < W && !bx[y][x + 1] {
            bx[y][x] = true;
            bx[y][x + 1] = true;
            ans += dfs(i + 1, bx, A - 1, B, H, W);
            bx[y][x] = false;
            bx[y][x + 1] = false;
        }

        if y + 1 < H && !bx[y + 1][x] {
            bx[y + 1][x] = true;
            bx[y + 1][x] = true;
            ans += dfs(i + 1, bx, A - 1, B, H, W);
            bx[y][x] = false;
            bx[y + 1][x] = false;
        }
    }

    ans
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: usize,
        B: usize
    }

    let mut bx = vec![vec![false; W]; H];
    let ans = dfs(0, &mut bx, A, B, H, W);
    println!("{}", ans);
}
