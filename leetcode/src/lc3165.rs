#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3, 5, 9];
        let queries = to_vec2d([[1, -2], [0, -3]]);
        let ans = 21;
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![0, -1];
        let queries = to_vec2d([[0, -5]]);
        let ans = 0;
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![6];
        let queries = to_vec2d([[0, -2]]);
        let ans = 0;
        assert_eq!(Solution::maximum_sum_subsequence(nums, queries), ans);
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

use crate::*;

struct Solution {}

#[derive(Default, Debug, Copy, Clone)]
struct Node {
    left_bound: usize,
    right_bound: usize,
    vals: [[i64; 2]; 2],
}

struct SegmentTree {
    nodes: Vec<Node>,
}

impl SegmentTree {
    fn new(data: &[i32]) -> Self {
        let n = data.len();
        let nodes = vec![Node::default(); n * 4];
        let mut tree = Self { nodes };
        tree.init(0, 0, n, data);
        tree
    }

    fn init(&mut self, root: usize, lb: usize, rb: usize, data: &[i32]) {
        self.nodes[root].left_bound = lb;
        self.nodes[root].right_bound = rb;
        if lb + 1 == rb {
            self.nodes[root].vals[1][1] = data[lb].max(0) as i64;
            return;
        }
        let left_child = root * 2 + 1;
        let right_child = root * 2 + 2;
        let mid = (lb + rb) / 2;
        self.init(left_child, lb, mid, data);
        self.init(right_child, mid, rb, data);
        self.nodes[root].vals[0][0] = (self.nodes[left_child].vals[0][1]
            + self.nodes[right_child].vals[0][0])
            .max(self.nodes[left_child].vals[0][0] + self.nodes[right_child].vals[1][0]);
        self.nodes[root].vals[0][1] = (self.nodes[left_child].vals[0][1]
            + self.nodes[right_child].vals[0][1])
            .max(self.nodes[left_child].vals[0][0] + self.nodes[right_child].vals[1][1]);
        self.nodes[root].vals[1][0] = (self.nodes[left_child].vals[1][0]
            + self.nodes[right_child].vals[1][0])
            .max(self.nodes[left_child].vals[1][1] + self.nodes[right_child].vals[0][0]);
        self.nodes[root].vals[1][1] = (self.nodes[left_child].vals[1][1]
            + self.nodes[right_child].vals[0][1])
            .max(self.nodes[left_child].vals[1][0] + self.nodes[right_child].vals[1][1]);
    }

    fn update(&mut self, query: &[i32]) -> i64 {
        self.inner_update(0, query[0] as usize, query[1] as i64);
        self.nodes[0].vals[1][1]
    }

    fn inner_update(&mut self, root: usize, pos: usize, val: i64) {
        let lb = self.nodes[root].left_bound;
        let rb = self.nodes[root].right_bound;
        if lb > pos || rb <= pos {
            return;
        }
        if lb + 1 == rb {
            self.nodes[root].vals[1][1] = val.max(0);
            return;
        }
        let left_child = root * 2 + 1;
        let right_child = root * 2 + 2;
        let mid = (lb + rb) / 2;
        self.inner_update(left_child, pos, val);
        self.inner_update(right_child, pos, val);
        self.nodes[root].vals[0][0] = (self.nodes[left_child].vals[0][1]
            + self.nodes[right_child].vals[0][0])
            .max(self.nodes[left_child].vals[0][0] + self.nodes[right_child].vals[1][0]);
        self.nodes[root].vals[0][1] = (self.nodes[left_child].vals[0][1]
            + self.nodes[right_child].vals[0][1])
            .max(self.nodes[left_child].vals[0][0] + self.nodes[right_child].vals[1][1]);
        self.nodes[root].vals[1][0] = (self.nodes[left_child].vals[1][0]
            + self.nodes[right_child].vals[1][0])
            .max(self.nodes[left_child].vals[1][1] + self.nodes[right_child].vals[0][0]);
        self.nodes[root].vals[1][1] = (self.nodes[left_child].vals[1][1]
            + self.nodes[right_child].vals[0][1])
            .max(self.nodes[left_child].vals[1][0] + self.nodes[right_child].vals[1][1]);
    }
}

impl Solution {
    pub fn maximum_sum_subsequence(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut ans = 0i64;
        let mut tree = SegmentTree::new(&nums);
        for query in queries {
            ans += tree.update(&query);
            ans %= MOD;
        }
        ans as i32
    }
}
