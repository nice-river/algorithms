#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let num = 100;
        let ans = "One Hundred";
        assert_eq!(Solution::number_to_words(num), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        let maps1000 = vec!["Thousand", "Million", "Billion", "Trillion"];
        let maps1000 = maps1000
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut ans: Vec<String> = vec![];
        let mut i = 0;
        loop {
            let k = num % 1000;
            if k != 0 {
                let mut res = Solution::less_thousand_to_english(k);
                res.reverse();
                ans.append(&mut res);
            }
            num /= 1000;
            if num != 0 {
                if num % 1000 != 0 {
                    ans.push(maps1000[i].clone());
                }
                i += 1;
            } else {
                break;
            }
        }
        ans.reverse();
        ans.join(" ")
    }

    fn less_thousand_to_english(num: i32) -> Vec<String> {
        assert!(num > 0);
        let maps = vec![
            "Zero",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];
        let maps = maps.into_iter().map(|s| s.to_string()).collect::<Vec<_>>();

        let maps10 = vec![
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        let maps10 = maps10
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        let mut ret = vec![];
        let _100 = num / 100;
        if _100 != 0 {
            ret.push(maps[_100 as usize].clone());
            ret.push("Hundred".to_string());
        }
        let num = num % 100;
        if num < 20 {
            if num > 0 {
                ret.push(maps[num as usize].clone());
            }
        } else {
            let _10 = num / 10;
            ret.push(maps10[_10 as usize].clone());
            if num % 10 != 0 {
                ret.push(maps[(num % 10) as usize].clone());
            }
        }
        dbg!(&ret);
        ret
    }
}
