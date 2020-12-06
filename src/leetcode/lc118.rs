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
	pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
		let num_rows = num_rows as usize;
		let mut ans = Vec::with_capacity(num_rows);
		if num_rows == 0 {
			return ans;
		}
		ans.push(vec![1]);
		for _ in 0..num_rows-1 {
			let prev = ans.last().unwrap();
			let mut v = Vec::with_capacity(prev.len() + 1);
			v.push(1);
			for i in 0..prev.len()-1 {
				v.push(prev[i] + prev[i + 1]);
			}
			v.push(1);
			ans.push(v);
		}
		ans
	}
}