#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let arr = vec![10, 9, 1, 10, 5];
        let k = 3;
        let ans = 14;
        assert_eq!(Solution::make_sub_k_sum_equal(arr, k), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let g = Self::gcd(arr.len(), k as usize);
        let mut ans = 0;
        let mut same = Vec::with_capacity(arr.len());
        for i in 0..g {
            same.clear();
            for j in 0.. {
                let t = i + j * g;
                if t >= arr.len() {
                    break;
                }
                same.push(arr[t]);
            }
            ans += Self::mini_op(&mut same);
        }
        ans
    }

    fn mini_op(arr: &mut Vec<i32>) -> i64 {
        if arr.len() == 0 {
            return 0;
        }
        arr.sort();
        let base = arr[(arr.len() - 1) / 2];
        arr.iter().map(|v| (*v - base).abs() as i64).sum::<i64>()
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}
