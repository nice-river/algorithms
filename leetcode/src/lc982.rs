#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let nums = vec![2, 1, 3];
        let ans = 12;
        assert_eq!(Solution::count_triplets(nums), ans);
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

#[derive(Clone)]
struct Bitset {
    bit_arr: Vec<i64>,
}

impl Bitset {
    fn new(n: usize) -> Self {
        Self {
            bit_arr: vec![0; (n + 63) / 64],
        }
    }

    fn join(&mut self, other: &Bitset) {
        for (a, b) in self.bit_arr.iter_mut().zip(other.bit_arr.iter()) {
            (*a) |= (*a) | (*b);
        }
    }

    fn cnt(&self) -> i32 {
        let mut ret = 0;
        for a in self.bit_arr.iter() {
            ret += a.count_ones();
        }
        ret as i32
    }

    fn set_bit(&mut self, bit: usize) {
        self.bit_arr[bit / 64] |= 1 << (bit % 64);
    }

    fn clear(&mut self) {
        self.bit_arr.iter_mut().for_each(|x| *x = 0);
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        const N: i32 = 1 << 16;
        let n = nums.len();
        let mut map = HashMap::new();
        let mut sets = vec![Bitset::new(n); 16];
        for i in 0..n {
            for j in 0..n {
                *map.entry(nums[i] & nums[j]).or_insert(0) += 1;
            }
            for j in 0..16 {
                if (nums[i] & (1 << j)) != 0 {
                    sets[j].set_bit(i);
                }
            }
        }
        let mut ans = 0;
        let mut s = Bitset::new(n);
        for (val, cnt) in map.into_iter() {
            s.clear();
            for j in 0..16 {
                if (val & (1 << j)) != 0 {
                    s.join(&sets[j]);
                }
            }
            ans += cnt * s.cnt();
        }
        n.pow(3) as i32 - ans
    }
}
