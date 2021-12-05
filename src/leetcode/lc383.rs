#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut map = vec![0; 256];
        for &b in magazine.as_bytes() {
            map[b as usize] += 1;
        }
        for &b in ransom_note.as_bytes() {
            if map[b as usize] == 0 {
                return false;
            }
            map[b as usize] -= 1;
        }
        true
    }
}
