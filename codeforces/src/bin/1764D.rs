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
        ($($expr:expr),*) => {
            write!(writer.get_mut_inner(), $($expr),*)?;
        };
    }

    macro_rules! fast_writeln {
        ($($expr:expr),*) => {
            writeln!(writer.get_mut_inner(), $($expr),*)?;
        };
    }

    let n: usize = reader.read();
    let p: i64 = reader.read();
    let mut frac = vec![1; n + 1];
    for i in 1..=n {
        frac[i] = frac[i - 1] * i as i64 % p;
    }
    let mut rfrac = frac.clone();
    let (x, y) = egcd(frac[n], p);
    rfrac[n] = ((x % p) + p) % p;
    for i in (1..=n).rev() {
        rfrac[i - 1] = rfrac[i] * i as i64 % p;
    }
    let comb = |a: usize, b: usize| frac[a] * rfrac[a - b] % p * rfrac[b] % p;
    let accu = |a: usize, b: usize| frac[a] * rfrac[a - b] % p;

    let mut ans = 0;
    for j in 2..=(n + 1) / 2 {
        let i0 = 1 + (n + 1) / 2;
        let i1 = n;
        let j0 = j + 1;
        let j1 = j + n / 2;
        let x = i0.max(j0);
        let y = i1.min(j1);
        for k in 0..=j - 2 {
            let t = comb(j - 2, k) * accu(j - 2 - k + n - j - 1, j - 2 - k + n - j - 1) % p
                * ((y - x + 1) as i64)
                % p;
            ans = (ans + t) % p;
        }
    }
    ans = ans * n as i64 % p;
    if n % 2 == 0 {
        let t = accu(n - 2, n - 2);
        ans = (ans + t * (n as i64) % p) % p;
    }
    fast_writeln!("{}", ans);
    writer.flush();
    Ok(())
}

fn egcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        (1, 0)
    } else {
        let (x, y) = egcd(b, a % b);
        (y, x - a / b * y)
    }
}
