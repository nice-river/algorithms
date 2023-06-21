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
    pub fn di_string_match(s: String) -> Vec<i32> {
        let s = s.chars().collect::<Vec<_>>();
        let mut ans = Vec::with_capacity(s.len());
        let mut mini = 0;
        let mut maxi = 0;
        ans.push(0);
        for c in s {
            if c == 'I' {
                ans.push(maxi + 1);
                maxi += 1;
            } else {
                ans.push(mini - 1);
                mini -= 1;
            }
        }
        for num in ans.iter_mut() {
            *num -= mini;
        }
        ans
    }
}
