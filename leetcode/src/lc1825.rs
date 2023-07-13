#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let mut avg = MKAverage::new(3, 1);
        avg.add_element(3);
        avg.add_element(1);
        assert_eq!(avg.calculate_mk_average(), -1);
        avg.add_element(10);
        assert_eq!(avg.calculate_mk_average(), 3);
        avg.add_element(5);
        avg.add_element(5);
        avg.add_element(5);
        assert_eq!(avg.calculate_mk_average(), 5);
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

use std::collections::BTreeSet;
use std::collections::VecDeque;

struct MKAverage {
    sum: i64,
    m: usize,
    k: usize,
    p: usize,
    min_sum: i64,
    max_sum: i64,
    que: VecDeque<(i64, usize)>,
    min_set: BTreeSet<(i64, usize)>,
    mid_set: BTreeSet<(i64, usize)>,
    max_set: BTreeSet<(i64, usize)>,
}

impl MKAverage {
    fn new(m: i32, k: i32) -> Self {
        Self {
            sum: 0,
            m: m as usize,
            k: k as usize,
            min_sum: 0,
            max_sum: 0,
            que: VecDeque::new(),
            p: 0,
            min_set: BTreeSet::new(),
            mid_set: BTreeSet::new(),
            max_set: BTreeSet::new(),
        }
    }

    fn add_element(&mut self, num: i32) {
        let num = num as i64;
        self.sum += num;
        self.que.push_back((num, self.p));
        self.min_sum += num;
        self.min_set.insert((num, self.p));
        if self.min_set.len() > self.k {
            let large = self.min_set.iter().rev().next().unwrap().clone();
            self.min_sum -= large.0;
            self.min_set.remove(&large);
            self.mid_set.insert(large);

            if self.mid_set.len() > 1 {
                let large = self.mid_set.iter().rev().next().unwrap().clone();
                self.mid_set.remove(&large);
                self.max_sum += large.0;
                self.max_set.insert(large);
                if self.max_set.len() > self.k {
                    let small = self.max_set.iter().next().unwrap().clone();
                    self.max_sum -= small.0;
                    self.max_set.remove(&small);
                    self.mid_set.insert(small);
                }
            }
        }

        if self.que.len() > self.m {
            let q = self.que.pop_front().unwrap();
            self.sum -= q.0;
            if !self.mid_set.remove(&q) {
                if self.min_set.remove(&q) {
                    self.min_sum -= q.0;
                    let small = self.mid_set.iter().next().unwrap().clone();
                    self.min_sum += small.0;
                    self.mid_set.remove(&small);
                    self.min_set.insert(small);
                } else {
                    self.max_sum -= q.0;
                    self.max_set.remove(&q);
                    let large = self.mid_set.iter().rev().next().unwrap().clone();
                    self.max_sum += large.0;
                    self.mid_set.remove(&large);
                    self.max_set.insert(large);
                }
            }
        }
        self.p += 1;
    }

    fn calculate_mk_average(&self) -> i32 {
        if self.que.len() < self.m {
            return -1;
        }
        let sum = self.sum - self.min_sum - self.max_sum;
        let k = self.m - 2 * self.k;
        (sum / k as i64) as i32
    }
}
