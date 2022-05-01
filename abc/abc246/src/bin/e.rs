use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use std::collections::VecDeque;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        A: (Usize1, Usize1),
        B: (Usize1, Usize1),
        S: [Chars; N]
    }

    let dxy = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut visited = vec![vec![vec![false; 4]; N]; N];
    let mut dist = vec![vec![vec![std::usize::MAX; 4]; N]; N];
    let mut deq = VecDeque::new();

    for (i, (dx, dy)) in dxy.iter().enumerate() {
        let nx = A.0 as isize + dx;
        let ny = A.1 as isize + dy;

        if nx >= 0
            && ny >= 0
            && nx < N as isize
            && ny < N as isize
            && S[nx as usize][ny as usize] == '.'
        {
            deq.push_back((nx, ny, i));
            dist[nx as usize][ny as usize][i] = 1;
        }
    }

    while let Some((x, y, d)) = deq.pop_front() {
        if B.0 == x as usize && B.1 == y as usize {
            println!("{}", dist[x as usize][y as usize][d]);
            return;
        }

        visited[x as usize][y as usize][d] = true;

        for (i, (dx, dy)) in dxy.iter().enumerate() {
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0
                && ny >= 0
                && nx < N as isize
                && ny < N as isize
                && S[nx as usize][ny as usize] == '.'
                && !visited[nx as usize][ny as usize][i]
            {
                if d == i {
                    if dist[nx as usize][ny as usize][i] > dist[x as usize][y as usize][d] {
                        deq.push_front((nx, ny, i));
                        dist[nx as usize][ny as usize][i] = dist[x as usize][y as usize][d];
                    }
                } else {
                    if dist[nx as usize][ny as usize][i] > dist[x as usize][y as usize][d] + 1 {
                        deq.push_back((nx, ny, i));
                        dist[nx as usize][ny as usize][i] = dist[x as usize][y as usize][d] + 1;
                    }
                }
            }
        }
    }
    println!("-1");
}
