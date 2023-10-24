#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let tiles = to_vec2d([[1, 5], [10, 11], [12, 18], [20, 25], [30, 32]]);
        let carpet_len = 10;
        let ans = 9;
        assert_eq!(Solution::maximum_white_tiles(tiles, carpet_len), ans);
    }

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

use crate::*;

struct Solution {}

impl Solution {
    pub fn maximum_white_tiles(mut tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        tiles.sort();
        let n = tiles.len();
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + (tiles[i][1] - tiles[i][0]) as i64 + 1;
        }
        let mut ans = 0;
        for i in 0..n {
            let q = tiles[i][0] + carpet_len - 1;
            let mut l = i;
            let mut r = n;
            while l < r {
                let m = (l + r) / 2;
                if q < tiles[m][0] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            ans = ans.max((s[r - 1] - s[i]) as i32 + tiles[r - 1][1].min(q) - tiles[r - 1][0] + 1);
        }
        ans
    }
}
