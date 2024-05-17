#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let difficulty = vec![68, 35, 52, 47, 86];
        let profit = vec![67, 17, 1, 81, 3];
        let worker = vec![92, 10, 85, 84, 82];
        let ans = 324;
        assert_eq!(
            Solution::max_profit_assignment(difficulty, profit, worker),
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

use crate::*;

struct Solution {}

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pairs: Vec<(i32, i32)> = profit.into_iter().zip(difficulty).collect::<Vec<_>>();
        pairs.sort();
        let mut jobs: Vec<(i32, i32)> = vec![];
        for (profit, difficulty) in pairs {
            while !jobs.is_empty() && jobs.last().unwrap().1 >= difficulty {
                jobs.pop();
            }
            jobs.push((profit, difficulty));
        }
        for w in worker {
            let mut l = 0;
            let mut r = jobs.len();
            while l < r {
                let m = (l + r) / 2;
                if jobs[m].1 > w {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            if r > 0 {
                ans += jobs[r - 1].0;
            }
        }
        ans
    }
}
