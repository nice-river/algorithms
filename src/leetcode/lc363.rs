struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
		if n == 0 {
			return 0;
		}
		let m = matrix[0].len();
		let mut col_sum = vec![vec![0; m]; n + 1];
		for i in 0..n {
			for j in 0..m {
				col_sum[i + 1][j] = col_sum[i][j] + matrix[i][j];
			}
		}
        let mut ans = i32::MIN;

		let mut set = BTreeSet::new();

        for x in 0..n {
			for y in x+1..=n {
				let mut row_sum = 0;
                set.clear();
                set.insert(0);
                for j in 0..m {
                    row_sum += col_sum[y][j] - col_sum[x][j];
					for &v in set.range(row_sum - k..) {
                        ans = ans.max(row_sum - v);
						break;
					}
                    set.insert(row_sum);
				}
			}
		}
        ans
    }
}
