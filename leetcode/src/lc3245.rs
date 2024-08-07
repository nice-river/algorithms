#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let colors = vec![0, 1, 1, 0, 1];
        let queries = vec![vec![2, 1, 0], vec![1, 4]];
        let ans = vec![2];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), ans);
    }

    #[test]
    fn test1() {
        let colors = vec![0, 0, 1, 0, 1, 1];
        let queries = vec![vec![1, 3], vec![2, 3, 0], vec![1, 5]];
        let ans = vec![2, 0];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), ans);
    }

    #[test]
    fn test2() {
        let colors = vec![0, 1, 0, 0, 0, 1];
        let queries = vec![vec![1, 5], vec![2, 5, 0], vec![1, 2]];
        let ans = vec![1, 2];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), ans);
    }

    #[test]
    fn test3() {
        let colors = vec![0, 0, 0, 1];
        let queries = vec![vec![2, 1, 1], vec![1, 3], vec![2, 1, 1], vec![2, 0, 1]];
        let ans = vec![4];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), ans);
    }

    #[test]
    fn test4() {
        let colors = vec![1, 0, 0, 1];
        let queries = vec![vec![2, 2, 0], vec![2, 2, 1], vec![2, 2, 0], vec![2, 3, 0]];
        let ans = vec![];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), ans);
    }

    #[test]
    fn test5() {
        let colors = vec![1, 0, 1, 0, 1, 0, 1, 1, 0];
        let queries = vec![
            vec![2, 6, 0],
            vec![2, 0, 0],
            vec![2, 6, 0],
            vec![2, 0, 1],
            vec![2, 4, 0],
            vec![2, 3, 1],
            vec![1, 6],
            vec![2, 4, 0],
            vec![1, 3],
            vec![1, 4],
        ];
        let ans = vec![1, 4, 3];
        assert_eq!(Solution::number_of_alternating_groups(colors, queries), ans);
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

use std::collections::{BTreeMap, HashMap};

impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = colors.len();

        let mut ans = vec![];
        let mut a2b = HashMap::new();
        let mut b2a = BTreeMap::new();
        let mut lens = BTreeMap::new();

        let mut i = 0;
        while i < colors.len() {
            let mut j = i + 1;
            while j < colors.len() && colors[j] != colors[j - 1] {
                j += 1;
            }
            a2b.insert(i, j - 1);
            b2a.insert(j - 1, i);
            *lens.entry(j - i).or_insert(0) += 1;
            i = j;
        }

        for query in queries {
            if query[0] == 1 {
                let sz = query[1] as usize;
                let mut x = 0;
                let mut y = 0;
                let mut res = 0;
                if colors[n - 1] != colors[0] {
                    x = a2b.get(&0).unwrap() + 1;
                    y = n - *b2a.get(&(n - 1)).unwrap();
                }
                if x == n {
                    res += n;
                } else {
                    if x + y >= sz {
                        res += x + y + 1 - sz;
                    }
                    for (&l, &c) in lens.range(sz..) {
                        let c = match (l == x, l == y) {
                            (true, true) => c - 2,
                            (false, false) => c,
                            _ => c - 1,
                        };
                        res += (l + 1 - sz) * c;
                    }
                }
                ans.push(res as i32);
            } else {
                let pos = query[1] as usize;
                let color = query[2];
                if color == colors[pos] {
                    continue;
                }
                colors[pos] = color;
                if pos == n - 1 {
                    if color == colors[pos - 1] {
                        let a = b2a.remove(&pos).unwrap();
                        let t = lens.get_mut(&(pos - a + 1)).unwrap();
                        *t -= 1;
                        if *t == 0 {
                            lens.remove(&(pos - a + 1));
                        }
                        a2b.remove(&a);
                        a2b.insert(a, pos - 1);
                        b2a.insert(pos - 1, a);
                        a2b.insert(pos, pos);
                        b2a.insert(pos, pos);
                        *lens.entry(1).or_insert(0) += 1;
                        *lens.entry(pos - a).or_insert(0) += 1;
                    } else {
                        let a = b2a.remove(&(pos - 1)).unwrap();
                        let t = lens.get_mut(&(pos - a)).unwrap();
                        *t -= 1;
                        if *t == 0 {
                            lens.remove(&(pos - a));
                        }
                        let t = lens.get_mut(&1).unwrap();
                        *t -= 1;
                        if *t == 0 {
                            lens.remove(&1);
                        }
                        a2b.insert(a, pos);
                        b2a.remove(&(pos - 1));
                        b2a.insert(pos, a);
                        *lens.entry(pos - a + 1).or_insert(0) += 1;
                    }
                } else if pos == 0 {
                    if color == colors[pos + 1] {
                        let b = a2b.remove(&pos).unwrap();
                        let t = lens.get_mut(&(b - pos + 1)).unwrap();
                        *t -= 1;
                        if *t == 0 {
                            lens.remove(&(b - pos + 1));
                        }
                        a2b.insert(pos + 1, b);
                        b2a.insert(b, pos + 1);
                        a2b.insert(pos, pos);
                        b2a.insert(pos, pos);
                        *lens.entry(1).or_insert(0) += 1;
                        *lens.entry(b - pos).or_insert(0) += 1;
                    } else {
                        let b = a2b.remove(&(pos + 1)).unwrap();
                        let t = lens.get_mut(&(b - pos)).unwrap();
                        *t -= 1;
                        if *t == 0 {
                            lens.remove(&(b - pos));
                        }
                        let t = lens.get_mut(&1).unwrap();
                        *t -= 1;
                        if *t == 0 {
                            lens.remove(&1);
                        }
                        a2b.insert(pos, b);
                        b2a.remove(&pos);
                        b2a.insert(b, pos);
                        *lens.entry(b - pos + 1).or_insert(0) += 1;
                    }
                } else {
                    match (colors[pos - 1] == color, color == colors[pos + 1]) {
                        (true, true) => {
                            let (&b, &a) = b2a.range(pos..).next().unwrap();
                            b2a.remove(&b);
                            a2b.remove(&a);
                            let t = lens.get_mut(&(b - a + 1)).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&(b - a + 1));
                            }
                            if pos > a {
                                a2b.insert(a, pos - 1);
                                b2a.insert(pos - 1, a);
                                *lens.entry(pos - a).or_insert(0) += 1;
                            }
                            if pos < b {
                                a2b.insert(pos + 1, b);
                                b2a.insert(b, pos + 1);
                                *lens.entry(b - pos).or_insert(0) += 1;
                            }
                            a2b.insert(pos, pos);
                            b2a.insert(pos, pos);
                            *lens.entry(1).or_insert(0) += 1;
                        }
                        (false, false) => {
                            let a = b2a.remove(&(pos - 1)).unwrap();
                            a2b.remove(&a);
                            let t = lens.get_mut(&(pos - a)).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&(pos - a));
                            }

                            let b = a2b.remove(&(pos + 1)).unwrap();
                            b2a.remove(&b);
                            let t = lens.get_mut(&(b - pos)).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&(b - pos));
                            }
                            let t = lens.get_mut(&1).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&1);
                            }
                            a2b.remove(&pos);
                            b2a.remove(&pos);

                            a2b.insert(a, b);
                            b2a.insert(b, a);
                            *lens.entry(b - a + 1).or_insert(0) += 1;
                        }
                        (true, false) => {
                            // before 0 1 1, after 0 0 1
                            let a = b2a.remove(&pos).unwrap();
                            a2b.remove(&a);
                            let t = lens.get_mut(&(pos - a + 1)).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&(pos - a + 1));
                            }
                            a2b.insert(a, pos - 1);
                            b2a.insert(pos - 1, a);
                            *lens.entry(pos - a).or_insert(0) += 1;

                            let b = a2b.remove(&(pos + 1)).unwrap();
                            b2a.remove(&b);
                            let t = lens.get_mut(&(b - pos)).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&(b - pos));
                            }
                            a2b.insert(pos, b);
                            b2a.insert(b, pos);
                            *lens.entry(b - pos + 1).or_insert(0) += 1;
                        }
                        (false, true) => {
                            // before 1 1 0, after 1 0 0
                            let b = a2b.remove(&pos).unwrap();
                            b2a.remove(&b);
                            let t = lens.get_mut(&(b - pos + 1)).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&(b - pos + 1));
                            }
                            a2b.insert(pos + 1, b);
                            b2a.insert(b, pos + 1);
                            *lens.entry(b - pos).or_insert(0) += 1;

                            let a = b2a.remove(&(pos - 1)).unwrap();
                            a2b.remove(&a);
                            let t = lens.get_mut(&(pos - a)).unwrap();
                            *t -= 1;
                            if *t == 0 {
                                lens.remove(&(pos - a));
                            }
                            a2b.insert(a, pos);
                            b2a.insert(pos, a);
                            *lens.entry(pos - a + 1).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        ans
    }
}
