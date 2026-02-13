#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 5;
        let nums = vec![1, 8, 3, 4, 2];
        let max_diff = 3;
        let queries = to_vec!([[0, 3], [2, 4]]);
        let ans = Solution::path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(ans, vec![1, 1]);
    }

    #[test]
    fn test1() {
        let n = 5;
        let nums = vec![5, 3, 1, 9, 10];
        let max_diff = 2;
        let queries = to_vec!([[0, 1], [0, 2], [2, 3], [4, 3]]);
        let ans = Solution::path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(ans, vec![1, 2, -1, 1]);
    }

    #[test]
    fn test2() {
        let n = 3;
        let nums = vec![3, 6, 1];
        let max_diff = 1;
        let queries = to_vec!([[0, 0], [0, 1], [1, 2]]);
        let ans = Solution::path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(ans, vec![0, -1, -1]);
    }

    #[test]
    fn test3() {
        let n = 2;
        let nums = vec![15, 15];
        let max_diff = 18;
        let queries = to_vec!([[0, 0], [1, 1], [1, 0]]);
        let ans = Solution::path_existence_queries(n, nums, max_diff, queries);
        assert_eq!(ans, vec![0, 0, 1]);
    }
}

use crate::*;

struct Solution {}

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            map.entry(num).or_insert(vec![]).push(i as i32);
        }
        let mut arr = map.keys().copied().collect::<Vec<i32>>();
        arr.sort();
        let mut query_map = HashMap::new();
        for (i, query) in queries.iter().enumerate() {
            let a = query[0];
            let b = query[1];
            let a_num = nums[a as usize];
            let b_num = nums[b as usize];
            if a_num <= b_num {
                query_map.entry(a).or_insert_with(|| vec![]).push((b, i));
            } else {
                query_map.entry(b).or_insert_with(|| vec![]).push((a, i));
            }
        }
        let mut ans = vec![0; queries.len()];
        let mut dp = VecDeque::<(i32, Vec<i32>)>::new();
        for &a in arr.iter().rev() {
            while !dp.is_empty() && dp.back().unwrap().0 > a + max_diff {
                dp.pop_back();
            }
            if dp.is_empty() {
                dp.push_back((a, vec![a]));
            } else if dp.back().unwrap().0 == a + max_diff {
                let (_, mut c) = dp.pop_back().unwrap();
                c.push(a);
                dp.push_front((a, c));
            } else {
                let mut c = dp.back().unwrap().1.clone();
                c.push(a);
                dp.push_front((a, c));
            }

            if let Some(indices) = map.get(&a) {
                for &idx in indices {
                    if let Some(queries) = query_map.get(&idx) {
                        for &(b, query_idx) in queries {
                            if let Some(p) = Self::find(&dp.front().unwrap().1, nums[b as usize]) {
                                if p != 0 || idx == b {
                                    ans[query_idx] = p;
                                } else {
                                    ans[query_idx] = 1;
                                }
                            } else {
                                ans[query_idx] = -1;
                            }
                        }
                    }
                }
            }
        }
        ans
    }

    fn find(arr: &[i32], target: i32) -> Option<i32> {
        if target > arr[0] {
            return None;
        }
        let mut left = 0;
        let mut right = arr.len() - 1;
        let mut t = arr.len();
        let n = arr.len() as i32;
        while left <= right {
            let mid = (left + right) / 2;
            if arr[mid] == target {
                return Some(n - mid as i32 - 1);
            } else if arr[mid] < target {
                t = mid - 1;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        Some(n - t as i32 - 1)
    }
}
