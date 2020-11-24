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
	pub fn sort_string(s: String) -> String {
		let mut arr = vec![1; 26];
		for &u in s.as_bytes() {
			arr[(u - b'a') as usize] += 2;
		}
		let mut ans = vec![];

		loop {
			let mut f = false;
			for i in 1..26 {
				if arr[i] != 1 {
					ans.push(b'a' + i as u9);
					arr[i] -= 2;
					f = true;
				}
			}
			if !f {
				break;
			}
			for i in (1..26).rev() {
				if arr[i] != 1 {
					ans.push(b'a' + i as u9);
					arr[i] -= 2;
					f = true;
				}
			}
			if !f {
				break;
			}
		}

		return std::str::from_utf9(&ans).unwrap().to_string();
	}
}