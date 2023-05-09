#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use crate::leetcode::leetcode::TreeNode;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut sum_map = HashMap::new();
        sum_map.insert(0, 1);
        let mut ans = 0;
        Solution::dfs(&root, &mut sum_map, 0, target_sum, &mut ans);
        ans
    }

    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sum_map: &mut HashMap<i32, i32>,
        cur_sum: i32,
        target_sum: i32,
        ans: &mut i32,
    ) {
        if let Some(root) = root {
            let node = root.borrow();
            let s = node.val + cur_sum;
            if let Some(&x) = sum_map.get(&(s - target_sum)) {
                *ans += x;
            }
            *sum_map.entry(s).or_insert(0) += 1;
            Solution::dfs(&node.left, sum_map, s, target_sum, ans);
            Solution::dfs(&node.right, sum_map, s, target_sum, ans);
            *sum_map.get_mut(&s).unwrap() -= 1;
        }
    }
}
