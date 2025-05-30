#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let buildings = to_vec!([
            [2, 4],
            [1, 2],
            [3, 1],
            [1, 4],
            [2, 3],
            [3, 3],
            [2, 2],
            [1, 3]
        ]);
        let ans = 1;
        assert_eq!(Solution::count_covered_buildings(n, buildings), ans);
    }
}

use crate::*;

struct Solution {}

use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut rows = HashMap::<i32, BTreeSet<i32>>::new();
        let mut cols = HashMap::<i32, BTreeSet<i32>>::new();
        for building in &buildings {
            let x = building[0];
            let y = building[1];
            rows.entry(x).or_default().insert(y);
            cols.entry(y).or_default().insert(x);
        }
        let mut ans = 0;
        for building in &buildings {
            let x = building[0];
            let y = building[1];
            let ys = rows.get(&x).unwrap();
            if ys.range(..y).next().is_none() || ys.range(y + 1..).next().is_none() {
                continue; // no other building covers this one
            }
            let xs = cols.get(&y).unwrap();
            if xs.range(..x).next().is_none() || xs.range(x + 1..).next().is_none() {
                continue; // no other building covers this one
            }
            ans += 1; // this building is covered by at least one other building
        }
        ans
    }
}
