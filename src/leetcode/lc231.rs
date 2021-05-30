struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sol = Solution {};
	}
}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n >= 0 && n.count_ones() == 1
    }
}