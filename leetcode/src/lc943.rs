#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let words = vec!["catg", "ctaagt", "gcta", "ttca", "atgcatc"];
        let words = words.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        let ans = "gctaagttcatgcatc".to_owned();
        assert_eq!(Solution::shortest_superstring(words), ans);
    }

    #[test]
    fn test1() {
        let words = vec!["sssv","svq","dskss","sksss"];
        let words = words.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>();
        let ans = "dsksssvq".to_owned();
        assert_eq!(Solution::shortest_superstring(words).len(), ans.len());
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
    pub fn shortest_superstring(words: Vec<String>) -> String {
        let words = words.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let n = words.len();
        let mut concat = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                for k in (1..=words[i].len().min(words[j].len())).rev() {
                    if words[i][words[i].len() - k..words[i].len()] == words[j][0..k] {
                        concat[i][j] = k;
                        break;
                    }
                }
            }
        }
        let mut dp = vec![vec![(usize::MAX, vec![]); n]; 1 << n];
        for i in 0..n {
            dp[1 << i][i] = (words[i].len(), vec![i]);
        }
        for i in 0..1 << n {
            for k in 0..n {
                if ((1 << k) & i) != 0 {
                    continue;
                }
                for j in 0..n {
                    if ((1 << j) & i) == 0 {
                        continue;
                    }
                    let t = dp[i][j].0 + words[k].len() - concat[j][k];
                    if t < dp[i | (1 << k)][k].0 {
                        dp[i | (1 << k)][k].0 = t;
                        dp[i | (1 << k)][k].1 = dp[i][j].1.clone();
                        dp[i | (1 << k)][k].1.push(k);
                    }
                }
            }
        }
        let mut mini = usize::MAX;
        let mut seq = &vec![];
        for (x, arr) in dp.last().unwrap() {
            if *x < mini {
                mini = *x;
                seq = arr;
            }
        }
        let mut ans = words[seq[0]].to_vec();
        for i in 1..seq.len() {
            let x = &words[seq[i]][concat[seq[i - 1]][seq[i]]..];
            ans.extend_from_slice(x);
        }
        String::from_utf8(ans).unwrap()
    }
}
