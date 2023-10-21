#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = vec![2, 4, 9];
        let b = vec![1, 6, 13];
        let ans = 1;
        assert_eq!(Solution::stone_game_vi(a, b), ans);
    }

    #[test]
    fn test1() {
        let a = vec![10, 1];
        let b = vec![1, 20];
        let ans = 0;
        assert_eq!(Solution::stone_game_vi(a, b), ans);
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

use std::collections::BTreeSet;

impl Solution {
    pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
        let sum = vec![
            alice_values.iter().sum::<i32>(),
            bob_values.iter().sum::<i32>(),
        ];
        let values = vec![&alice_values, &bob_values];
        let mut set = vec![BTreeSet::new(), BTreeSet::new()];
        for i in 0..alice_values.len() {
            set[0].insert((alice_values[i] - sum[1] + bob_values[i], i));
            set[1].insert((bob_values[i] - sum[0] + alice_values[i], i));
        }
        let mut score = vec![0, 0];
        for i in 0..alice_values.len() {
            let w = i % 2;
            let &(v, p) = set[w].iter().rev().next().unwrap();
            set[w].remove(&(v, p));
            score[w] += values[w][p];
            set[w ^ 1].remove(&(values[w ^ 1][p] - sum[w] + values[w][p], p));
        }
        if score[0] > score[1] {
            return 1;
        } else if score[0] == score[1] {
            return 0;
        } else {
            return -1;
        }
    }
}
