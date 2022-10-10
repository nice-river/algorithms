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
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);
    for _ in 0..reader.read() {
        let n: i64 = reader.read();
        let m: i64 = reader.read();
        println!("{}", solve(n, m));
    }

    Ok(())
}

fn solve(n: i64, m: i64) -> i64 {
    const MOD: i64 = 998244353;
    const D: usize = 31;
    let mut ans = 0;
    let mut digit_cnt = vec![0i64; D];
    let mut tail = 0;
    for i in 0..D {
        let k = m >> (i + 1);
        digit_cnt[i] = k * (1 << i);
        if ((1 << i) & m) != 0 {
            digit_cnt[i] += tail + 1;
        }
        tail += (1 << i) & m;
    }
    for i in 0..D {
        if digit_cnt[i] != 0 {
            ans = (ans + fast_pow(digit_cnt[i], n) * (1 << i) % MOD) % MOD;
        }
    }
    ans
}

fn fast_pow(mut base: i64, mut exp: i64) -> i64 {
    const MOD: i64 = 998244353;
    let mut ans = 1;
    while exp != 0 {
        if (exp & 1) == 1 {
            ans = ans * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    ans
}
