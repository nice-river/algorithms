#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num = 26;
        let ans = "1a".to_string();
        assert_eq!(Solution::to_hex(num), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn to_hex(mut num: i32) -> String {
        let mut hexes = vec![];
        for _ in 0..8 {
            let t = num & 0b1111;
            let t = if t <= 9 {
                ((b'0' + t as u8) as char).to_string()
            } else {
                ((b'a' + (t as u8 - 10)) as char).to_string()
            };
            hexes.push(t);
            num >>= 4;
        }
        while hexes.len() > 1 && hexes.last().unwrap() == "0" {
            hexes.pop();
        }
        hexes.reverse();
        hexes.join("")
    }
}
