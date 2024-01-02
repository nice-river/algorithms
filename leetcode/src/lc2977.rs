#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_trivial() {
        let source = "ab".to_owned();
        let target = "bc".to_owned();
        let original = vec!["a", "b"].into_iter().map(|s| s.to_owned()).collect();
        let changed = vec!["b", "c"].into_iter().map(|s| s.to_owned()).collect();
        let cost = vec![1, 2];
        let ans = 3;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            ans
        );
    }

    #[test]
    fn test0() {
        let source = "abcdefgh".to_owned();
        let target = "acdeeghh".to_owned();
        let original = vec!["bcd", "fgh", "thh"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let changed = vec!["cde", "thh", "ghh"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let cost = vec![1, 3, 5];
        let ans = 9;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            ans
        );
    }

    #[test]
    fn test3() {
        let source = "ssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnsscln".to_owned();
        let target = "mivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcg".to_owned();
        let original = vec!["ss","cl","nss","clns","scl","nss","cl","nss","cl","nss","cl","ns","scl","ns","sc","lns","sc","ln","ss","cl","ns","sc","lns","sc","lns","sc","ln","ssc","ln","ss","cl","ns","sc","ln","sscl","ns","scln","ss","cl","nss","cl","ns","scl","ns","sc","ln","ss","cl","ns","sc","ln","ssc","ln","ssc","lns","sc","ln","ss","cln","ss","cl","nss","cl","ns","sc","lnss","cln","ssc","ln","ssc","ln","ss","cl","ns","scl","nssc","ln","sscl","nss","cln","ss","clns","sc","lnss","cl","ns","scl","nss","cl","nss","cl","ns","scl","nss","cln","ss","cl","ns","scl","ssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnssclnsscln"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let changed = vec!["mi","vc","gmi","vcgm","ivc","gmi","vc","gmi","vc","gmi","vc","gm","ivc","gm","iv","cgm","iv","cg","mi","vc","gm","iv","cgm","iv","cgm","iv","cg","miv","cg","mi","vc","gm","iv","cg","mivc","gm","ivcg","mi","vc","gmi","vc","gm","ivc","gm","iv","cg","mi","vc","gm","iv","cg","miv","cg","miv","cgm","iv","cg","mi","vcg","mi","vc","gmi","vc","gm","iv","cgmi","vcg","miv","cg","miv","cg","mi","vc","gm","ivc","gmiv","cg","mivc","gmi","vcg","mi","vcgm","iv","cgmi","vc","gm","ivc","gmi","vc","gmi","vc","gm","ivc","gmi","vcg","mi","vc","gm","ivc","mivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcgmivcg"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let cost = vec![
            718, 164, 678, 933, 981, 324, 344, 758, 145, 123, 38, 821, 382, 35, 931, 239, 268, 762,
            638, 679, 661, 177, 232, 172, 847, 942, 795, 459, 938, 790, 662, 523, 713, 5, 392, 329,
            324, 362, 785, 346, 417, 483, 101, 485, 255, 617, 880, 138, 879, 507, 333, 444, 913,
            428, 70, 393, 101, 209, 200, 326, 971, 250, 643, 335, 377, 880, 323, 465, 26, 243, 892,
            554, 642, 565, 277, 831, 827, 947, 954, 369, 959, 887, 485, 687, 996, 927, 753, 587,
            182, 317, 659, 214, 561, 487, 632, 543, 967, 985, 273, 94747,
        ];
        let ans = 27387;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            ans
        );
    }

    #[test]
    fn test1() {
        let source = "asmlmoumomvummakmlbabvmvvavlavtsvbvssuumsllttsusts".to_owned();
        let target = "asmlmoumomvummakmlbabvmvvaolklllvbsltuomkslvmmusts".to_owned();
        let original = vec![
            "vlavtsvbvssuumslltts",
            "mtmubkasuvumkobbmsmo",
            "vsbbvauauvuvsauaastl",
            "uovumoluksslvkvlkmam",
            "smsvsuubusskmublvvst",
            "momuatbkubosmmavvssk",
            "mommltotttbtvlvalsbt",
            "vbbbuvslutblvvkvtmmo",
            "boaosasttvtvtabtubab",
            "mbtkumblvbasoobaauvm",
            "vbotklaoltambktlulot",
            "vluamsokkbaalsmmalav",
            "mttmbuosbmlttabmbabl",
            "sskvkbmlabaulluomovt",
            "lstbuomkaoatmavsmvml",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
        let changed = vec![
            "mtmubkasuvumkobbmsmo",
            "vsbbvauauvuvsauaastl",
            "uovumoluksslvkvlkmam",
            "smsvsuubusskmublvvst",
            "momuatbkubosmmavvssk",
            "mommltotttbtvlvalsbt",
            "vbbbuvslutblvvkvtmmo",
            "boaosasttvtvtabtubab",
            "mbtkumblvbasoobaauvm",
            "vbotklaoltambktlulot",
            "vluamsokkbaalsmmalav",
            "mttmbuosbmlttabmbabl",
            "sskvkbmlabaulluomovt",
            "lstbuomkaoatmavsmvml",
            "olklllvbsltuomkslvmm",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
        let cost = vec![
            9758, 7133, 9355, 8885, 6055, 7360, 9168, 9288, 7422, 6995, 8167, 6154, 6939, 6343,
            9733,
        ];
        let ans = 118755;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            ans
        );
    }

    #[test]
    fn test2() {
        let source =
"addbacbddcbbccaaccdccabbdbdadcbdbbcabcdbbcccbdbdacbcbdabbdaaacbbcaaaaabaaadbcdcbdbaddddadacabcaddbcd".to_owned();
        let target =
"cadbacbddcbbccaaccbcdcbddbaddadaabacbccbbbaaccbdacbcbdcdacdbccbbcaaaaaabdbdccccbdbadddbadacabcaddbda".to_owned();
        let original = vec![
            "ad",
            "ab",
            "dc",
            "bd",
            "ac",
            "dcca",
            "cdcb",
            "dadb",
            "bbdc",
            "bbda",
            "cd",
            "ad",
            "bb",
            "baaadbcd",
            "cbabadda",
            "dabbcdca",
            "cbabacbc",
            "bbbabbbd",
            "cccbd",
            "badac",
            "bacac",
            "bdaaa",
            "bdbdadcbdbbcabcd",
            "caadbbdcdbcbabdb",
            "badddcbccccaaabb",
            "bcabcbcdbaacbdca",
            "cddbaacbbbacbddd",
            "d",
            "dabbdaaa",
            "abbcadcb",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
        let changed = vec![
            "ab",
            "dc",
            "bd",
            "ac",
            "ca",
            "cdcb",
            "dadb",
            "bbdc",
            "bbda",
            "bcdc",
            "ad",
            "bb",
            "da",
            "cbabadda",
            "dabbcdca",
            "cbabacbc",
            "bbbabbbd",
            "abdbdccc",
            "badac",
            "bacac",
            "bdaaa",
            "baacc",
            "caadbbdcdbcbabdb",
            "badddcbccccaaabb",
            "bcabcbcdbaacbdca",
            "cddbaacbbbacbddd",
            "ddbaddadaabacbcc",
            "b",
            "abbcadcb",
            "dcdacdbc",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
        let cost = vec![
            40, 73, 64, 56, 16, 52, 84, 34, 49, 29, 97, 99, 47, 53, 54, 74, 84, 96, 98, 94, 95, 74,
            92, 61, 53, 62, 79, 93, 89, 85,
        ];
        let ans = 2076;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            ans
        );
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

use crate::*;

struct Solution {}

use std::collections::{HashMap, HashSet};

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        let source = source.as_bytes();
        let target = target.as_bytes();
        let mut idx = HashMap::new();

        // let mut debug_map = HashMap::new();

        fn hash(s: &[u8]) -> i64 {
            let mut r = 0;
            for b in s {
                r = (r * 27 + (b - b'a' + 1) as i64) % MOD;
            }
            r
        }

        for s in &original {
            let h = hash(s.as_bytes());
            if !idx.contains_key(&h) {
                let v = idx.len();
                idx.insert(h, v);

                // debug_map.insert(h, s.clone());
            }
            // if let Some(x) = debug_map.get(&h) {
            //     if x != s {
            //         eprintln!("wrong: {}, {}", x, s);
            //     }
            // }
        }
        for s in &changed {
            let h = hash(s.as_bytes());
            if !idx.contains_key(&h) {
                let v = idx.len();
                idx.insert(h, v);
                // debug_map.insert(h, s.clone());
            }
            // if let Some(x) = debug_map.get(&h) {
            //     if x != s {
            //         eprintln!("wrong: {}, {}", x, s);
            //     }
            // }
        }
        const BLOCK: i64 = i64::MAX / 8;
        let n = idx.len();
        let mut dist = vec![vec![BLOCK; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for i in 0..original.len() {
            let ha = hash(original[i].as_bytes());
            let hb = hash(changed[i].as_bytes());
            let &a = idx.get(&ha).unwrap();
            let &b = idx.get(&hb).unwrap();
            dist[a][b] = dist[a][b].min(cost[i] as i64);
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        // let n = source.len();
        // let mut same = vec![vec![false; source.len() + 1]; source.len() + 1];
        // for i in 0..=n {
        //     same[i][i] = true;
        // }
        // for i in (0..n).rev() {
        //     for j in i + 1..=n {
        //         same[i][j] = source[i] == target[i] && same[i + 1][j];
        //     }
        // }

        // dbg!(&same);
        // for (k, v) in idx.iter() {
        //     dbg!((k, v));
        // }
        // dbg!(&dist);

        let szs = original
            .iter()
            .map(|x| x.as_bytes().len())
            .collect::<HashSet<_>>();

        let hs = Self::construct_hashes(source, &szs);
        let ht = Self::construct_hashes(target, &szs);

        let mut dp = vec![BLOCK; source.len() + 1];
        dp[source.len()] = 0;

        for i in (0..source.len()).rev() {
            for &x in &szs {
                if let Some(&s) = hs[i].get(&x) {
                    let &t = ht[i].get(&x).unwrap();
                    if let Some(&a) = idx.get(&s) {
                        if let Some(&b) = idx.get(&t) {
                            dp[i] = dp[i].min(dist[a][b] + dp[i + x]);
                        }
                    }
                }
                if source[i] == target[i] {
                    dp[i] = dp[i].min(dp[i + 1]);
                }
            }

            // let mut hs = (source[i] - b'a' + 1) as i64;
            // let mut ht = (target[i] - b'a' + 1) as i64;
            // for j in i + 1..=source.len() {
            //     if let Some(&a) = idx.get(&hs) {
            //         if let Some(&b) = idx.get(&ht) {
            //             dp[i] = dp[i].min(dist[a][b] + dp[j]);
            //         }
            //     }
            //     if same[i][j] {
            //         dp[i] = dp[i].min(dp[j]);
            //     }
            //     if j < source.len() {
            //         hs = (hs * 27 + (source[j] - b'a' + 1) as i64) % MOD;
            //         ht = (ht * 27 + (target[j] - b'a' + 1) as i64) % MOD;
            //     }
            // }
        }

        if dp[0] != BLOCK {
            dp[0]
        } else {
            -1
        }
    }

    fn construct_hashes(s: &[u8], szs: &HashSet<usize>) -> Vec<HashMap<usize, i64>> {
        let mut ret = Vec::with_capacity(s.len());
        let mut hash_arr = vec![0; s.len() + 1];
        let mut rev_mod = vec![0; s.len() + 1];
        let mut x = 1;
        rev_mod[s.len()] = 1;
        for i in (0..s.len()).rev() {
            hash_arr[i] = ((s[i] - b'a' + 1) as i64 * x % MOD + hash_arr[i + 1]) % MOD;
            x = x * 27 % MOD;
            rev_mod[i] = x;
        }
        let (a, b) = Self::egcd(rev_mod[0], MOD);
        rev_mod[0] = (a % MOD + MOD) % MOD;
        for i in 1..s.len() {
            rev_mod[i] = (rev_mod[i - 1] * 27) % MOD;
        }

        for i in 0..s.len() {
            ret.push(HashMap::new());
            for &n in szs.iter() {
                if i + n > s.len() || ret[i].contains_key(&n) {
                    continue;
                }
                let mut h = ((hash_arr[i] - hash_arr[i + n]) + MOD) % MOD;
                h *= rev_mod[i + n];
                h %= MOD;
                ret[i].insert(n, h);
            }
        }
        ret
    }

    fn egcd(a: i64, b: i64) -> (i64, i64) {
        if b == 0 {
            (1, 0)
        } else {
            let (x, y) = Self::egcd(b, a % b);
            (y, x - a / b * y)
        }
    }
}
