#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
        let mut x = vec![vec![1], vec![1], vec![1], ];
        assert_eq!(Solution::least_bricks(x), 3);
	}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        for wall in wall.iter() {
            let mut axis = 0;
            for i in 0..wall.len()-1 {
                axis += wall[i];
                *map.entry(axis).or_insert(0) += 1;
            }
        }
        let maxi = map.into_iter().map(|(k, v)| v).max().unwrap_or(0);
        wall.len() as i32 - maxi
    }
}