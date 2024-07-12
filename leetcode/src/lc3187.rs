#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![3, 1, 4, 2, 5];
        let queries = to_vec2d([[2, 3, 4], [1, 0, 4]]);
        let ans = vec![0];
        assert_eq!(Solution::count_of_peaks(nums, queries), ans);
    }

    #[test]
    fn test1() {
        let nums = vec![4, 1, 4, 2, 1, 5];
        let queries = to_vec2d([[2, 2, 4], [1, 0, 2], [1, 0, 4]]);
        let ans = vec![0, 1];
        assert_eq!(Solution::count_of_peaks(nums, queries), ans);
    }

    #[test]
    fn test2() {
        let nums = vec![5, 4, 8, 6];
        let queries = to_vec2d([[1, 2, 2], [1, 1, 2], [2, 1, 6]]);
        let ans = vec![0, 0];
        assert_eq!(Solution::count_of_peaks(nums, queries), ans);
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

use std::ops::Add;

impl Solution {
    pub fn count_of_peaks(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tree = BinaryIndexedTree::<i32>::new(nums.len() + 1);
        for i in 1..nums.len() - 1 {
            if nums[i - 1] < nums[i] && nums[i + 1] < nums[i] {
                tree.update(i + 1, 1);
            }
        }

        let mut ans = vec![];
        for query in queries {
            if query[0] == 1 {
                let left = query[1] as usize;
                let right = query[2] as usize;
                let mut r = tree.get(right + 1) - tree.get(left);
                if left > 0
                    && left < nums.len() - 1
                    && nums[left - 1] < nums[left]
                    && nums[left + 1] < nums[left]
                {
                    r -= 1;
                }
                if right != left && right > 0
                    && right < nums.len() - 1
                    && nums[right - 1] < nums[right]
                    && nums[right + 1] < nums[right]
                {
                    r -= 1;
                }
                ans.push(r);
            } else {
                let idx = query[1] as usize;
                let val = query[2];
                let mut a = vec![];
                for i in 1.max(idx) - 1..=idx + 1 {
                    if i > 0 && i < nums.len() - 1 {
                        a.push((i, nums[i - 1] < nums[i] && nums[i + 1] < nums[i]));
                    }
                }
                nums[idx] = val;
                let mut b = vec![];
                for i in 1.max(idx) - 1..=idx + 1 {
                    if i > 0 && i < nums.len() - 1 {
                        b.push(nums[i - 1] < nums[i] && nums[i + 1] < nums[i]);
                    }
                }
                for ((i, a), b) in a.into_iter().zip(b.into_iter()) {
                    match (a, b) {
                        (true, false) => tree.update(i + 1, -1),
                        (false, true) => tree.update(i + 1, 1),
                        _ => {}
                    }
                }
            }
        }
        ans
    }
}

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
