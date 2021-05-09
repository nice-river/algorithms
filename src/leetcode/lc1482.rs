#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let bloom_day = vec![1,2,4,9,3,4,1];
		let m = 2;
		let k = 2;
		let ans = 4;
		assert_eq!(Solution::min_days(bloom_day, m, k), ans);
	}
}

struct Solution {}


impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let mut left = 1;
	    let mut right = *bloom_day.iter().max().unwrap() + 1;
        let mut ans = -1;
	    while left < right {
            let mid = left + (right - left) / 2;
            if Solution::check(&bloom_day, mid, m, k) {
	            ans = mid;
	            right = mid;
            } else {
	            left = mid + 1;
            }
	    }
	    ans
    }

	fn check(bloom_day: &Vec<i32>, day: i32, mut m: i32, k: i32) -> bool {
        let mut i = None;
        for j in 0..bloom_day.len() {
	        if bloom_day[j] <= day && i.is_none() {
		        i = Some(j);
	        }
	        if bloom_day[j] > day && i.is_some() {
                m -= (j - i.unwrap()) as i32 / k;
		        i = None;
	        }
        }
		if let Some(i) = i {
			m -= (bloom_day.len() - i) as i32 / k;
		}
		m <= 0
	}
}