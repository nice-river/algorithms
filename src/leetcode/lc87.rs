struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s1 = String::from("ab");
		let s2 = String::from("ba");
        assert!(Solution::is_scramble(s1, s2));
	}
}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes();
		let s2 = s2.as_bytes();
		let mut dp = vec![vec![vec![None; s1.len() + 1]; s1.len()]; s1.len()];
        Solution::dfs(0, 0, s1.len(), s1, s2, &mut dp)
    }

	fn dfs(i: usize, j: usize, k: usize, s1: &[u8], s2: &[u8], dp: &mut Vec<Vec<Vec<Option<bool>>>>) -> bool {
        if let Some(res) = dp[i][j][k] {
			return res;
		}
		if k == 1 && s1[i] == s2[j] {
			return true;
		}
        let mut res = false;
        for l in 1..k {
			if Solution::dfs(i, j, l, s1, s2, dp) & Solution::dfs(i + l, j + l, k - l, s1, s2, dp) {
                res = true;
                break;
			}
			if Solution::dfs(i, j + k - l, l, s1, s2, dp) & Solution::dfs(i + l, j, k - l, s1, s2, dp) {
                res = true;
				break;
			}
		}
		dp[i][j][k] = Some(res);
        res
	}
}