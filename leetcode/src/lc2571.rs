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

struct Solution {}

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut que = VecDeque::new();
        let mut vis = HashSet::new();
        vis.insert(n);
        que.push_back((n, 0));
        while let Some((n, s)) = que.pop_front() {
            if n == 0 {
                return s;
            }
            for i in 0.. {
                if (1 << i) <= n {
                    let k = n - (1 << i);
                    if !vis.contains(&k) {
                        que.push_back((k, s + 1));
                        vis.insert(k);
                    }
                    if (1 << i) < n {
                        let k = n + (1 << i);
                        if !vis.contains(&k) {
                            que.push_back((k, s + 1));
                            vis.insert(k);
                        }
                    }
                } else {
                    break;
                }
            }
        }
        unreachable!()
    }
}
