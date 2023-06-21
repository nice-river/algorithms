#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let inventory = vec![2, 5];
        let orders = 4;
        let ans = 14;
        assert_eq!(Solution::max_profit(inventory, orders), ans);
    }

    #[test]
    fn test1() {
        let inventory = vec![497978859, 167261111, 483575207, 591815159];
        let orders = 836556809;
        let ans = 373219333;
        assert_eq!(Solution::max_profit(inventory, orders), ans);
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
    pub fn max_profit(mut inventory: Vec<i32>, orders: i32) -> i32 {
        let module = 1000000007;
        let mut ans = 0i64;
        inventory.sort_unstable();
        let inventory = inventory
            .into_iter()
            .map(|e| e as i64)
            .rev()
            .collect::<Vec<_>>();
        let mut l = 0;
        let mut r = inventory[0];
        while l <= r {
            let m = (l + r) / 2;
            let mut t = 0;
            let mut k = orders as i64;
            let mut c = 0;
            for i in 0..inventory.len() {
                if inventory[i] < m || k == 0 {
                    break;
                }
                c += 1;
                let u = (inventory[i] - (m + 1) + 1).min(k);
                t += (inventory[i] + inventory[i] - u + 1) * u / 2;
                t %= module;
                k -= u;
            }
            if k == 0 {
                ans = t;
                l = m + 1;
            } else if c >= k {
                t += m * k;
                t %= module;
                ans = t;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        ans as i32
    }
}
