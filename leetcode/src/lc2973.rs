#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let edges = to_vec2d([[0,1],[0,2]]);
        let cost = vec![1,2,-2];
        let ans = vec![0,1,1];
        assert_eq!(Solution::placed_coins(edges, cost), ans);
    }

    #[test]
    fn test1() {
        let edges = to_vec2d([[7,0],[4,3],[4,8],[1,5],[6,2],[2,7],[7,9],[1,8],[1,9]]);
        let cost = vec![37,-48,30,-67,-84,36,-96,24,29,38];
        let ans = vec![306432,202608,1,1,1,1,1,306432,163212,213864];
        assert_eq!(Solution::placed_coins(edges, cost), ans);
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

impl Solution {
    pub fn placed_coins(edges: Vec<Vec<i32>>, cost: Vec<i32>) -> Vec<i64> {
        let mut ans = vec![0; cost.len()];
        let mut tree = vec![vec![]; cost.len()];
        for edge in &edges {
            tree[edge[0] as usize].push(edge[1] as usize);
            tree[edge[1] as usize].push(edge[0] as usize);
        }
        Self::dfs(0, cost.len(), &tree, &cost, &mut ans);
        ans
    }

    fn dfs(root: usize, parent: usize, tree: &Vec<Vec<usize>>, cost: &Vec<i32>, ans: &mut Vec<i64>) -> (i32, Vec<i64>, Vec<i64>) {
        let mut maxi = vec![cost[root] as i64];
        let mut mini = vec![cost[root] as i64];
        let mut cnt = 0;
        for &ch in tree[root].iter() {
            if ch != parent {
                let (t, ma, mi) = Self::dfs(ch, root, tree, cost, ans);
                cnt += t;
                Self::merge_maxi(&mut maxi, &ma);
                Self::merge_mini(&mut mini, &mi);
            }
        }
        cnt += 1;
        if cnt < 3 {
            ans[root] = 1;
        } else {
            ans[root] = Self::max(&maxi, &mini);
        }
        (cnt , maxi, mini)
    }

    fn max(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
        let mut r = 0;
        if a.len() >= 3 {
            r = r.max(a[0] * a[1] * a[2]);
        }
        if b.len() >= 2 && a.len() >= 1 {
            r = r.max(b[0] * b[1] * a[0]);
        }
        r
    }

    fn merge_maxi(m: &mut Vec<i64>, arr: &Vec<i64>) {
        for &v in arr {
            m.push(v);
        }
        m.sort();
        m.reverse();
        m.truncate(3);
    }

    fn merge_mini(m: &mut Vec<i64>, arr: &Vec<i64>) {
        for &v in arr {
            m.push(v);
        }
        m.sort();
        m.truncate(3);
    }
}