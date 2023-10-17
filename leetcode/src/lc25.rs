#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut a = Box::new(ListNode::new(1));
        let b = Box::new(ListNode::new(2));
        a.next = Some(b);
        Solution::reverse_k_group(Some(a), 2);
    }

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

use crate::*;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut sentinel = Box::new(ListNode::new(-1));
        let mut p: *mut Box<ListNode> = &mut sentinel as *mut _;
        let mut t = k;
        unsafe {
            while let Some(mut node) = head.take() {
                head = node.next.take();
                node.next = (*p).next.take();
                (*p).next = Some(node);
                t -= 1;
                if t == 0 {
                    while let Some(nxt) = (*p).next.as_mut() {
                        p = nxt as *mut _;
                    }
                    t = k;
                }
            }
            if t != k {
                let mut t = None;
                while let Some(mut node) = (*p).next.take() {
                    (*p).next = node.next.take();
                    node.next = t;
                    t = Some(node);
                }
                (*p).next = t;
            }
        }
        sentinel.next
    }
}
