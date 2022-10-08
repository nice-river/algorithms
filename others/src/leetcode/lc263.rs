struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
    pub fn is_ugly(mut n: i32) -> bool {
        while n != 0 {
			if n % 2 == 0 {
				n /= 2;
			} else if n % 5 == 0 {
				n /= 5;
			} else if n % 3 == 0 {
				n /= 3;
			} else {
				break;
			}
		}
		n == 1
    }
}