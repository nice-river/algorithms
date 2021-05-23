#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let nums = vec![0,1,2,3,4];
        let queries = vec![vec![3,1],vec![1,3],vec![5,6]];
        let ans = vec![3, 3,7];
        assert_eq!(Solution::maximize_xor(nums, queries), ans);
	}

    #[test]
    fn test1() {
        let nums = vec![2];
        let queries = vec![vec![1, 3]];
        let ans = vec![3];
        assert_eq!(Solution::maximize_xor(nums, queries), ans);
    }
}

struct Solution {}

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default, Debug)]
struct TrieNode {
    children: [Option<Rc<RefCell<TrieNode>>>; 2],
}

#[derive(Default)]
struct Trie {
    node: Rc<RefCell<TrieNode>>,
}

impl Trie {
    fn insert(&mut self, num: i32) {
        let mut node = self.node.clone();
        for i in (0..31).rev() {
            let d = ((num & (1 << i)) != 0) as usize;
            let mut inner = node.as_ref().borrow_mut();
            if inner.children[d].is_none() {
                inner.children[d] = Some(Rc::new(RefCell::new(TrieNode::default())));
            }
            let _node = inner.children[d].as_ref().unwrap().clone();
            drop(inner);
            node = _node;
        }
    }

    fn query(&self, num: i32, maxi: i32) -> i32 {
        let mut sel = 0;
        if Trie::_query(self.node.clone(), 30, num, maxi, &mut sel) {
            num ^ sel
        } else {
            -1
        }
    }

    fn _query(node: Rc<RefCell<TrieNode>>, idx: i32, num: i32, maxi: i32, sel: &mut i32) -> bool {
        if *sel > maxi {
            return false;
        }
        if idx == -1 {
            return true;
        }
        let inner = node.as_ref().borrow();
        let d = num & (1 << idx);
        let p = (d != 0) as usize;
        let q = (p ^ 1) as usize;
        if let Some(x) = inner.children[q].clone() {
            *sel ^= (q << idx) as i32;
            if Trie::_query(x, idx - 1, num, maxi, sel) {
                return true;
            }
            *sel ^= (q << idx) as i32;
        }
        if let Some(x) = inner.children[p].clone() {
            *sel ^= d;
            if Trie::_query(x, idx - 1, num, maxi, sel) {
                return true;
            }
            *sel ^= d;
        }
        false
    }
}

impl Solution {
    pub fn maximize_xor(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut trie = Trie::default();
        for num in nums {
            trie.insert(num);
        }
        let mut ans = Vec::with_capacity(queries.len());
        for query in queries {
            ans.push(trie.query(query[0], query[1]));
        }
        ans
    }
}