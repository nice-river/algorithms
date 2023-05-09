#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr = vec![1, 13, 17, 59];
        let k = 6;
        let ans = vec![13, 17];
        assert_eq!(Solution::kth_smallest_prime_fraction(arr, k), ans);
    }

    #[test]
    fn test1() {
        let a = Node {
            x: 3,
            y: 2,
            p: 0,
            q: 0,
        };
        let b = Node {
            x: 2,
            y: 3,
            p: 0,
            q: 0,
        };
        assert!(a > b);
        assert!(b < a);
    }
}

struct Solution {}

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        for i in 1..arr.len() {
            heap.push(Reverse(Node {
                x: arr[0],
                y: arr[i],
                p: 0,
                q: i,
            }));
        }
        for _ in 1..k {
            let Reverse(Node { x, y, p, q }) = heap.pop().unwrap();
            if p + 1 < q {
                heap.push(Reverse(Node {
                    x: arr[p + 1],
                    y,
                    p: p + 1,
                    q,
                }))
            }
        }
        heap.pop()
            .map(|Reverse(Node { x, y, p, q })| vec![x, y])
            .unwrap()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord)]
struct Node {
    x: i32,
    y: i32,
    p: usize,
    q: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let p = self.x * other.y;
        let q = self.y * other.x;
        if p == q {
            Some(Ordering::Equal)
        } else if p < q {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}
