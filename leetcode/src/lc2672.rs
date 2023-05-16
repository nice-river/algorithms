#[cfg(test)]
mod tests {
    #[test]
    fn test0() {}
}

struct Solution {}

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut ans = Vec::with_capacity(queries.len());
        let mut colors = vec![0; n + 2];
        let mut c = 0;
        for query in queries {
            let i = query[0] as usize + 1;
            let color = query[1];
            if color != colors[i] {
                if colors[i] != 0 {
                    if colors[i - 1] == colors[i] {
                        c -= 1;
                    }
                    if colors[i] == colors[i + 1] {
                        c -= 1;
                    }
                }
                if color != 0 {
                    if colors[i - 1] == color {
                        c += 1;
                    }
                    if colors[i + 1] == color {
                        c += 1;
                    }
                }
            }
            colors[i] = color;
            ans.push(c);
        }
        ans
    }
}
