#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 5;
        let queries = to_vec2d([[1, 4], [2, 4]]);
        let ans = vec![2, 2];
        assert_eq!(Solution::shortest_distance_after_queries(n, queries), ans);
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

use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut map = BTreeMap::new();
        let mut remap = HashMap::new();
        for i in 1..n {
            map.insert(i - 1, i);
            remap.insert(i, i - 1);
        }
        let mut step = n - 1;
        for query in queries {
            let x = query[0];
            let y = query[1];
            if let Some(&f) = remap.get(&y) {
                if f > x {
                    let mut remove_idx = vec![];
                    for (a, b) in map.range(x..f + 1) {
                        remap.remove(b);
                        remove_idx.push(*a);
                        step -= 1;
                    }
                    for idx in remove_idx {
                        map.remove(&idx);
                    }
                    map.insert(x, y);
                    remap.insert(y, x);
                    step += 1;
                }
            }
            ans.push(step);
        }
        ans
    }
}
