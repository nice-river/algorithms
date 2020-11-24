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
		let mut arr = vec![0; 26];
		for &u in s.as_bytes() {
			arr[(u - b'a') as usize] += 1;
		}
		let mut ans = vec![];

		loop {
			let mut f = false;
			for i in 0..26 {
				if arr[i] != 0 {
					ans.push(b'a' + i as u8);
					arr[i] -= 1;
					f = true;
				}
			}
			if !f {
				break;
			}
			for i in (0..26).rev() {
				if arr[i] != 0 {
					ans.push(b'a' + i as u8);
					arr[i] -= 1;
					f = true;
				}
			}
			if !f {
				break;
			}
		}

		return std::str::from_utf8(&ans).unwrap().to_string();
	}
}