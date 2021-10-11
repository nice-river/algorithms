#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let machines = vec![1, 0, 5];
        let ans = 3;
        assert_eq!(Solution::find_min_moves(machines), ans);
    }

    #[test]
    fn test1() {
        let machines = vec![2, 2, 2, 0, 0, 0];
        let ans = 3;
        assert_eq!(Solution::find_min_moves(machines), ans);
    }

    #[test]
    fn test2() {
        let machines = vec![3, 2, 1, 0, 0, 0];
        let ans = 3;
        assert_eq!(Solution::find_min_moves(machines), ans);
    }

    #[test]
    fn test3() {
        let machines = vec![1, 0, 10, 1];
        let ans = 7;
        assert_eq!(Solution::find_min_moves(machines), ans);
    }

    #[test]
    fn test4() {
        let machines = vec![0, 0, 12, 0];
        let ans = 9;
        assert_eq!(Solution::find_min_moves(machines), ans);

        let machines = vec![0, 12, 0, 0];
        let ans = 9;
        assert_eq!(Solution::find_min_moves(machines), ans);
    }

    #[test]
    fn test5() {
        let machines = vec![0, 12, 12, 0];
        let ans = 6;
        assert_eq!(Solution::find_min_moves(machines), ans);
    }

    #[test]
    fn test6() {
        let machines = vec![1, 0, 5, 6];
        let ans = 5;
        assert_eq!(Solution::find_min_moves(machines), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn find_min_moves(mut machines: Vec<i32>) -> i32 {
        let n = machines.len();
        let sum: i32 = machines.iter().sum();
        if sum % n as i32 != 0 {
            return -1;
        }
        let target = sum / n as i32;
        let mut ans = 0;
        let mut s = 0;
        for num in machines {
            let d = num - target;
            s += d;
            ans = ans.max(s.abs()).max(d);
        }
        ans
    }
}
