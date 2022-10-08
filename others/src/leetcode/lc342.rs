struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let n = 16;
		assert!(Solution::is_power_of_four(n));
	}
}


impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n <= 0 || (n & -n) != n {
			return false;
		}
		for i in 0..31 {
			if ((1 << i) & n) != 0 {
				return i % 2 == 0;
			}
		}
		false
    }
}