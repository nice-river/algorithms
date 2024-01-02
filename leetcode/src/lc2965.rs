#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len();
        let mut cnt = vec![0; n * n + 1];
        let mut ans = vec![0, 0];
        for row in &grid {
            for &cell in row {
                cnt[cell as usize] += 1;
            }
        }
        for i in 1..=n * n {
            if cnt[i] == 0 {
                ans[1] = i as i32;
            } else if cnt[i] == 2 {
                ans[0] = i as i32;
            }
        }
        ans
    }
}
