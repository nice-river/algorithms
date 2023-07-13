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

use std::collections::HashMap;

impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut timeline = vec![];
        for log in logs {
            let i = log[0];
            let t = log[1];
            timeline.push((t, -1, i as usize));
            timeline.push((t + x, 1, i as usize));
        }
        let mut ans = vec![0; queries.len()];
        for (i, query) in queries.into_iter().enumerate() {
            timeline.push((query, 0, i));
        }
        let mut map = HashMap::new();
        timeline.sort();
        for (_, p, i) in timeline {
            if p == -1 {
                *map.entry(i).or_insert(0) += 1;
            } else if p == 0 {
                ans[i] = n - map.len() as i32;
            } else {
                let ent = map.get_mut(&i).unwrap();
                *ent -= 1;
                if *ent == 0 {
                    map.remove(&i);
                }
            }
        }
        ans
    }
}
