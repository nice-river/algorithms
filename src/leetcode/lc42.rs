struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let height = [2, 2,2,2,2,2,2,2,2];
		assert_eq!(Solution::trap(height.to_vec()), 0);

		let height = [2, 3];
		assert_eq!(Solution::trap(height.to_vec()), 0);

		let height = [2];
		assert_eq!(Solution::trap(height.to_vec()), 0);

		let height = [100,1,2,3,4,3,2,1,10000];
		assert_eq!(Solution::trap(height.to_vec()), 12 + 96 * 7);

		let height = [3,1,2];
		assert_eq!(Solution::trap(height.to_vec()), 1);

		let height = [3,2,1];
		assert_eq!(Solution::trap(height.to_vec()), 0);

		let height = [1,2,3];
		assert_eq!(Solution::trap(height.to_vec()), 0);

		let height = [0,1,0,2,1,0,1,3,2,1,2,1];
		assert_eq!(Solution::trap(height.to_vec()), 6);
	}
}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
		let mut stk = vec![];
		let mut ans = 0;
        for i in 0..height.len() {
            while !stk.is_empty() && height[i] > height[stk[stk.len()-1]] {
                let b = stk.pop().unwrap();
                if let Some(&a) = stk.last() {
                    ans += (i - a - 1) as i32 * (height[i].min(height[a]) - height[b]);
				}
			}
			stk.push(i);
		}
        ans
    }
}