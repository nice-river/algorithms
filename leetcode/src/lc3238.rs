#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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

use std::collections::HashMap;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut players = vec![HashMap::<i32, i32>::default(); n as usize];
        let mut ans = 0;
        for p in pick {
            let player = p[0] as usize;
            let color = p[1];
            *players[player].entry(color).or_insert(0) += 1;
        }
        for (i, m) in players.into_iter().enumerate() {
            if m.values().any(|x| x > &(i as i32)) {
                ans += 1;
            }
        }
        ans
    }
}
