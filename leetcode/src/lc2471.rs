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

struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut arr = vec![];
        let mut ans = 0;
        let mut que = VecDeque::new();
        if root.is_none() {
            return 0;
        }
        que.push_back((root.unwrap(), 1));
        while let Some((root, layer)) = que.pop_front() {
            if arr.len() < layer {
                arr.push(vec![]);
            }
            arr[layer - 1].push(root.borrow().val);
            if let Some(left) = root.borrow_mut().left.take() {
                que.push_back((left, layer + 1));
            }
            if let Some(right) = root.borrow_mut().right.take() {
                que.push_back((right, layer + 1));
            }
        }
        for a in arr {
            ans += Self::calc(a);
        }
        ans
    }

    fn calc(mut arr: Vec<i32>) -> i32 {
        let mut pos = HashMap::new();
        let mut brr = arr.clone();
        brr.sort();
        let mut ret = 0;
        for (i, &num) in arr.iter().enumerate() {
            pos.insert(num, i);
        }
        for i in 0..arr.len() {
            let num = arr[i];
            if num != brr[i] {
                ret += 1;
                let &p = pos.get(&brr[i]).unwrap();
                arr[p] = num;
                pos.insert(num, p);
            }
        }
        ret
    }
}
