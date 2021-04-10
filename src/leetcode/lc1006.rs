struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
    }
}

impl Solution {
    pub fn clumsy(mut n: i32) -> i32 {
        let mut ans = 0;
        match n {
            3 => ans += n * (n - 1) / (n - 2),
            2 => ans += n * (n - 1),
            1 => ans += n,
            _ => ans += n * (n - 1) / (n - 2) + (n - 3),
        }
        n -= 4;
        while n >= 4 {
            ans -= n * (n - 1) / (n - 2) - (n - 3);
            n -= 4;
        }
        match n {
            4 => ans -= n * (n - 1) / (n - 2) - (n - 3),
            3 => ans -= n * (n - 1) / (n - 2),
            2 => ans -= n * (n - 1),
            1 => ans -= n,
            _ => {},
        }
        ans
    }
}