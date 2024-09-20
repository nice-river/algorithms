#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let kx = 1;
        let ky = 1;
        let positions = to_vec!([[0, 0]]);
        let ans = 4;
        assert_eq!(Solution::max_moves(kx, ky, positions), ans);
    }

    #[test]
    fn test1() {
        let kx = 0;
        let ky = 0;
        let positions = to_vec!([[1, 2], [2, 4]]);
        let ans = 3;
        assert_eq!(Solution::max_moves(kx, ky, positions), ans);
    }

    #[test]
    fn test_dist() {
        let a = [0, 0];
        let b = [49, 49];
        assert_eq!(Solution::calc_dist(&a, &b), 34);
    }
}

use crate::*;

struct Solution {}

use std::collections::VecDeque;

impl Solution {
    const WIDTH: i32 = 50;

    pub fn max_moves(kx: i32, ky: i32, mut positions: Vec<Vec<i32>>) -> i32 {
        let n = positions.len();
        positions.push(vec![kx, ky]);
        let mut dist = vec![vec![0; n + 1]; n + 1];
        for i in 0..=n {
            for j in i + 1..=n {
                let d = Self::calc_dist(&positions[i], &positions[j]);
                dist[i][j] = d;
                dist[j][i] = d;
            }
        }
        let mut dp = vec![vec![vec![None::<i32>; n]; 1 << n]; 2];
        let mut ans = 0;
        for i in 0..n {
            ans = ans.max(Self::dfs(1 << i, i, &dist, &mut dp, 1) + dist[n][i]);
        }
        ans
    }

    fn dfs(
        status: usize,
        to: usize,
        dist: &Vec<Vec<i32>>,
        dp: &mut Vec<Vec<Vec<Option<i32>>>>,
        turn: usize,
    ) -> i32 {
        if let Some(x) = dp[turn][status][to] {
            return x;
        }
        let n = dist.len() - 1;
        if status + 1 == 1 << n {
            return 0;
        }
        let mut x = if turn == 0 { 0 } else { i32::MAX };
        let op = if turn == 0 {
            std::cmp::max::<i32>
        } else {
            std::cmp::min::<i32>
        };
        for i in 0..n {
            if status & (1 << i) == 0 {
                let r = Self::dfs(status | (1 << i), i, dist, dp, turn ^ 1);
                x = op(x, r + dist[to][i]);
            }
        }
        dp[turn][status][to] = Some(x);
        x
    }

    fn calc_dist(pos0: &[i32], pos1: &[i32]) -> i32 {
        let mut que = VecDeque::new();
        que.push_back((pos0[0], pos0[1], 0));
        let mut vis = vec![vec![false; Self::WIDTH as usize]; Self::WIDTH as usize];
        vis[pos0[0] as usize][pos0[1] as usize] = true;
        while let Some((x, y, s)) = que.pop_front() {
            if x == pos1[0] && y == pos1[1] {
                return s;
            }
            for dir in [
                (-1, 2),
                (1, 2),
                (2, 1),
                (2, -1),
                (1, -2),
                (-1, -2),
                (-2, -1),
                (-2, 1),
            ] {
                let nx = x + dir.0;
                let ny = y + dir.1;
                if nx < 0 || ny < 0 || nx >= Self::WIDTH || ny >= Self::WIDTH {
                    continue;
                }
		if vis[nx as usize][ny as usize] {
		    continue;
		}
                vis[nx as usize][ny as usize] = true;
                que.push_back((nx, ny, s + 1));
            }
        }
        unreachable!();
    }
}
