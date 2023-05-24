#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let n = 1000;
        let ans = 182;
        assert_eq!(Solution::punishment_number(n), ans);
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
    pub fn punishment_number(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            let s = (i * i).to_string();
            let s = s.into_bytes();
            let s = s.into_iter().map(|v| (v - b'0') as i32).collect::<Vec<_>>();
            if Self::check(&s, 0, 0, i) {
                ans += i * i;
            }
        }
        ans
    }
    fn check(digit: &Vec<i32>, idx: usize, cur_sum: i32, target: i32) -> bool {
        if idx == digit.len() {
            return cur_sum == target;
        }
        let mut t = 0;
        for i in idx..digit.len() {
            t = t * 10 + digit[i];
            if cur_sum + t > target {
                break;
            }
            if Self::check(digit, i + 1, cur_sum + t, target) {
                return true;
            }
        }
        false
    }
}
