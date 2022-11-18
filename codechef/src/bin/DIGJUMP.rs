#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    io::{BufReader, Read, Write},
    str::FromStr,
};

struct Reader<R> {
    reader: BufReader<R>,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
}

impl<R: Read> Reader<R> {
    fn new(inner: R) -> Self {
        Self {
            reader: BufReader::new(inner),
            buf: vec![0; 4],
            pos: 0,
            len: 0,
        }
    }

    #[inline]
    fn skip_whitespaces(&mut self) {
        loop {
            while self.pos < self.len && self.buf[self.pos].is_ascii_whitespace() {
                self.pos += 1
            }
            if self.pos == self.len {
                if let Ok(n) = self.reader.read(&mut self.buf) {
                    self.len = n;
                    self.pos = 0;
                    if n != 0 {
                        continue;
                    }
                }
            }
            break;
        }
    }

    fn read<T>(&mut self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut val = Vec::with_capacity(32);

        self.skip_whitespaces();

        loop {
            if self.pos == self.len {
                if let Ok(n) = self.reader.read(&mut self.buf) {
                    self.len = n;
                    self.pos = 0;
                    if n == 0 {
                        break;
                    }
                }
            } else {
                if self.buf[self.pos].is_ascii_whitespace() {
                    break;
                }
                val.push(self.buf[self.pos]);
                self.pos += 1;
            }
        }
        std::str::from_utf8(&val).unwrap().parse().unwrap()
    }
}

static DIRS4: [i32; 5] = [-1, 0, 1, 0, -1];
static DIRS8: [i32; 9] = [-1, -1, 0, -1, 1, 0, 1, 1, -1];

use std::collections::VecDeque;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    let writer = std::io::stdout();
    let mut writer = writer.lock();

    let s: String = reader.read();
    let s = s.as_bytes();
    let mut pos = vec![vec![]; 10];
    for (i, &c) in s.iter().enumerate() {
        pos[(c - b'0') as usize].push(i);
    }
    let mut vis = vec![false; s.len()];
    vis[0] = true;
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((p, step)) = que.pop_front() {
        if p == s.len() - 1 {
            writeln!(writer, "{}", step)?;
            break;
        }
        if p > 0 && !vis[p - 1] {
            que.push_back((p - 1, step + 1));
            vis[p - 1] = true;
        }
        if !vis[p + 1] {
            que.push_back((p + 1, step + 1));
            vis[p + 1] = true;
        }
        for &q in &pos[(s[p] - b'0') as usize] {
            if !vis[q] {
                que.push_back((q, step + 1));
                vis[q] = true;
            }
        }
        pos[(s[p] - b'0') as usize].clear();
    }
    Ok(())
}
