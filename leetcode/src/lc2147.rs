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

const MODULE: i64 = 1_000_000_007;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let corridor = corridor.chars().collect::<Vec<_>>();
        let mut seats = vec![];
        let mut prev_seats = vec![vec![]; corridor.len()];
        for (i, &ch) in corridor.iter().enumerate() {
            if ch == 'S' {
                if seats.len() >= 2 {
                    prev_seats[i] = vec![seats[seats.len() - 2], seats[seats.len() - 1]];
                }
                seats.push(i);
            }
        }
        if seats.len() < 2 || seats.len() % 2 != 0 {
            return 0;
        }
        let mut dp = vec![0; corridor.len()];
        dp[seats[1]] = 1;
        let mut ans = 1;
        for i in seats[1] + 1..corridor.len() {
            if corridor[i] == 'S' {
                let k = dp[prev_seats[i][0]];
                let t = prev_seats[i][1] - prev_seats[i][0];
                dp[i] = k * t as i64 % MODULE;
                ans = dp[i];
            }
        }
        ans as i32
    }
}
