struct Solution {}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let stones = vec![0,1,3,5,6,8,12,17];
        assert!(Solution::can_cross(stones));
	}
}

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let first_stone = *stones.first().unwrap();
        let last_stone = *stones.last().unwrap();
        let stones: HashSet<i32>= HashSet::from_iter(stones);
        let mut dp = HashSet::new();
        Solution::dfs(0, first_stone, last_stone, &stones, &mut dp)
    }

    fn dfs(k: i32, stone: i32, last_stone: i32, stones: &HashSet<i32>, dp: &mut HashSet<(i32, i32)>) -> bool {
        if stone == last_stone {
            return true;
        }
        if dp.contains(&(k, stone)) {
            return false;
        }
        for i in -1..=1 {
            if k + i > 0 {
                if stones.contains(&(stone + k + i)) {
                    let ret = Solution::dfs(k + i, stone + k + i, last_stone, stones, dp);
                    if ret {
                        return ret;
                    }
                }
            }
        }
        dp.insert((k, stone));
        false
    }
}