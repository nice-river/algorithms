#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let s = "owoztneoer";
        let ans = "012";
        assert_eq!(Solution::original_digits(s.to_string()), ans);
    }

    #[test]
    fn test1() {
        let s = "fievfuro";
        let ans = "45";
        assert_eq!(Solution::original_digits(s.to_string()), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut char_cnt = vec![0; 256];
        for &ch in s.as_bytes() {
            char_cnt[ch as usize] += 1;
        }

        let mut number_cnt = vec![0; 10];
        number_cnt[6] = char_cnt[b'x' as usize];
        number_cnt[0] = char_cnt[b'z' as usize];
        number_cnt[2] = char_cnt[b'w' as usize];
        number_cnt[7] = char_cnt[b's' as usize] - number_cnt[6];
        number_cnt[5] = char_cnt[b'v' as usize] - number_cnt[7];
        number_cnt[4] = char_cnt[b'f' as usize] - number_cnt[5];
        number_cnt[3] = char_cnt[b'r' as usize] - number_cnt[0] - number_cnt[4];
        number_cnt[1] = char_cnt[b'o' as usize] - number_cnt[0] - number_cnt[2] - number_cnt[4];
        number_cnt[8] = char_cnt[b'g' as usize];
        number_cnt[9] = char_cnt[b'i' as usize] - number_cnt[8] - number_cnt[5] - number_cnt[6];

        number_cnt
            .into_iter()
            .enumerate()
            .map(|(num, cnt)| vec![num.to_string(); cnt as usize].join(""))
            .collect::<Vec<String>>()
            .join("")
    }
}
