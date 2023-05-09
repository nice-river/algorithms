struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let mut date = date.split("-");
        let year = date.next().unwrap().parse::<i32>().unwrap();
        let month = date.next().unwrap().parse::<usize>().unwrap();
        let day = date.next().unwrap().parse::<i32>().unwrap();
        let mut month_days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            month_days[1] += 1;
        }
        let mut ans: i32 = month_days.iter().take(month - 1).sum();
        ans + day
    }
}
