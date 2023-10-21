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

const MODULE: i64 = 10i64.pow(9) + 7;

impl Solution {
    pub fn max_nice_divisors(prime_factors: i32) -> i32 {
        let k = prime_factors / 3;
        let r = prime_factors % 3;
        let mut ans = 0;
        match r {
            0 => { Self::fast_pow(3, k as i64) as i32 }
            1 => {
                if k > 0 {
                    (Self::fast_pow(3, k as i64 - 1) * 4 % MODULE) as i32
                } else {
                    1
                }
            }
            2 => {
                if k > 0 {
                    (Self::fast_pow(3, k as i64) * 2 % MODULE) as i32
                } else {
                    2
                }
            }
            _ => unreachable!()
        }
    }

    fn fast_pow(mut base: i64, mut exp: i64) -> i64 {
        let mut ret = 1;
        while exp != 0 {
            if (exp & 1) != 0 {
                ret = ret * base % MODULE;
            }
            base = base * base % MODULE;
            exp >>= 1;
        }
        ret
    }
}