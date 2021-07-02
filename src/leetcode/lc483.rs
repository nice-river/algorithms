struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = "13";
		let ans = "3";
		assert_eq!(Solution::smallest_good_base(n.to_string()), ans.to_string());
	}

	#[test]
	fn test1() {
		let n = "4681";
		let ans = "8";
		assert_eq!(Solution::smallest_good_base(n.to_string()), ans.to_string());
	}

	#[test]
	fn test2() {
		let n = "1000000000000000000";
		let ans = "999999999999999999";
		assert_eq!(Solution::smallest_good_base(n.to_string()), ans.to_string());
	}

	#[test]
	fn test3() {
		let n = ((1u64 << 63) - 1).to_string();
		let ans = "2";
		assert_eq!(Solution::smallest_good_base(n.to_string()), ans.to_string());
	}

}

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
		let n = n.parse().unwrap();
        let mut ans = n - 1;

		let mut i = 2;
        let mut t = n;
        while t != 0 {
            let ret = Solution::check(n, i);
			if ret != 0 {
				ans = ret;
			}
			t /= 2;
			i += 1;
		}

		ans.to_string()
    }

	fn check(n: u64, e: u64) -> u64 {
        let mut l = 2;
		let mut r = n;
		while l <= r {
			let m = l + (r - l) / 2;
			let mut cnt = 1;
            for _ in 1..=e {
                if cnt <= n / m {
					cnt = cnt * m + 1;
				} else if cnt > n / m {
                    cnt = n + 1;
					break;
				}
			}
            if cnt > n {
				r = m - 1;
			} else if cnt < n {
                l = m + 1;
			} else {
				return m;
			}
		}
		0
	}
}