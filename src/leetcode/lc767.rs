struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}


impl Solution {
	pub fn reorganize_string(s: String) -> String {
		let mut cnt = vec![0; 30];
		for &b in s.as_bytes().iter() {
			cnt[(b - b'a') as usize] += 1;
		}
		let mut cnt = cnt.iter().enumerate().map(|(i, &c)| (c, i)).collect::<Vec<(i32, usize)>>();
		cnt.sort_by_key(|e| std::cmp::Reverse(*e));
		let n = s.len();
		let b = if n % 2 == 0 {
			n / 2
		} else {
			n / 2 + 1
		};
		if cnt[0].0 > b as i32 {
			return "".to_string();
		}
		let mut ans = vec![0; n];
		let mut p = 0;
		for i in (0..n).step_by(2) {
			ans[i] = (cnt[p].1 as u8 + b'a');
			cnt[p].0 -= 1;
			if cnt[p].0 == 0 {
				p += 1;
			}
		}
		for i in (1..n).step_by(2) {
			ans[i] = (cnt[p].1 as u8 + b'a');
			cnt[p].0 -= 1;
			if cnt[p].0 == 0 {
				p += 1;
			}
		}
		std::str::from_utf8(&ans).unwrap().to_string()
	}
}