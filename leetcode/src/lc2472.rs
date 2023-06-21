#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let ans = 2;
        let s = "oisbzwhoyzbjobrruurrbojbzyosniqhnnhqinsozfklolmxmlolkfzwxnvjvnxwxdszzsdxakidaezjrcxeexcrjwfygdlpcnjsysjncpldyvmfpiipfmvypsgqbfnfbqgsyhelmrtkktrmlhtdhllhdthjpynqwzwdwwdwzwqnyprmvaxkxavhhaobqevveqboahhsxyeeylxkfsddsfkxlyeeyyfnnvnnfgrzwgwudduwgqwbbbbwqbyumekdssdkemuypxhgmzzmghnylvtaaxamixmaftzukjjkuztfamxfujrxpkikpxrjufkvpfftivyhyaayhydagivecljyoyjlcevigasxkkdvxvdkmsjydsdmibimdsdyjsxhmljxossoxjeomkepfcibbicfpekmixzzezzxiw".to_owned();
        let k = 6;
        assert_eq!(Solution::max_palindromes(s, k), ans);
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
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut is_palindrome = vec![vec![false; n]; n];
        for l in 1..=n {
            for i in (0..n).rev() {
                let j = i + l - 1;
                if j >= n {
                    continue;
                }
                if s[i] != s[j] {
                    is_palindrome[i][j] = false;
                } else {
                    if i + 2 < j {
                        is_palindrome[i][j] = is_palindrome[i + 1][j - 1];
                    } else {
                        is_palindrome[i][j] = true;
                    }
                }
            }
        }
        let mut dp = vec![0; n + 1];
        for j in 0..n {
            dp[j + 1] = dp[j];
            for i in 0..=j {
                if is_palindrome[i][j] && j - i + 1 >= k as usize {
                    dp[j + 1] = dp[j + 1].max(dp[i] + 1);
                }
            }
        }
        dp[n]
    }
}
