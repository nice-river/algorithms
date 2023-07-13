#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let vals = vec![2, 2, 5, 5];
        let edges = to_vec2d([[1, 0], [0, 2], [3, 2]]);
        let ans = 6;
        assert_eq!(Solution::number_of_good_paths(vals, edges), ans);
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

use std::collections::BTreeMap;
use std::convert::TryInto;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut tree = vec![vec![]; vals.len()];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            tree[u].push(v);
            tree[v].push(u);
        }
        let mut dsu = DSU::new(vals.len());
        let mut val_map = BTreeMap::new();
        for (i, &val) in vals.iter().enumerate() {
            val_map.entry(val).or_insert_with(|| vec![]).push(i);
        }
        let mut ans = 0;
        for (val, idxes) in val_map.into_iter() {
            for &idx in &idxes {
                for &adj in &tree[idx] {
                    if vals[adj] <= val {
                        dsu.merge(idx + 1, adj + 1);
                    }
                }
            }
            let mut m = BTreeMap::new();
            for &idx in &idxes {
                *m.entry(dsu.find(idx + 1)).or_insert(0) += 1;
            }
            for (_, v) in m.into_iter() {
                ans += v;
                ans += v * (v - 1) / 2;
            }
        }
        ans
    }
}

struct DSU {
    mark: Vec<usize>,
    disjointed_sets: usize,
}

impl DSU {
    fn new<T: TryInto<usize>>(n: T) -> Self {
        let n = n.try_into().unwrap_or_else(|_| unreachable!());
        Self {
            mark: vec![0; n + 1],
            disjointed_sets: n,
        }
    }

    fn find<T: TryInto<usize> + Copy>(&mut self, x: T) -> usize {
        let p = x.try_into().unwrap_or_else(|_| unreachable!());
        if self.mark[p] == 0 {
            p
        } else {
            let r = self.find(self.mark[p]);
            self.mark[p] = r.try_into().unwrap_or_else(|_| unreachable!());
            r
        }
    }

    fn merge<T: TryInto<usize> + Copy>(&mut self, a: T, b: T) {
        let u = self.find(a);
        let v = self.find(b);
        if u != v {
            self.mark[u] = v;
            self.disjointed_sets -= 1;
        }
    }

    fn set_count(&self) -> usize {
        self.disjointed_sets
    }

    fn is_same_set<T: TryInto<usize> + Copy>(&mut self, a: T, b: T) -> bool {
        self.find(a) == self.find(b)
    }
}
