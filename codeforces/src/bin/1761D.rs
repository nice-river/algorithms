#![allow(
    non_snake_case,
    unused_imports,
    unused_variables,
    dead_code,
    unused_macros
)]

use std::{
    fmt::Debug,
    io::{BufReader, BufWriter, Read, Write},
    str::FromStr,
    sync::Once,
};

struct Reader<R> {
    inner: BufReader<R>,
    buf: Vec<u8>,
    pos: usize,
    len: usize,
}

impl<R: Read> Reader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner: BufReader::new(inner),
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
                if let Ok(n) = self.inner.read(&mut self.buf) {
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
                if let Ok(n) = self.inner.read(&mut self.buf) {
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

    fn try_read<T>(&mut self) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut val = Vec::with_capacity(32);
        self.skip_whitespaces();
        loop {
            if self.pos == self.len {
                if let Ok(n) = self.inner.read(&mut self.buf) {
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
        if val.len() == 0 {
            None
        } else {
            Some(std::str::from_utf8(&val).unwrap().parse().unwrap())
        }
    }
}

struct Writer<W: Write> {
    inner: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    fn new(inner: W) -> Self {
        Self {
            inner: BufWriter::new(inner),
        }
    }

    #[inline]
    fn get_mut_inner(&mut self) -> &'_ mut BufWriter<W> {
        &mut self.inner
    }

    fn flush(&mut self) {
        self.inner.flush().unwrap();
    }
}

static DIRS4: [i32; 5] = [-1, 0, 1, 0, -1];
static DIRS8: [i32; 9] = [-1, -1, 0, -1, 1, 0, 1, 1, -1];

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("input.txt")?;
    let mut reader = Reader::new(input);
    let output = std::io::stdout();
    let output: std::io::StdoutLock = output.lock();
    let mut writer = Writer::new(output);
    macro_rules! fast_write {
        ($($tt:expr),*) => {
            write!(writer.get_mut_inner(), $($tt),*)?;
        };
    }

    macro_rules! fast_writeln {
        ($($tt:expr),*) => {
            writeln!(writer.get_mut_inner(), $($tt),*)?;
        };
    }

    const MOD: i64 = 10i64.pow(9) + 7;

    let n: usize = reader.read();
    let k: i64 = reader.read();
    let mut pows = vec![1; n + 1];
    for i in 1..=n {
        pows[i] = pows[i - 1] * 3 % MOD;
    }
    let mut facs = vec![1; n + 1];
    for i in 2..=n {
        facs[i] = facs[i - 1] * i as i64 % MOD;
    }
    let mut rfacs = vec![1; n + 1];
    let (x, y) = egcd(facs[n], MOD);
    rfacs[n] = ((x % MOD) + MOD) % MOD;
    for i in (1..=n).rev() {
        rfacs[i - 1] = rfacs[i] * i as i64 % MOD;
    }
    let mut ans = 0;
    for q in 0..=n {
        let mut s = pows[n - q];
        s = s * comb(k - 1, (q as i64 + 1) / 2 - 1, &facs, &rfacs, MOD) % MOD;
        s = s * comb(n as i64 - k, q as i64 / 2, &facs, &rfacs, MOD) % MOD;
        ans += s;
        ans %= MOD;
    }

    fast_writeln!("{}", ans);

    writer.flush();
    Ok(())
}

fn comb(n: i64, k: i64, facs: &Vec<i64>, rfacs: &Vec<i64>, MOD: i64) -> i64 {
    if n == k {
        1
    } else if k > n || k < 0 {
        0
    } else {
        let n = n as usize;
        let k = k as usize;
        facs[n] * rfacs[k] % MOD * rfacs[n - k] % MOD
    }
}

fn egcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        (1, 0)
    } else {
        let (x, y) = egcd(b, a % b);
        (y, x - a / b * y)
    }
}
