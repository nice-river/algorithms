#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut ans = 0;
        for i in 0.. {
            if i < apples.len() {
                let apple = apples[i];
                let day = days[i];
                heap.push((Reverse(i + day as usize), apple));
            } else if heap.is_empty() {
                break;
            }
            while let Some((Reverse(due), cnt)) = heap.pop() {
                if due > i && cnt > 0 {
                    ans += 1;
                    heap.push((Reverse(due), cnt - 1));
                    break;
                }
            }
        }
        ans
    }
}
