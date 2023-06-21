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
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;
        for mut num in nums {
            a += num;
            while num != 0 {
                b += num % 10;
                num /= 10;
            }
        }
        (a - b).abs()
    }
}
