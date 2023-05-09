#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 1, 3];
        let ans = 5;
        assert_eq!(Solution::count_operations_to_empty_array(nums), ans);
        let nums = vec![3, 1, 2];
        let ans = 4;
        assert_eq!(Solution::count_operations_to_empty_array(nums), ans);
    }

    #[test]
    fn test1() {
        use rand::seq::SliceRandom;

        let n = 1000;
        let t = 100;

        let mut random = rand::thread_rng();
        for _ in 0..t {
            let mut arr = (1..=n).collect::<Vec<_>>();
            arr.shuffle(&mut random);
            let nums = arr.clone();
            let mut ans = 0;
            let mut mini = 1;
            while !arr.is_empty() {
                let x = arr.remove(0);
                ans += 1;
                if x != mini {
                    arr.push(x);
                } else {
                    mini += 1;
                }
            }
            assert_eq!(Solution::count_operations_to_empty_array(nums.clone()), ans);
        }
    }
}

struct Solution {}

impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        if nums.is_empty() {
            return 0;
        }
        let mut tree = BinaryIndexedTree::<i64>::new(nums.len() + 1);
        for i in 0..nums.len() {
            tree.update(i + 1, 1);
        }
        let mut nums = nums
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<Vec<_>>();
        nums.sort_unstable();

        let mut ans = 0;
        let mut prev = 0;
        for &(_, cur) in &nums {
            ans += 1;
            if prev < cur {
                ans += tree.get(cur) - tree.get(prev);
            } else if prev > cur {
                ans += tree.get(nums.len()) - tree.get(prev) + tree.get(cur) - tree.get(0);
            }
            tree.update(cur + 1, -1);
            prev = (cur + 1) % nums.len();
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
