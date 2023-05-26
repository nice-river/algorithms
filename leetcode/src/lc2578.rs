#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num = 687;
        let ans = 75;
        assert_eq!(Solution::split_num(num), ans);
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
    pub fn split_num(num: i32) -> i32 {
        let mut digits = num.to_string().into_bytes().into_iter().map(|v| (v - b'0') as i32 ).collect::<Vec<_>>();
        digits.sort();
        let mut n = [0, 0];
        let mut w = 0;
        for digit in digits {
            n[w] = n[w] * 10 + digit;
            w ^= 1;
        }
        n[0] + n[1]
    }
}