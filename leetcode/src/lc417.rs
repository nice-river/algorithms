#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = heights.len();
        let m = heights[0].len();
        let mut marks = vec![vec![0; m]; n];
        let mut ans = vec![];
        for i in 0..n {
            Self::dfs(i, 0, 1, &mut marks, &heights);
            Self::dfs(i, m - 1, 2, &mut marks, &heights);
        }
        for j in 0..m {
            Self::dfs(0, j, 1, &mut marks, &heights);
            Self::dfs(n - 1, j, 2, &mut marks, &heights);
        }
        for (i, row) in marks.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell == &3 {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }

    fn dfs(i: usize, j: usize, mark: i32, marks: &mut Vec<Vec<i32>>, heights: &Vec<Vec<i32>>) {
        if (marks[i][j] & mark) != 0 {
            return;
        }
        marks[i][j] |= mark;
        let dirs = vec![-1, 0, 1, 0, -1];
        for d in 0..4 {
            let x = i as i32 + dirs[d];
            let y = j as i32 + dirs[d + 1];
            if x < 0 || y < 0 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            if x >= marks.len() || y >= marks[0].len() || heights[x][y] < heights[i][j] {
                continue;
            }
            Self::dfs(x, y, mark, marks, heights);
        }
    }
}
