struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 1], vec![1, 3]];
        let target = 3;
        let ans = 1;
        assert_eq!(Solution::num_submatrix_sum_target(matrix, target), ans);
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut pre_sum = vec![vec![0; m]; n + 1];
        for i in 0..n {
            for j in 0..m {
                pre_sum[i + 1][j] = pre_sum[i][j] + matrix[i][j];
            }
        }

        let mut ans = 0;
        let mut map = HashMap::new();
        for x in 0..n {
            for y in x + 1..=n {
                map.clear();
                map.insert(0, 1);
                let mut cum = 0;
                for i in 0..m {
                    let t = pre_sum[y][i] - pre_sum[x][i];
                    cum += t;
                    ans += *map.get(&(cum - target)).unwrap_or(&0);
                    *map.entry(cum).or_insert(0) += 1;
                }
            }
        }
        ans
    }
}
