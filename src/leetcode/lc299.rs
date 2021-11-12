#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut map = HashMap::new();
        let mut bull = 0;
        let mut cow = 0;
        for (&a, &b) in secret.as_bytes().iter().zip(guess.as_bytes()) {
            if a == b {
                bull += 1;
            } else {
                *map.entry(a).or_insert(0) += 1;
            }
        }
        for (&a, &b) in secret.as_bytes().iter().zip(guess.as_bytes()) {
            if a != b {
                if let Some(cnt) = map.get_mut(&b) {
                    if *cnt > 0 {
                        cow += 1;
                        *cnt -= 1;
                    }
                }
            }
        }
        format!("{}A{}B", bull, cow)
    }
}
