#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let instructions = vec![1, 5, 6, 2];
        let ans = 1;
        assert_eq!(Solution::create_sorted_array(instructions), ans);
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

impl Solution {
    pub fn create_sorted_array(instructions: Vec<i32>) -> i32 {
        const MAX: usize = 100005;
        let mut bitree = BinaryIndexedTree::new(MAX);
        let module = 1000000007;
        let mut ans = 0;
        for instruction in instructions {
            let ins = instruction as usize + 1;
            let large: i32 = bitree.get(MAX - 1) - bitree.get(ins);
            let small: i32 = bitree.get(ins - 1);
            ans += large.min(small);
            ans %= module;
            bitree.update(ins, 1);
        }
        ans
    }
}

use std::ops::Add;

struct BinaryIndexedTree<T> {
    arr: Vec<T>,
}

impl<T: Add<Output = T> + Default + Copy> BinaryIndexedTree<T> {
    fn new(sz: usize) -> Self {
        Self {
            arr: vec![T::default(); sz],
        }
    }

    fn update(&mut self, mut idx: usize, val: T) {
        while idx < self.arr.len() {
            self.arr[idx] = self.arr[idx] + val;
            idx += (!idx + 1) & idx;
        }
    }

    fn get(&self, mut idx: usize) -> T {
        assert!(idx < self.arr.len());
        let mut res = T::default();
        while idx > 0 {
            res = res + self.arr[idx];
            idx -= (!idx + 1) & idx;
        }
        res
    }
}
