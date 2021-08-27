struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let weights = vec![3, 2, 2, 4, 1, 4];
		let d = 3;
		assert_eq!(Solution::ship_within_days(weights, d), 6);
	}
}



impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, d: i32) -> i32 {
        let mut x = 1;
		let mut y: i32 = weights.iter().sum();
        while x < y {
            let m = x + (y - x) / 2;
            if Solution::check(&weights, m, d) {
				y = m;
			} else {
				x = m + 1;
			}
		}
        x
    }

	fn check(weights: &[i32], limit: i32, mut d: i32) -> bool {
        let mut i = 0;
		let mut cur = 0 ;
		while i < weights.len() && d > 0 {
			if weights[i] + cur > limit {
				if cur == 0 {
					return false;
				}
				d -= 1;
				cur = 0;
			} else {
				cur += weights[i];
				i += 1;
			}
		}
        i == weights.len() && d > 0
	}
}