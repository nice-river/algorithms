#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut sr = SummaryRanges::new();
        assert_eq!(sr.get_intervals(), Vec::<Vec<i32>>::new());
        sr.add_num(3);
        assert_eq!(sr.get_intervals(), vec![vec![3, 3]]);
        sr.add_num(1);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
        sr.add_num(2);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3]]);
        sr.add_num(5);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![5, 5]]);
        sr.add_num(3);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![5, 5]]);
        sr.add_num(5);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![5, 5]]);
        sr.add_num(4);
        assert_eq!(sr.get_intervals(), vec![vec![1, 5]]);
        sr.add_num(10);
        assert_eq!(sr.get_intervals(), vec![vec![1, 5], vec![10, 10]]);
    }
}

struct Solution {}

use std::collections::BTreeMap;

struct SummaryRanges {
    map: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn add_num(&mut self, val: i32) {
        let le = self.map.range(..=val);
        let (a, mut b) = if let Some((&p, &q)) = le.rev().next() {
            if val <= p + q - 1 {
                return;
            }
            if p + q == val {
                (p, q + 1)
            } else {
                (val, 1)
            }
        } else {
            (val, 1)
        };
        if let Some(&k) = self.map.get(&(val + 1)) {
            b += k;
            self.map.remove(&(val + 1));
        }
        *self.map.entry(a).or_insert(0) = b;
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        let mut ret = Vec::with_capacity(self.map.len());
        for (&k, &v) in self.map.iter() {
            ret.push(vec![k, k + v - 1]);
        }
        ret
    }
}
