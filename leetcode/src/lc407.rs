#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let height_map = vec![
            [3, 3, 3, 3, 3],
            [3, 2, 2, 2, 3],
            [3, 2, 1, 2, 3],
            [3, 2, 2, 2, 3],
            [3, 3, 3, 3, 3],
        ];
        let height_map = height_map.into_iter().map(|arr| arr.to_vec()).collect();
        let ans = 10;
        assert_eq!(Solution::trap_rain_water(height_map), ans);
    }

    #[test]
    fn test1() {
        let height_map = vec![[3, 3, 3], [3, 2, 3], [3, 3, 3]];
        let height_map = height_map.into_iter().map(|arr| arr.to_vec()).collect();
        let ans = 1;
        assert_eq!(Solution::trap_rain_water(height_map), ans);
    }

    #[test]
    fn test2() {
        let height_map = vec![[1, 4, 3, 1, 3, 2], [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1]];
        let height_map = height_map.into_iter().map(|arr| arr.to_vec()).collect();
        let ans = 4;
        assert_eq!(Solution::trap_rain_water(height_map), ans);
    }

    #[test]
    fn test3() {
        let height_map = vec![
            [9, 9, 9, 9, 9],
            [9, 2, 1, 2, 9],
            [9, 2, 8, 2, 9],
            [9, 2, 3, 2, 9],
            [9, 9, 9, 9, 9],
        ];
        let height_map = height_map.into_iter().map(|arr| arr.to_vec()).collect();
        let ans = 57;
        assert_eq!(Solution::trap_rain_water(height_map), ans);
    }

    #[test]
    fn test4() {
        let height_map = vec![
            [14, 17, 18, 16, 14, 16],
            [17, 3, 10, 2, 3, 8],
            [11, 10, 4, 7, 1, 7],
            [13, 7, 2, 9, 8, 10],
            [13, 1, 3, 4, 8, 6],
            [20, 3, 3, 9, 10, 8],
        ];
        let height_map = height_map.into_iter().map(|arr| arr.to_vec()).collect();
        let ans = 25;
        assert_eq!(Solution::trap_rain_water(height_map), ans);
    }
}

struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

static DIRECTION: [i32; 5] = [0, -1, 0, 1, 0];

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        let m = height_map[0].len();
        if n <= 2 || m <= 2 {
            return 0;
        }
        let mut vis = vec![vec![false; m]; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            vis[i][0] = true;
            vis[i][m - 1] = true;
            heap.push((Reverse(height_map[i][0]), i, 0));
            heap.push((Reverse(height_map[i][m - 1]), i, m - 1));
        }
        for j in 0..m {
            vis[0][j] = true;
            vis[n - 1][j] = true;
            heap.push((Reverse(height_map[0][j]), 0, j));
            heap.push((Reverse(height_map[n - 1][j]), n - 1, j));
        }
        let mut ans = 0;
        while !heap.is_empty() {
            let (Reverse(cur_height), x, y) = heap.pop().unwrap();
            for d in 0..4 {
                let nx = x as i32 + DIRECTION[d];
                let ny = y as i32 + DIRECTION[d + 1];
                if nx < 0 || ny < 0 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if nx >= n || ny >= m || vis[nx][ny] {
                    continue;
                }
                if cur_height > height_map[nx][ny] {
                    ans += cur_height - height_map[nx][ny];
                }
                vis[nx][ny] = true;
                heap.push((Reverse(cur_height.max(height_map[nx][ny])), nx, ny));
            }
        }
        ans
    }
}
