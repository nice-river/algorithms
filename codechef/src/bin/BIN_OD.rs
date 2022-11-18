#![allow(non_snake_case, unused_imports, unused_variables, dead_code)]

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
            buf: vec![0; 128],
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

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    let writer = std::io::stdout();
    let mut writer = writer.lock();

    const B: usize = 60;

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let q: usize = reader.read();
        let mut arr = vec![0i64; n + 1];
        let mut s = vec![vec![vec![0; 2]; B]; n + 1];
        for i in 1..=n {
            arr[i] = reader.read();
            for b in 0..B {
                s[i][b][0] = 1 - ((arr[i] >> b) & 1) + s[i - 1][b][0];
                s[i][b][1] = ((arr[i] >> b) & 1) + s[i - 1][b][1];
            }
        }

        for _ in 0..q {
            let k: usize = reader.read();
            let l1: usize = reader.read();
            let r1: usize = reader.read();
            let l2: usize = reader.read();
            let r2: usize = reader.read();
            let mut ans = 0;
            ans += (s[r1][k][0] - s[l1 - 1][k][0]) * (s[r2][k][1] - s[l2 - 1][k][1]);
            ans += (s[r1][k][1] - s[l1 - 1][k][1]) * (s[r2][k][0] - s[l2 - 1][k][0]);
            writeln!(writer, "{}", ans)?;
        }
    }

    Ok(())
}
