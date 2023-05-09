#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use crate::leetcode::leetcode::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        Solution::dfs(root, &mut ans);
        ans
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if let Some(root) = root {
            let node = root.borrow();
            let left_sum = Solution::dfs(node.left.clone(), ans);
            let right_sum = Solution::dfs(node.right.clone(), ans);
            *ans += (left_sum - right_sum).abs();
            left_sum + right_sum + node.val
        } else {
            0
        }
    }
}
