#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}

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
    pub fn minimum_money(transactions: Vec<Vec<i32>>) -> i64 {
        let mut wins = transactions
            .iter()
            .filter(|a| a[1] > a[0])
            .collect::<Vec<_>>();
        wins.sort_by_key(|a| -a[0]);
        let mut loses = transactions
            .iter()
            .filter(|a| a[1] <= a[0])
            .collect::<Vec<_>>();
        loses.sort_by_key(|a| a[1]);
        let (mut ans, mut cash) = if loses.len() != 0 {
            (loses[0][0] as i64, loses[0][1] as i64)
        } else {
            (wins[0][0] as i64, wins[0][0] as i64)
        };

        for i in 1..loses.len() {
            if cash < loses[i][0] as i64 {
                ans += loses[i][0] as i64 - cash;
                cash = loses[i][1] as i64;
            } else {
                cash -= (loses[i][0] - loses[i][1]) as i64;
            }
        }
        if wins.len() > 0 {
            if cash < wins[0][0] as i64 {
                ans += wins[0][0] as i64 - cash
            }
        }
        ans
    }
}
