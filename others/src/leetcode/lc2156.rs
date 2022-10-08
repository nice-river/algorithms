#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "leetcode".to_string();
        let power = 7;
        let module = 20;
        let k = 2;
        let hash_value = 0;
        let ans = "ee".to_string();
        assert_eq!(Solution::sub_str_hash(s, power, module, k, hash_value), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn sub_str_hash(s: String, power: i32, modulo: i32, k: i32, hash_value: i32) -> String {
        let s = s.as_bytes();
        let mut idx = 0;
        let k = k as usize;
        let power = power as i64;
        let modulo = modulo as i64;

        let mut h = 0;
        let mut x = 1;

        for i in 0..k {
            let p = s[s.len() - (k - i)] - b'a' + 1;
            h = (h + p as i64 * x) % modulo;
            if i < k - 1 {
                x = x * power % modulo;
            }
        }

        if h as i32 == hash_value {
            idx = s.len() - k;
        }

        for i in (0..s.len() - k).rev() {
            let p = s[i + k] - b'a' + 1;
            h = ((h - (p as i64 * x) % modulo) + modulo) % modulo;
            h = (h * power) % modulo;
            let p = s[i] - b'a' + 1;
            h = (h + p as i64) % modulo;
            if h as i32 == hash_value {
                idx = i;
            }
        }

        std::str::from_utf8(&s[idx..idx + k]).unwrap().to_string()
    }
}
