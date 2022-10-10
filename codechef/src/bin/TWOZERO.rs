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

const MOD: i64 = 1_000_000_000 + 7;
const R3: i64 = 333333336;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    // let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    for _ in 0..reader.read() {
        let n: i64 = reader.read();
        let m: i64 = reader.read();
        let mut ans = 0;
        ans = (ans + f0(n) * f0(m) % MOD) % MOD;
        ans = (ans + f1(n) * f1(m) % MOD) % MOD;
        ans = (ans + f2(n) * f2(m) % MOD) % MOD;
        println!("{}", (ans - 1 + MOD) % MOD);
    }

    Ok(())
}

fn fast_pow(mut base: i64, mut exp: i64) -> i64 {
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

fn egcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (m, n) = egcd(b, a % b);
    (n, m - a / b * n)
}

fn f0(n: i64) -> i64 {
    let x = fast_pow(2, n);
    let y = ((n as f64 * std::f64::consts::PI / 3.0f64).cos() * 2.0f64).round() as i64;
    (x + y) * R3 % MOD
}

fn f1(n: i64) -> i64 {
    if n < 2 {
        return n;
    }
    let x = fast_pow(2, n);
    let y = (((n - 2) as f64 * std::f64::consts::PI / 3.0f64).cos() * 2.0f64).round() as i64;
    (x + y) * R3 % MOD
}

fn f2(n: i64) -> i64 {
    if n < 4 {
        return n * (n - 1) / 2 % MOD;
    }
    let x = fast_pow(2, n);
    let y = (((n - 4) as f64 * std::f64::consts::PI / 3.0f64).cos() * 2.0f64).round() as i64;
    (x + y) * R3 % MOD
}
