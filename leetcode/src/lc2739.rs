#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = 5;
        let b = 10;
        assert_eq!(Solution::distance_traveled(a, b), 60);
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
    pub fn distance_traveled(mut main_tank: i32, mut additional_tank: i32) -> i32 {
        let mut ans = 0;
        while main_tank >= 5 {
            let c = main_tank / 5;
            ans += c * 5 * 10;
            main_tank %= 5;
            main_tank += additional_tank.min(c);
            additional_tank -= additional_tank.min(c);
        }
        ans + main_tank * 10
    }
}
