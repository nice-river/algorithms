struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}


use crate::leetcode::leetcode::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut map = BTreeMap::new();
        Solution::dfs(root.as_ref(), 0, 0, &mut map);
        let mut ans = vec![];
        let mut pre_col= -1;
        if let Some(((x, _),  _)) = map.iter().next() {
            pre_col = *x - 1;
        } else {
            return ans;
        }
        for ((x, _), mut arr) in map.into_iter() {
            arr.sort();
            if pre_col == x && ans.len() > 0 {
                ans.last_mut().unwrap().extend(arr);
            } else {
                ans.push(arr);
            }
            pre_col = x;
        }
        ans
    }

    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, x: i32, y: i32, map: &mut BTreeMap<(i32, i32), Vec<i32>>) {
        if let Some(root) = root {
            let node = root.as_ref().borrow();
            map.entry((x, y)).or_insert(Vec::new()).push(node.val);
            Solution::dfs(node.left.as_ref(), x - 1, y + 1, map);
            Solution::dfs(node.right.as_ref(), x + 1, y + 1, map);
        }
    }
}
