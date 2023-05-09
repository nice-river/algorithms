#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let words = vec![
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ];
        let words = words.into_iter().map(|s| s.to_string()).collect();
        let max_width = 16;
        println!("{:?}", Solution::full_justify(words, max_width));
    }
}

struct Solution {}

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let words = words.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
        let max_width = max_width as usize;
        let mut ans = vec![];
        let mut line = vec![];
        let mut line_width = 0;
        for word in words {
            let word_len = word.len();
            if line_width + line.len() + word_len > max_width {
                Solution::push_to_ans(&mut ans, &mut line, &mut line_width, max_width);
            }
            line_width += word_len;
            line.push(word);
        }
        if line.len() != 0 {
            let mut s = vec![];
            for w in line {
                s.extend(w.iter());
                s.push(b' ');
            }
            if s.len() > max_width {
                s.pop();
            }
            s.append(&mut vec![b' '; max_width - s.len()]);
            ans.push(String::from_utf8(s).unwrap());
        }
        ans
    }

    fn push_to_ans(
        ans: &mut Vec<String>,
        line: &mut Vec<&[u8]>,
        line_width: &mut usize,
        max_width: usize,
    ) {
        let space_pos = line.len() - 1;
        let left_width = max_width - *line_width;
        let mut s: Vec<u8> = vec![];
        s.extend(line[0].iter());
        if space_pos == 0 {
            s.append(&mut vec![b' '; left_width]);
        } else {
            let space = left_width / space_pos;
            let mut extra_space = left_width % space_pos;
            for i in 1..line.len() {
                s.append(&mut vec![
                    b' ';
                    space
                        + if extra_space > 0 {
                            extra_space -= 1;
                            1
                        } else {
                            0
                        }
                ]);
                s.extend(line[i].iter());
            }
        }
        ans.push(String::from_utf8(s).unwrap());
        line.clear();
        *line_width = 0;
    }
}
