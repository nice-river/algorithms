struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "3+2*2";
        let ans = 7;
        assert_eq!(Solution::calculate(s.to_string()), ans);
    }

    #[test]
    fn test2() {
        let s = "3/2";
        let ans = 1;
        assert_eq!(Solution::calculate(s.to_string()), ans);
    }

    #[test]
    fn test3() {
        let s = "3+5 / 2";
        let ans = 5;
        assert_eq!(Solution::calculate(s.to_string()), ans);
    }

    #[test]
    fn test4() {
        let s = "-(1 + 24 / 2 - 3 * 8)";
        let ans = 11;
        assert_eq!(Solution::calculate(s.to_string()), ans);
    }
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        Solution::helper(s, 0)
    }

    fn helper(s: &[u8], idx: usize) -> i32 {
        let (num, i) = Solution::get_parenthesis_result(s, idx);
        num
    }

    fn get_parenthesis_result(s: &[u8], idx: usize) -> (i32, usize) {
        let mut i = idx;
        let mut num_stk = vec![];
        let mut op_stk = vec![];
        let g: i32;
        if let Some(v) = num_stk.pop() {

        }
        loop {
            Solution::skip_white_spaces(s, &mut i);
            if i == s.len() {
                break;
            }
            match s[i] {
                b'(' => {
                    let (num, k) = Solution::get_parenthesis_result(s, i + 1);
                    num_stk.push(num);
                    i = k;
                }
                b')' => {
                    i += 1;
                    break;
                }
                b'-' => {
                    if op_stk.len() == num_stk.len() {
                        let (num, j) = Solution::get_next_num(s, i + 1);
                        num_stk.push(-num);
                        i = j;
                    } else {
                        op_stk.push(s[i]);
                        i += 1;
                    }
                }
                b'*' | b'/' | b'-' | b'+' => {
                    op_stk.push(s[i]);
                    i += 1;
                }
                b'0'..=b'9' => {
                    let (num, j) = Solution::get_next_num(s, i);
                    num_stk.push(num);
                    i = j;
                }
                _ => panic!(format!("unexpected character {}", s[i] as char)),
            }
        }
        if num_stk.is_empty() {
            return (0, i);
        }
        let mut nums = vec![num_stk[0]];
        let mut ops = vec![];
        for p in 0..op_stk.len() {
            match op_stk[p] {
                b'+' | b'-' => {
                    nums.push(num_stk[p + 1]);
                    ops.push(op_stk[p]);
                }
                b'*' => {
                    let n = nums.pop().unwrap() * num_stk[p + 1];
                    nums.push(n);
                }
                _ => {
                    let n = nums.pop().unwrap() / num_stk[p + 1];
                    nums.push(n);
                }
            }
        }
        let mut ret = nums[0];
        for p in 0..ops.len() {
            if ops[p] == b'+' {
                ret += nums[p + 1];
            } else {
                ret -= nums[p + 1];
            }
        }
        (ret, i)
    }

    fn get_next_num(s: &[u8], idx: usize) -> (i32, usize) {
        let mut i = idx;
        Solution::skip_white_spaces(s, &mut i);
        if i == s.len() {
            return (0, i);
        }
        if s[i] == b'(' {
            return Solution::get_parenthesis_result(s, i + 1);
        }
        let mut j = i + 1;
        while j < s.len() && b'0' <= s[j] && s[j] <= b'9' {
            j += 1;
        }
        let num = std::str::from_utf8(&s[i..j])
            .map(|s| s.parse::<i32>().unwrap())
            .unwrap();
        return (num, j);
    }

    fn skip_white_spaces(s: &[u8], idx: &mut usize) {
        while *idx < s.len() && s[*idx] == b' ' {
            *idx += 1;
        }
    }
}
