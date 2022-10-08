#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let words = vec!["abcb".to_string()];
        let ans: Vec<String> = vec![];
        assert_eq!(Solution::find_words(board, words), ans);
    }

    #[test]
    fn test1() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v'],
        ];
        let words = vec![
            "oath".to_string(),
            "pea".into(),
            "eat".into(),
            "rain".into(),
        ];
        let ans: Vec<String> = vec!["oath".into(), "eat".into()];
        assert_eq!(Solution::find_words(board, words), ans);
    }

    #[test]
    fn test2() {
        let board = vec![
            vec!['a', 'b', 'c'],
            vec!['a', 'e', 'd'],
            vec!['a', 'f', 'g'],
        ];
        let words = vec![
            "abcdefg".to_string(),
            "gfedcbaaa".into(),
            "eaabcdgfa".into(),
            "befa".into(),
            "dgc".into(),
            "ade".into(),
        ];
        let ans: Vec<String> = vec![
            "abcdefg".into(),
            "gfedcbaaa".into(),
            "eaabcdgfa".into(),
            "befa".into(),
        ];
        assert_eq!(Solution::find_words(board, words), ans);
    }
}

struct Solution {}

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut char_pos = HashMap::new();
        for (i, row) in board.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                char_pos.entry(ch as u8).or_insert(vec![]).push((i, j))
            }
        }
        let mut vis = vec![vec![false; board[0].len()]; board.len()];
        let mut ans = vec![];
        for word in words {
            let w = word.as_bytes();
            if let Some(p) = char_pos.get(&w[0]) {
                for &start_pos in p {
                    for row in vis.iter_mut() {
                        row.iter_mut().for_each(|cell| *cell = false);
                    }
                    if Solution::in_board(&board, &mut vis, start_pos, w) {
                        ans.push(word);
                        break;
                    }
                }
            }
        }
        ans
    }

    fn in_board(
        board: &Vec<Vec<char>>,
        vis: &mut Vec<Vec<bool>>,
        start: (usize, usize),
        word: &[u8],
    ) -> bool {
        static dirs: [i32; 5] = [-1, 0, 1, 0, -1];

        fn dfs(
            idx: usize,
            board: &Vec<Vec<char>>,
            vis: &mut Vec<Vec<bool>>,
            i: usize,
            j: usize,
            word: &[u8],
        ) -> bool {
            if idx == word.len() {
                return true;
            }
            let n = board.len();
            let m = board[0].len();
            for d in 0..4 {
                let x = i as i32 + dirs[d];
                let y = j as i32 + dirs[d + 1];
                if x < 0 || y < 0 || x >= n as i32 || y >= m as i32 {
                    continue;
                }
                let x = x as usize;
                let y = y as usize;
                if vis[x][y] || board[x][y] as u8 != word[idx] {
                    continue;
                }
                vis[x][y] = true;
                if dfs(idx + 1, board, vis, x, y, word) {
                    return true;
                }
                vis[x][y] = false;
            }
            false
        }
        vis[start.0][start.1] = true;
        dfs(1, board, vis, start.0, start.1, word)
    }
}
