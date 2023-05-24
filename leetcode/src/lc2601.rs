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

impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        if nums.len() <= 1{
            return true;
        }
        let mut primes = vec![2];
        'outer: for i in (3..1000).step_by(2) {
            for j in 2..=(i as f64).sqrt() as i32 {
                if i % j == 0 {
                    continue 'outer;
                }
            }
            primes.push(i);
        }
        primes.reverse();
        for i in 0..primes.len() {
            if primes[i] < nums[0] {
                nums[0] -= primes[i];
                break;
            }
        }
        for i in 1..nums.len() {
            for j in 0..primes.len() {
                if primes[j] < nums[i] && nums[i] - primes[j] > nums[i - 1] {
                    nums[i] -= primes[j];
                    break;
                }
            }
        }
        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                return false;
            }
        }
        true
    }
}
