struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in (1..arr.len() + 1).step_by(2) {
            for j in 0..arr.len() - i + 1 {
                ans += arr[j..j + i].iter().sum();
            }
        }
        ans
    }
}
