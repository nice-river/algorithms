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
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut zeros = 0;
        let mut neg = 0;
        for num in nums {
            if num == 0 {
                zeros += 1;
            } else if num < 0 {
                neg += 1;
            }
        }
        if zeros > 0 {
            0
        } else if neg % 2 == 0 {
            1
        } else {
            -1
        }
    }
}