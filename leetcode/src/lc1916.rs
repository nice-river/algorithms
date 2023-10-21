#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let prev_room = vec![-1, 0, 0, 1, 2];
        let ans = 6;
        assert_eq!(Solution::ways_to_build_rooms(prev_room), ans);
    }

    #[test]
    fn test_egcd() {
        let a = 5;
        let b = MODULE;
        dbg!(egcd(a, b));
    }

    #[test]
    fn test_comb() {
        let comb = Combine::new(5);
        dbg!(&comb.frac);
        dbg!(&comb.rfrac);
        dbg!(comb.comb(2, 1));
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

const MODULE: i64 = 1_000_000_007;

impl Solution {
    pub fn ways_to_build_rooms(prev_room: Vec<i32>) -> i32 {
        let n = prev_room.len();
        let mut tree = vec![vec![]; n];
        for (i, &r) in prev_room.iter().enumerate() {
            if r != -1 {
                tree[r as usize].push(i);
            }
        }
        let comb = Combine::new(n * 2);
        Solution::dfs(0, &tree, &comb).1 as i32
    }

    fn dfs(root: usize, tree: &Vec<Vec<usize>>, comb: &Combine) -> (i64, i64) {
        if tree[root].is_empty() {
            return (1, 1);
        }
        let mut ret = Self::dfs(tree[root][0], tree, comb);
        for &ch in tree[root].iter().skip(1) {
            let c = Self::dfs(ch, tree, comb);
            ret.1 = comb.comb(c.0 + ret.0, c.0) * ret.1 % MODULE * c.1 % MODULE;
            ret.0 = (ret.0 + c.0) % MODULE;
        }
        ret.0 += 1;
        ret
    }
}

struct Combine {
    frac: Vec<i64>,
    rfrac: Vec<i64>,
}

impl Combine {
    fn new(n: usize) -> Self {
        let mut frac = vec![1; n + 1];
        for i in 1..frac.len() {
            frac[i] = i as i64 * frac[i - 1] % MODULE;
        }
        let mut rfrac = vec![1; n + 1];
        rfrac[n] = (egcd(frac[n], MODULE).0 % MODULE + MODULE) % MODULE;
        for i in (1..rfrac.len() - 1).rev() {
            rfrac[i] = rfrac[i + 1] * (i as i64 + 1) % MODULE;
        }
        Self { frac, rfrac }
    }

    fn comb(&self, a: i64, b: i64) -> i64 {
        self.frac[a as usize] * self.rfrac[b as usize] % MODULE * self.rfrac[(a - b) as usize]
            % MODULE
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (x, y) = egcd(b, a % b);
    let k = a / b;
    (y, (x - k * y))
}
