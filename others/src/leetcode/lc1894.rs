#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

struct Solution {}

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let k = (k as i64) % chalk.iter().map(|c| *c as i64).sum::<i64>();
        let mut k = k as i32;
        for (i, &ch) in chalk.iter().enumerate() {
            if k < ch {
                return i as i32;
            }
            k -= ch;
        }
        unreachable!()
    }
}
