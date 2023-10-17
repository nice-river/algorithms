#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

    fn to_vec2d<T, O, I>(a: O) -> Vec<Vec<T>>
    where
        T: Clone,
        I: AsRef<[T]>,
        O: AsRef<[I]>,
    {
        a.as_ref()
            .iter()
            .map(|v| v.as_ref().to_vec())
            .collect::<Vec<_>>()
    }
}

use crate::*;

struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if root.is_none() {
            return ans;
        }
        let mut nodes = vec![vec![root.unwrap()], vec![]];
        while !nodes[0].is_empty() {
            let (cur, nxt) = nodes.split_at_mut(1);
            ans.push(cur[0][0].borrow().val);
            for node in cur[0].iter_mut() {
                let mut node = node.borrow_mut();
                if let Some(c) = node.right.take() {
                    nxt[0].push(c);
                }
                if let Some(c) = node.left.take() {
                    nxt[0].push(c);
                }
            }
            nodes.swap(0, 1);
            nodes[1].clear();
        }
        ans
    }
}
