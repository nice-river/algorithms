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

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let n = lists.len();
        Self::merge(&mut lists, 0, n)
    }

    fn merge(
        lists: &mut Vec<Option<Box<ListNode>>>,
        start: usize,
        end: usize,
    ) -> Option<Box<ListNode>> {
        if start == end {
            return None;
        }
        if start + 1 == end {
            return lists[start].take();
        }
        let mid = (start + end) / 2;
        let mut left = Self::merge(lists, start, mid);
        let mut right = Self::merge(lists, mid, end);
        let mut sentinel = Box::new(ListNode::new(-1));
        let mut p: *mut Box<ListNode> = &mut sentinel;
        unsafe {
            loop {
                match (left.take(), right.take()) {
                    (Some(mut a), Some(mut b)) => {
                        if a.val <= b.val {
                            left = a.next.take();
                            right = Some(b);
                            (*p).next = Some(a);
                        } else {
                            left = Some(a);
                            right = b.next.take();
                            (*p).next = Some(b);
                        }
                        p = (*p).next.as_mut().unwrap();
                    }
                    (Some(mut a), None) => {
                        left = a.next.take();
                        right = None;
                        (*p).next = Some(a);
                        p = (*p).next.as_mut().unwrap();
                    }
                    (None, Some(mut b)) => {
                        left = None;
                        right = b.next.take();
                        (*p).next = Some(b);
                        p = (*p).next.as_mut().unwrap();
                    }
                    (None, None) => break,
                }
            }
        }
        sentinel.next
    }
}
