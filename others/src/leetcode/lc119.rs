struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let row_index = 3;
		assert_eq!(Solution::get_row(row_index), vec![1, 3, 3, 1]);
		let row_index = 0;
		assert_eq!(Solution::get_row(row_index), vec![1]);
		let row_index = 1;
		assert_eq!(Solution::get_row(row_index), vec![1, 1]);
	}
}

impl Solution {
	pub fn get_row(row_index: i32) -> Vec<i32> {
		let row_index = row_index as usize;
		let mut ans = [Vec::with_capacity(row_index + 1), Vec::with_capacity(row_index + 1)];
		let mut cur = 0;
		ans[0].push(1);

		for i in 1..=row_index {
			let nxt = cur ^ 1;
			ans[nxt].clear();

			ans[nxt].push(1);
			for j in 1..i {
				ans[nxt].push(ans[cur][j] + ans[cur][j-1]);
			}
			ans[nxt].push(1);

			cur ^= 1;
		}

		ans[cur].clone()
	}


}