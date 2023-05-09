#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 5;
		let ans = 5;
		assert_eq!(Solution::find_integers(n), ans);
	}

	#[test]
	fn test1() {
		let n = 2;
		let ans = 3;
		assert_eq!(Solution::find_integers(n), ans);
	}
}

struct Solution {}


impl Solution {
    pub fn find_integers(mut n: i32) -> i32 {
	    let mut bits = Vec::with_capacity(32);
	    while n != 0 {
		    bits.push(n & 1);
		    n >>= 1;
	    }
	    bits.reverse();
	    let mut dp = vec![vec![-1; bits.len()]; 2];;
	    let mut ans = Solution::dfs(1, 0, true, true, &bits, &mut dp);
	    ans += Solution::dfs(1, 1, false, false, &bits, &mut dp);
	    ans
    }

	fn dfs(idx: usize, fill_bit: usize, leading_zero: bool, is_less: bool, bits: &Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
		if idx == bits.len() {
			return 1;
		}
		if is_less && dp[fill_bit][idx] != -1 {
			return dp[fill_bit][idx];
		}
		let bound = if is_less { 1 } else { bits[idx] };

		let mut cnt = 0;
		for i in 0..=bound {
			if i == 0 {
				cnt += Solution::dfs(idx + 1, 0, leading_zero, is_less | (i < bound), bits, dp);
			} else if fill_bit != 1 {
				cnt += Solution::dfs(idx + 1, 1, false, is_less, bits, dp);
			}
		}

		if is_less {
			dp[fill_bit][idx] = cnt;
		}
		return cnt;
	}
}
