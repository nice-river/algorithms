#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let persons = vec![0, 1, 1, 0, 0, 1, 0];
        let times = vec![0, 5, 10, 15, 20, 25, 30];
        let qs = vec![3, 12, 25, 15, 24, 8];
        let anss = vec![0, 1, 1, 0, 0, 1];
        let t = TopVotedCandidate::new(persons, times);
        for (q, ans) in qs.into_iter().zip(anss.into_iter()) {
            assert_eq!(t.q(q), ans);
        }
    }
}

use std::collections::BTreeMap;

struct TopVotedCandidate {
    times: Vec<i32>,
    elect: Vec<i32>,
}

impl TopVotedCandidate {
    fn new(persons: Vec<i32>, times: Vec<i32>) -> Self {
        let mut map = BTreeMap::new();
        let mut elect = vec![0; persons.len()];
        let mut maxi = 0;
        let mut e = 0;
        for i in 0..persons.len() {
            let v = map.entry(persons[i]).or_insert(0);
            *v += 1;
            if *v >= maxi {
                maxi = *v;
                e = persons[i];
            }
            elect[i] = e;
        }
        Self { times, elect }
    }

    fn q(&self, t: i32) -> i32 {
        let mut l = 0;
        let mut r = self.times.len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if t < self.times[m] {
                r = m;
            } else {
                l = m;
            }
        }
        self.elect[l]
    }
}
