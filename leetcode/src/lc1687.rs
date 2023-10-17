#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let boxes = to_vec2d([
            [2, 4],
            [2, 5],
            [3, 1],
            [3, 2],
            [3, 7],
            [3, 1],
            [4, 4],
            [1, 3],
            [5, 2],
        ]);
        let ports_count = 5;
        let max_boxes = 5;
        let max_weight = 7;
        let ans = 14;
        assert_eq!(
            Solution::box_delivering(boxes, ports_count, max_boxes, max_weight),
            ans
        );
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

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn box_delivering(
        boxes: Vec<Vec<i32>>,
        ports_count: i32,
        max_boxes: i32,
        max_weight: i32,
    ) -> i32 {
        let n = boxes.len();
        let mut diffs = vec![0; n];
        for i in 1..n {
            diffs[i] = diffs[i - 1];
            if boxes[i][0] != boxes[i - 1][0] {
                diffs[i] += 1;
            }
        }
        let mut sums = vec![0; n + 1];
        for i in 0..n {
            sums[i + 1] = sums[i] + boxes[i][1];
        }
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, 0)));
        for i in 0..n {
            while let Some(Reverse((cost, p))) = heap.pop() {
                if i + 1 - p > max_boxes as usize || sums[i + 1] - sums[p] > max_weight {
                    continue;
                }
                heap.push(Reverse((cost, p)));
                let c = cost + 2 + diffs[i] - diffs[0];
                if i == n - 1 {
                    return c;
                }
                heap.push(Reverse((
                    c - (diffs[i] - diffs[0]) - (diffs[i + 1] - diffs[i]),
                    i + 1,
                )));
                break;
            }
        }
        unreachable!()
    }
}
