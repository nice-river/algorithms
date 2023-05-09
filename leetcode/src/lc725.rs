#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use crate::leetcode::leetcode::ListNode;

impl Solution {
    pub fn split_list_to_parts(
        mut head: Option<Box<ListNode>>,
        k: i32,
    ) -> Vec<Option<Box<ListNode>>> {
        let mut ans = vec![];
        let mut nodes = vec![];

        while let Some(mut node) = head.take() {
            head = node.next.take();
            nodes.push(node)
        }
        let n = nodes.len();
        let k = k as usize;
        if n == 0 {
            return vec![None; k];
        }

        let p = n / k;
        let q = n % k;

        while !nodes.is_empty() {
            let mut head = None;
            for _ in 0..(p + if ans.len() >= k - q { 1 } else { 0 }) {
                let mut node = nodes.pop().unwrap();
                node.next = head;
                head = Some(node);
            }
            ans.push(head);
        }

        ans.reverse();
        ans
    }
}
