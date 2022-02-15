use std::arch::x86_64::__readeflags;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ages = vec![16, 16];
        let ans = 2;
        assert_eq!(Solution::num_friend_requests(ages), ans);
    }

    #[test]
    fn test1() {
        let ages = vec![1, 7];
        let ans = 0;
        assert_eq!(Solution::num_friend_requests(ages), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn num_friend_requests(mut ages: Vec<i32>) -> i32 {
        let mut ans = 0;
        ages.sort_unstable();
        let mut i = 0;
        let mut k = 0;
        for j in 0..ages.len() {
            k = k.max(j);
            while i < j && ages[i] <= ages[j] / 2 + 7 {
                i += 1;
            }
            while k + 1 < ages.len() && ages[k + 1] == ages[j] && ages[j] > ages[k + 1] / 2 + 7 {
                k += 1;
            }
            ans += (k - i) as i32;
        }
        ans
    }
}
