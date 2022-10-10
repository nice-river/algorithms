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

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    // let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    const MOD: i64 = 998244353;

    for _ in 0..reader.read() {
        let s: String = reader.read();
        let s = s.into_bytes();
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1i64;
        for i in 1..=s.len() {
            dp[i] = dp[i - 1];
            if i >= 2 && (&s[i - 2..i] == &[b'a', b'b'] || &s[i - 2..i] == &[b'b', b'a']) {
                dp[i] = (dp[i] + dp[i - 2]) % MOD;
            }
        }
        println!("{}", dp[s.len()]);
    }

    Ok(())
}
