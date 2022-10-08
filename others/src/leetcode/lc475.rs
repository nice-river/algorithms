struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let houses = vec![1, 2, 3];
        let heaters = vec![2];
        let ans = 1;
        assert_eq!(Solution::find_radius(houses, heaters), ans);
    }
}

impl Solution {
    pub fn find_radius(mut houses: Vec<i32>, mut heaters: Vec<i32>) -> i32 {
        houses.sort_unstable();
        heaters.sort_unstable();
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        for house in houses {
            while i + 1 < heaters.len() && heaters[i + 1] <= house {
                i += 1;
            }
            while heaters[j] < house && j + 1 < heaters.len() {
                j += 1;
            }
            ans = ans.max((house - heaters[i]).abs().min((heaters[j] - house).abs()));
        }
        ans
    }
}
