#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::BTreeMap;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        if hand.len() % group_size as usize != 0 {
            return false;
        }
        if group_size == 1 {
            return true;
        }
        let mut map = BTreeMap::new();
        for h in hand {
            *map.entry(h).or_insert(0) += 1
        }
        while !map.is_empty() {
            let (&x, &c) = map.iter().next().unwrap();
            for i in 1..group_size {
                if let Some(t) = map.get_mut(&(x + i)) {
                    *t -= 1;
                    if *t == 0 {
                        map.remove(&(x + i));
                    }
                } else {
                    return false;
                }
            }
            if c == 1 {
                map.remove(&x);
            } else {
                *map.get_mut(&x).unwrap() -= 1;
            }
        }
        true
    }
}
