#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let charge_times = vec![3, 6, 1, 3, 4];
        let running_costs = vec![2, 1, 3, 4, 5];
        let budget = 25;
        let ans = 3;
        assert_eq!(
            Solution::maximum_robots(charge_times, running_costs, budget),
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

use std::collections::BTreeSet;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let mut ans = 0;
        let mut p = 0;
        let mut q = 0;
        let mut maxi = BTreeSet::new();
        let mut s = 0;
        while q < charge_times.len() {
            maxi.insert((charge_times[q], q));
            s += running_costs[q] as i64;
            q += 1;
            while p < q && maxi.iter().rev().next().unwrap().0 as i64 + (q - p) as i64 * s > budget
            {
                maxi.remove(&(charge_times[p], p));
                s -= running_costs[p] as i64;
                p += 1;
            }
            ans = ans.max(q - p);
        }
        ans as i32
    }
}
