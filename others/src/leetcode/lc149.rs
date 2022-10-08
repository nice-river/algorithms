struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let points = vec![[1,1],[2,2],[3,3]];
        let points = points.into_iter().map(|a| a.to_vec()).collect();
		assert_eq!(Solution::max_points(points), 3);
	}
}


impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 1;
		let n = points.len();
		for i in 0..n {
			for j in i+1..n {
				let mut cur = 2;
				for k in j+1..n {
					let a = (points[j][1] - points[i][1]) * (points[k][0] - points[i][0]);
					let b = (points[k][1] - points[i][1]) * (points[j][0] - points[i][0]);
					if a == b {
						cur += 1;
					}
				}
				ans = ans.max(cur);
			}
		}
		ans
    }
}