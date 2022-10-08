struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn compare_version(version1: String, version2: String) -> i32 {
		let mut version1 = version1.split(".");
		let mut version2 = version2.split(".");
		while let Some(v1) = version1.next() {
			let v1 = v1.parse::<i32>().unwrap();
			if let Some(v2) = version2.next() {
				let v2 = v2.parse::<i32>().unwrap();
				if v1 < v2 {
					return -1;
				} else if v1 > v2 {
					return 1;
				}
			} else if v1 > 0 {
				return 1;
			}
		}
		while let Some(v2) = version2.next() {
			let v2 = v2.parse::<i32>().unwrap();
			if v2 > 0 {
				return -1;
			}
		}
		0
	}
}