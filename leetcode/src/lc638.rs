#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let price = vec![2, 5];
        let special = vec![vec![3, 0, 5], vec![1, 2, 10]];
        let needs = vec![3, 2];
        let ans = 14;
        assert_eq!(Solution::shopping_offers(price, special, needs), ans);
    }
}

struct Solution {}

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        Solution::dfs(
            0,
            &price,
            &special,
            &needs,
            &mut vec![0; price.len()],
            0,
            &mut ans,
        );
        ans
    }

    pub fn dfs(
        idx: usize,
        price: &Vec<i32>,
        special: &Vec<Vec<i32>>,
        needs: &Vec<i32>,
        cnts: &mut Vec<i32>,
        cur_price: i32,
        ans: &mut i32,
    ) {
        if idx == special.len() {
            let mut t = 0;
            for i in 0..price.len() {
                t += price[i] * (needs[i] - cnts[i]);
            }
            *ans = (*ans).min(t + cur_price);
            return;
        }
        Solution::dfs(idx + 1, price, special, needs, cnts, cur_price, ans);
        let special_price = *special[idx].last().unwrap();
        'outer: for i in 1.. {
            for j in 0..cnts.len() {
                if cnts[j] + special[idx][j] * i > needs[j] {
                    break 'outer;
                }
            }
            for j in 0..cnts.len() {
                cnts[j] += special[idx][j] * i;
            }
            Solution::dfs(
                idx + 1,
                price,
                special,
                needs,
                cnts,
                cur_price + special_price * i,
                ans,
            );
            for j in 0..cnts.len() {
                cnts[j] -= special[idx][j] * i;
            }
        }
    }
}
