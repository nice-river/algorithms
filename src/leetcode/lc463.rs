struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
		let n = grid.len();
		let mut ans = 0;
		if n == 0 {
			return ans;
		}
		let m = grid.get(0).unwrap().len();
		let dirs = vec![-1, 0, 1, 0, -1];
		for (i, row) in grid.iter().enumerate() {
			for (j, cell) in row.iter().enumerate() {
				if cell == &1 {
					for d in 0..4 {
						let x = i as i32 + dirs[d];
						let y = j as i32 + dirs[d + 1];
						if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 || &grid[x as usize][y as usize] == &0 {
							ans += 1;
						}
					}
				}
			}
		}
		ans
	}
}