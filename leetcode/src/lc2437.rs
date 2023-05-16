#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut ans = 0;
        'outer: for i in 0..2400 {
            let s = format!("{:04}", i);
            let mut s = s.into_bytes();
            s.insert(2, b':');
            for (&a, &b) in time.as_bytes().iter().zip(s.iter()) {
                if a != b'?' && a != b {
                    continue 'outer;
                }
            }
            let m = std::str::from_utf8(&s[3..])
                .unwrap()
                .parse::<i32>()
                .unwrap();
            if m < 60 {
                ans += 1
            }
        }
        ans
    }
}
