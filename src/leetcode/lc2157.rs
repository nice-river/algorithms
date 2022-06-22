#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let words = vec!["a", "b", "ab", "cde"];
        let ans = vec![2, 3];
        let words = words.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(Solution::group_strings(words), ans);
    }
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_strings(words: Vec<String>) -> Vec<i32> {
        let mut map = HashMap::new();
        for word in words {
            let mut x = 0;
            for &ch in word.as_bytes() {
                x |= 1 << (ch - b'a');
            }
            *map.entry(x).or_insert(0) += 1;
        }

        let mut cnt = vec![0; map.len()];
        let mut arr = vec![0; map.len()];
        let mut idx_map = HashMap::new();

        for (i, (k, v)) in map.into_iter().enumerate() {
            cnt[i] = v;
            arr[i] = k;
            idx_map.insert(k, i);
        }
        let mut ds = DisjointSet::new(cnt.len());

        for (i, &num) in arr.iter().enumerate() {
            for p in 0..26 {
                if (num & (1 << p)) == 0 {
                    if let Some(&j) = idx_map.get(&(num | (1 << p))) {
                        ds.union(i as i32, j as i32);
                    }
                } else {
                    let x = num ^ (1 << p);
                    if let Some(&j) = idx_map.get(&x) {
                        ds.union(i as i32, j as i32);
                    }
                    for q in 0..26 {
                        if (x & (1 << q)) == 0 && (x | (1 << q)) != num {
                            if let Some(&j) = idx_map.get(&(x | (1 << q))) {
                                ds.union(i as i32, j as i32);
                            }
                        }
                    }
                }
            }
        }

        let mut map = HashMap::new();
        for i in 0..ds.marks.len() {
            let t = ds.find(i as i32);
            *map.entry(t).or_insert(0) += cnt[i];
        }
        let maxi = map.iter().map(|(k, v)| *v).max().unwrap();
        vec![ds.get_disjoint_num() as i32, maxi]
    }
}

#[derive(Debug, Clone)]
struct DisjointSet {
    marks: Vec<i32>,
    size: Vec<i32>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            marks: vec![-1; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.marks[x as usize] != -1 {
            self.marks[x as usize] = self.find(self.marks[x as usize]);
            self.marks[x as usize]
        } else {
            x
        }
    }

    fn union(&mut self, u: i32, v: i32) {
        let u = self.find(u);
        let v = self.find(v);
        if u != v {
            self.marks[v as usize] = u;
            self.size[u as usize] += self.size[v as usize];
        }
    }

    fn get_disjoint_num(&mut self) -> usize {
        let mut set = std::collections::HashSet::new();
        for i in 0..self.marks.len() {
            set.insert(self.find(i as i32));
        }
        set.len()
    }

    fn get_size(&mut self, x: i32) -> i32 {
        let x = self.find(x);
        self.size[x as usize]
    }
}
