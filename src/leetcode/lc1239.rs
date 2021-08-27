#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let arr = vec!["un", "iq", "ue"];
		let ans = 4;
		let arr = arr.into_iter().map(|s| s.to_string()).collect();
		assert_eq!(Solution::max_length(arr), ans);
	}

	#[test]
	fn test1() {
		let arr = vec!["cha", "r", "act", "ers"];
		let ans = 6;
		let arr = arr.into_iter().map(|s| s.to_string()).collect();
		assert_eq!(Solution::max_length(arr), ans);
	}

	#[test]
	fn test2() {
		let arr = vec!["abcdefghijklmnopqrstuvwxyz", "abcdefghijklmnopqrstuvwxyz"];
		let ans = 26;
		let arr = arr.into_iter().map(|s| s.to_string()).collect();
		assert_eq!(Solution::max_length(arr), ans);
	}

	#[test]
	fn test3() {
		let arr = vec!["aa"];
		let ans = 0;
		let arr = arr.into_iter().map(|s| s.to_string()).collect();
		assert_eq!(Solution::max_length(arr), ans);
	}

	#[test]
	fn test4() {
		let arr = vec!["a", "c", "d"];
		let ans = 3;
		let arr = arr.into_iter().map(|s| s.to_string()).collect();
		assert_eq!(Solution::max_length(arr), ans);
	}
}

struct Solution {}


impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
	    const INVALID: u32 = (1 << 31) - 1;
	    let n = arr.len();
	    let mut dp = vec![INVALID; (1 << n)];
	    dp[0] = 0;
	    for i in 0..n {
		    let mut k = 0;
		    for &b in arr[i].as_bytes() {
			    if (1 << (b - b'a')) & k != 0 {
				    k = INVALID;
				    break;
			    }
			    k |= 1 << (b - b'a');
		    }
		    dp[1 << i] = k;
	    }

	    for i in 1..(1 << n) {
		    if dp[i] != INVALID {
			    for j in 0..n {
				    if ((1 << j) & i) == 0 && (dp[i] & dp[1 << j]) == 0 {
					    dp[(1 << j) | i] = dp[i] ^ dp[1 << j];
				    }
			    }
		    }
	    }
	    let mut ans = 0;
	    for val in dp {
		    if val != INVALID {
			    ans = ans.max(val.count_ones());
		    }
	    }
	    ans as i32
    }
}