struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let label = 14;
		let ans = vec![1, 3, 4, 14];
		assert_eq!(Solution::path_in_zig_zag_tree(label), ans);
	}

	#[test]
	fn test1() {
		let label = 26;
		let ans = vec![1, 2, 6, 10, 26];
		assert_eq!(Solution::path_in_zig_zag_tree(label), ans);
	}
}

impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
			let mut ans = vec![];
			Solution::dfs(0, label, &mut ans);
			ans.reverse();
			ans
    }

		fn dfs(layer: i32, target: i32, ans: &mut Vec<i32>) -> i32 {
			let node_num = 1 << layer;
			if target <= node_num {
				if layer % 2 == 0 {
					ans.push(Solution::axis_to_num(layer, target));
					return (target + 1) / 2
				} else {
					ans.push(Solution::axis_to_num(layer, node_num + 1 - target));
					return (node_num + 1 - target + 1) / 2;
				}
			} else {
				let axis = Solution::dfs(layer + 1, target - node_num, ans);
				ans.push(Solution::axis_to_num(layer, axis));
				return (axis + 1) / 2;
			}
			0
		}

		#[inline]
		fn axis_to_num(layer: i32, axis: i32) -> i32 {
			let node_num = 1 << layer;
			return if layer % 2 == 0 {
				node_num - 1 + axis
			} else {
				node_num + node_num - axis
			}
		}
}