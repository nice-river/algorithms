#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    io::{BufReader, Read},
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

static DIRS: [i32; 5] = [-1, 0, 1, 0, -1];
static DIRS8: [i32; 9] = [-1, -1, 0, -1, 1, 0, 1, 1, -1];

use std::collections::VecDeque;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    let mut vis = vec![vec![false; 8]; 8];
    let mut que = VecDeque::new();

    for _ in 0..reader.read() {
        let r: i32 = reader.read();
        let c: i32 = reader.read();
        let k: i32 = reader.read();
        vis.iter_mut().for_each(|row| row.fill(false));
        que.push_back((r, c, 0));
        vis[r as usize - 1][c as usize - 1] = true;
        let mut ans = 0;
        while let Some((x, y, s)) = que.pop_front() {
            ans += 1;
            if s == k {
                continue;
            }
            for d in 0..8 {
                let nx = x + DIRS8[d];
                let ny = y + DIRS8[d + 1];
                if nx < 1 || ny < 1 || nx > 8 || ny > 8 || vis[nx as usize - 1][ny as usize - 1] {
                    continue;
                }
                vis[nx as usize - 1][ny as usize - 1] = true;
                que.push_back((nx, ny, s + 1));
            }
        }
        println!("{}", ans);
    }

    Ok(())
}
