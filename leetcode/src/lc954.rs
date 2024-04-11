#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr = vec![-2, -4, 2, 4, 1, 2];
        assert_eq!(Solution::can_reorder_doubled(arr), true);
    }

    #[test]
    fn test1() {
        let arr = vec![2, 1, 2, 6];
        assert_eq!(Solution::can_reorder_doubled(arr), false);
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

use std::collections::HashMap;

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        let mut pos = vec![];
        let mut neg = vec![];
        let mut counter = HashMap::new();
        for v in arr {
            *counter.entry(v).or_insert(0) += 1;
            if v >= 0 {
                pos.push(v);
            } else {
                neg.push(v);
            }
        }
        if pos.len() % 2 != 0 || neg.len() % 2 != 0 {
            return false;
        }
        pos.sort();
        neg.sort();
        neg.reverse();
        for v in pos {
            if let Some(t) = counter.get(&v) {
                if *t > 0 {
                    *counter.get_mut(&v).unwrap() -= 1;
                    if let Some(g) = counter.get(&(v * 2)) {
                        if *g == 0 {
                            return false;
                        }
                        *counter.get_mut(&(v * 2)).unwrap() -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        for v in neg {
            if let Some(t) = counter.get(&v) {
                if *t > 0 {
                    *counter.get_mut(&v).unwrap() -= 1;
                    if let Some(g) = counter.get(&(v * 2)) {
                        if *g == 0 {
                            return false;
                        }
                        *counter.get_mut(&(v * 2)).unwrap() -= 1;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
