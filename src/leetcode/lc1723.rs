struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test0() {
		let jobs = vec![3, 2];
		let k = 2;
		let ans = 3;
		assert_eq!(Solution::minimum_time_required(jobs, k), ans);
	}

	#[test]
	fn test1() {
		let jobs = vec![3, 2, 3];
		let k = 3;
		let ans = 3;
		assert_eq!(Solution::minimum_time_required(jobs, k), ans);
	}

	#[test]
	fn test2() {
		let jobs = vec![1, 2, 4, 7, 8];
		let k = 2;
		let ans = 11;
		assert_eq!(Solution::minimum_time_required(jobs, k), ans);
	}

	#[test]
	fn test3() {
		let jobs = vec![1, 2, 4, 7, 8];
		let k = 5;
		let ans = 8;
		assert_eq!(Solution::minimum_time_required(jobs, k), ans);
	}

	#[test]
	fn test4() {
		let jobs = vec![1, 2, 4, 7, 8];
		let k = 1;
		let ans = 22;
		assert_eq!(Solution::minimum_time_required(jobs, k), ans);

		let jobs = vec![3];
		let k = 1;
		let ans = 3;
		assert_eq!(Solution::minimum_time_required(jobs, k), ans);
	}

	#[test]
	fn test5() {
		let jobs = vec![1, 2, 3, 4];
		let k = 2;
		let ans = 5;
		assert_eq!(Solution::minimum_time_required(jobs, k), ans);
	}
}


impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
		if k == 1 {
			return jobs.into_iter().sum();
		}
		let mut jobs_sum = vec![0; 1usize << jobs.len()];
		for i in 1..jobs_sum.len() {
			let mut cur = 0;
			for j in 0..jobs.len() {
				if (1usize << j & i) != 0 {
                    cur += jobs[j];
				}
			}
			jobs_sum[i] = cur;
		}
		let k = k as usize;
        let mut col = 1usize << (jobs.len() - 1);
		let mut dp = vec![vec![i32::MAX; col]; k - 1];
        for j in 1..col {
            dp[0][j] = jobs_sum[j];
		}
		for i in 1..dp.len() {
            for j in 1..col {
				for k in 1..j {
					if j == k || (j & k) != k {
						continue;
					}
					dp[i][j] = dp[i][j].min(dp[i-1][k].max(jobs_sum[j ^ k]));
				}
			}
		}
		let mut ans = i32::MAX;

		let all = (1usize << jobs.len()) - 1;
		for j in 1..col {
			ans = ans.min(jobs_sum[all ^ j].max(dp.last().unwrap()[j]))
		}

		ans
    }
}