#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 4;
        let ans = 3;
        assert_eq!(Solution::minimum_boxes(n), ans);
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
    pub fn minimum_boxes(n: i32) -> i32 {
        let n = n as i64;
        let mut k = 0;
        let mut s = 0;
        let mut ans = 0;
        for i in 1.. {
            k += i;
            if s + k > n {
                break;
            }
            s += k;
            ans = k;
        }
        let mut n = n - s;
        for i in 1.. {
            if n <= 0 {
                break;
            }
            ans += 1;
            n -= i;
        }
        ans as i32
    }
}
