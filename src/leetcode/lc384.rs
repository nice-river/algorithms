#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

use rand::Rng;

struct Solution {
    nums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut ret = vec![0; self.nums.len()];
        for i in 0..self.nums.len() {
            let j = rand::thread_rng().gen_range(0, i + 1);
            if i != j {
                ret[i] = ret[j];
            }
            ret[j] = self.nums[i];
        }
        ret
    }
}
