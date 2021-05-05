use std::ops::MulAssign;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
	}
}

struct Solution {}

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let mut dp = vec![vec![vec![-2; target as usize + 1]; n as usize + 1]; cost.len()];
        let mut ans = i32::MAX;
        for i in 1..=n {
            let ret = Solution::dfs(cost.len() - 1, i, target, &houses, &cost, n, &mut dp);
            if ret != -1 {
                ans = ans.min(ret);
            }
        }
        if ans == i32::MAX { -1 } else { ans }
    }

	fn dfs(idx: usize, cur_n: i32, target: i32, houses: &Vec<i32>, cost: &Vec<Vec<i32>>, n: i32, dp: &mut Vec<Vec<Vec<i32>>>) -> i32 {
        if target <= 0 {
            return -1;
        }
        if idx == 0 {
            if target != 1 {
                return -1;
            }
            if houses[idx] != 0 {
                return if houses[idx] == cur_n { 0 } else { -1 };
            } else {
                return cost[idx][cur_n as usize - 1];
            }
        }
        if dp[idx][cur_n as usize][target as usize] != -2 {
            return dp[idx][cur_n as usize][target as usize];
        }

		let mut ret = i32::MAX;
        if houses[idx] != 0 {
            if houses[idx] != cur_n {
	            return -1;
            }
            for i in 1..=n {
	            if i != cur_n {
                    let res = Solution::dfs(idx - 1, i, target - 1, houses, cost, n, dp);
                    if res != -1 {
                        ret = ret.min(res);
                    }
                }
            }
            let res = Solution::dfs(idx - 1, cur_n, target, houses, cost, n, dp);
            if res != -1 {
                ret = ret.min(res);
            }
        } else {
            for i in 1..=n {
                if i != cur_n {
                    let res = Solution::dfs(idx - 1, i, target - 1, houses, cost, n, dp);
                    if res != -1 {
                        ret = ret.min(res + cost[idx][cur_n as usize - 1]);
                    }
                }
            }
            let res = Solution::dfs(idx - 1, cur_n, target, houses, cost, n, dp);
            if res != -1 {
                ret = ret.min(res + cost[idx][cur_n as usize - 1]);
            }
        }
        if ret == i32::MAX {
            ret = -1;
        }
        dp[idx][cur_n as usize][target as usize] = ret;
        ret
	}
}