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

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn highest_ranked_k_items(
        grid: Vec<Vec<i32>>,
        pricing: Vec<i32>,
        start: Vec<i32>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = vec![];
        let mut pq = BinaryHeap::new();
        let row = start[0] as usize;
        let col = start[1] as usize;
        let mut vis = vec![vec![false; m]; n];
        pq.push(Reverse(Node::new(0, grid[row][col], row, col)));
        vis[row][col] = true;
        while let Some(Reverse(node)) = pq.pop() {
            if node.price >= pricing[0] && node.price <= pricing[1] {
                ans.push(vec![node.row as i32, node.col as i32]);
                if ans.len() >= k as usize {
                    break;
                }
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
                let nx = node.row as i32 + dx;
                let ny = node.col as i32 + dy;
                if nx < 0 || ny < 0 || nx as usize >= n || ny as usize >= m {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if vis[nx][ny] || grid[nx][ny] == 0 {
                    continue;
                }
                vis[nx][ny] = true;
                pq.push(Reverse(Node::new(node.step + 1, grid[nx][ny], nx, ny)));
            }
        }
        ans
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Node {
    step: i32,
    price: i32,
    row: usize,
    col: usize,
}

impl Node {
    fn new(step: i32, price: i32, row: usize, col: usize) -> Self {
        Self {
            step,
            price,
            row,
            col,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.step.partial_cmp(&other.step) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.price.partial_cmp(&other.price) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.row.partial_cmp(&other.row) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.col.partial_cmp(&other.col)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
