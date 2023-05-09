#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let m = 100;
        let n = 100;
        let mut sol = Solution::new(m, n);
        let mut map = HashMap::new();
        for _ in 0..m * n {
            map.insert(sol.flip(), ());
        }
        assert_eq!(map.len() as i32, m * n);
    }
}

use rand::Rng;
use std::collections::HashMap;

struct Solution {
    m: i32,
    n: i32,
    cnt: i32,
    occupied: HashMap<i32, i32>,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Self {
            m,
            n,
            cnt: m * n,
            occupied: HashMap::new(),
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let x = rand::thread_rng().gen_range(0, self.cnt);
        self.cnt -= 1;
        let &idx = self.occupied.get(&x).unwrap_or(&x);
        let &y = self.occupied.get(&self.cnt).unwrap_or(&self.cnt);
        self.occupied.insert(x, y);
        vec![idx / self.n, idx % self.n]
    }

    fn reset(&mut self) {
        self.cnt = self.m * self.n;
        self.occupied.clear();
    }
}
