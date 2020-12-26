struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
		g.sort();
		s.sort();
		let (mut i, mut j) = (0, 0);
		while j < s.len() && i < g.len() {
			if s[j] >= g[i] {
				i += 1;
			}
			j += 1;
		}
		i as i32
	}
}