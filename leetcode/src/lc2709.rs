#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![10007, 20014];
        assert!(Solution::can_traverse_all_pairs(nums));
    }

    #[test]
    fn test1() {
        let nums = vec![32, 1024, 6];
        assert!(Solution::can_traverse_all_pairs(nums));
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

impl Solution {
    pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        if nums.contains(&1) {
            return false;
        }
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313,
        ];
        let mut ds = DisjointSet::new(100010);
        for &num in &nums {
            let mut n = num;
            let mut a = -1;
            for &prime in &primes {
                while n % prime == 0 {
                    if a != -1 && a != prime {
                        ds.union(a, prime);
                    } else {
                        a = prime;
                    }
                    n /= prime;
                }
            }
            if a != -1 && n != 1 {
                ds.union(a, n);
            }
        }
        let mut s = -1;
        for &num in &nums {
            let mut n = num;
            for &prime in &primes {
                if n % prime == 0 {
                    let t = ds.find(prime);
                    if s != -1 && s != t {
                        return false;
                    }
                    s = t;
                    while n % prime == 0 {
                        n /= prime;
                    }
                }
            }
            if n != 1 {
                let t = ds.find(n);
                if s != -1 && s != t {
                    return false;
                }
            }
        }
        true
    }
}

struct DisjointSet {
    marks: Vec<i32>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self { marks: vec![-1; n] }
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.marks[x as usize] != -1 {
            self.marks[x as usize] = self.find(self.marks[x as usize]);
            self.marks[x as usize]
        } else {
            x
        }
    }

    fn union(&mut self, u: i32, v: i32) {
        let u = self.find(u);
        let v = self.find(v);
        if u != v {
            self.marks[v as usize] = u;
        }
    }

    fn get_disjoint_num(&mut self) -> usize {
        let mut set = std::collections::HashSet::new();
        for i in 0..self.marks.len() {
            set.insert(self.find(i as i32));
        }
        set.len()
    }
}
