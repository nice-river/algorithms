#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let blocked = vec![vec![0, 1], vec![1, 0]];
        let source = vec![0, 0];
        let target = vec![2, 2];
        assert_eq!(Solution::is_escape_possible(blocked, source, target), false);
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

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for block in blocked {
            set.insert((block[0], block[1]));
        }
        let source = (source[0], source[1]);
        let target = (target[0], target[1]);
        Self::check(source, target, &set) && Self::check(target, source, &set)
    }

    fn check(source: (i32, i32), target: (i32, i32), blocked: &HashSet<(i32, i32)>) -> bool {
        let mut visited = HashSet::new();
        visited.insert(source);
        let mut que = VecDeque::new();
        que.push_back((source, 0));
        while let Some((p, step)) = que.pop_front() {
            if step >= blocked.len() || p == target {
                return true;
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = p.0 + dx;
                let ny = p.1 + dy;
                if nx < 0 || ny < 0 || nx >= 1_000_000 || ny >= 1_000_000 {
                    continue;
                }
                if visited.contains(&(nx, ny)) || blocked.contains(&(nx, ny)) {
                    continue;
                }
                visited.insert((nx, ny));
                que.push_back(((nx, ny), step + 1));
            }
        }
        false
    }
}
