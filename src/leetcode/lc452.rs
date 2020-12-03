struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

impl Solution {
	pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
		let mut arr = vec![];
		let n = points.len();
		for (idx, point) in points.into_iter().enumerate() {
			arr.push((point[0], 0, idx));
			arr.push((point[1], 1, idx));
		}
		arr.sort();
		let mut vis = vec![false; n];
		let mut que = Vec::with_capacity(n);
		let mut ans = 0;
		for (_, side, idx) in arr {
			if !vis[idx] {
				if side == 1 {
					ans += 1;
					for &q in que.iter() {
						vis[q] = true;
					}
					que.clear();
				} else {
					que.push(idx);
				}
			}
		}
		ans
	}
}