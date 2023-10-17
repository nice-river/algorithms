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
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();
        let n = satisfaction.len();
        let mut s = vec![0; n + 1];
        for i in 1..=n {
            s[i] = s[i - 1] + satisfaction[i - 1];
        }
        let mut ans = 0;
        for i in 0..n {
            ans += satisfaction[i] * (i as i32 + 1);
        }
        let mut c = 0;
        for i in 0..n {
            let k = satisfaction[i] * (i as i32 + 1 - c);
            let t = s[n] - s[i + 1];
            if ans - k - t > ans {
                ans = ans - k - t;
                c += 1;
            }
        }
        ans
    }
}
