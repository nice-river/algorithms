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
    pub fn longest_string(mut x: i32, mut y: i32, mut z: i32) -> i32 {
        let mut ans = x.min(y).min(z - 1);
        x -= ans;
        y -= ans;
        z -= ans;
        ans *= 3;
        ans += z;
        let c = x.min(y);
        ans += c * 2;
        x -= c;
        y -= c;
        if x >= 1 {
            ans += 1;
        } else if y >= 1 {
            ans += 1;
        }
        ans * 2
    }
}
