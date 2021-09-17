#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut dists = vec![HashMap::new(); n];
        for i in 0..n {
            for j in i + 1..n {
                let x = points[i][0] - points[j][0];
                let y = points[i][1] - points[j][1];
                let d = x * x + y * y;
                *dists[i].entry(d).or_insert(0) += 1;
                *dists[j].entry(d).or_insert(0) += 1;
            }
        }
        let mut ans = 0;
        for map in dists {
            for (_, c) in map.into_iter() {
                ans += c * (c - 1);
            }
        }
        ans
    }
}
