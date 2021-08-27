#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
	pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
		let n = n as usize;
		let mut ans = 0;
		let mut pair_arr = vec![0; n];
		for pair in pairs {
			pair_arr[pair[0] as usize] = pair[1];
			pair_arr[pair[1] as usize] = pair[0];
		}
		let mut mat = vec![vec![0; n]; n];
		for (i, row) in preferences.iter().enumerate() {
			for (j, &c) in row.iter().enumerate() {
				mat[i][c as usize] = j;
			}
		}
		for x in 0..n {
			let y = pair_arr[x];
			for &u in &preferences[x] {
				if u == y {
					break;
				}
				let u = u as usize;
				let v = pair_arr[u];
				if mat[u][x] < mat[u][v as usize] {
					ans += 1;
					break;
				}
			}
		}
		ans
	}
}