#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num = 5;
        let ans = 2;
        assert_eq!(Solution::find_complement(num), ans);
    }

    #[test]
    fn test1() {
        let num = 4;
        let ans = 3;
        assert_eq!(Solution::find_complement(num), ans);
    }

    #[test]
    fn test2() {
        let num = i32::MAX;
        let ans = 0;
        assert_eq!(Solution::find_complement(num), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        ((num as u64 + 1).next_power_of_two() - 1) as i32 ^ num
    }
}
