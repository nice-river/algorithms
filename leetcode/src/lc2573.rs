#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let lcp = vec![[4, 0, 2, 0], [0, 3, 0, 1], [2, 0, 2, 0], [0, 1, 0, 1]];
        let lcp = lcp.into_iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(Solution::find_the_string(lcp), "abab".to_owned());
    }
}

struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        if n == 0 || lcp[0].len() != n {
            return "".to_owned();
        }
        let mut ans = vec![b' '; n];
        let mut c = 0;
        for i in 0..n {
            if ans[i] == b' ' {
                if c >= 26 {
                    return "".to_owned();
                }
                ans[i] = b'a' + c;
                c += 1;
            }
            for j in i + 1..n {
                if lcp[i][j] > 0 {
                    ans[j] = ans[i];
                }
            }
        }
        for i in (0..n).rev() {
            if lcp[i][i] != (n - i) as i32 {
                return "".to_owned();
            }
            for j in i + 1..n {
                if lcp[i][j] != lcp[j][i] {
                    return "".to_owned();
                }
                if lcp[i][j] == 0 {
                    if ans[i] == ans[j] {
                        return "".to_owned();
                    }
                } else {
                    if i + 1 < n && j + 1 < n && lcp[i][j] != lcp[i + 1][j + 1] + 1 {
                        return "".to_owned();
                    }
                    let l = lcp[i][j] as usize;
                    if i + l > n || j + l > n {
                        return "".to_owned();
                    }
                    if ans[i] != ans[j] {
                        return "".to_owned();
                    }
                }
            }
        }

        String::from_utf8(ans).unwrap()
    }
}
