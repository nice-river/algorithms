#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
	    let n = n as usize;
	    let mut dp = vec![i32::MAX; n + 1];
	    dp[0] = 0;
	    for i in 1.. {
		    if i * i > n {
			    break;
		    }
		    let i = i * i;
		    for j in 0..=n-i {
			    dp[j + i] = dp[j + i].min(dp[j] + 1);
		    }
	    }
	    dp[n]
    }
}