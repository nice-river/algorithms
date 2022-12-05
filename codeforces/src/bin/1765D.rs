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

    let n: usize = reader.read();
    let m: i64 = reader.read();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(reader.read::<i64>());
    }
    a.sort_by_key(|&v| std::cmp::Reverse(v));
    let mut vis = vec![false; n];
    let mut b = Vec::with_capacity(n);
    let mut left = 0;
    let mut right = n - 1;
    while left < right {
        if a[left] + a[right] <= m {
            if b.last().is_none() || b.last().unwrap() != &left {
                b.push(left);
                b.push(right);
                vis[left] = true;
                vis[right] = true;
                if left + 1 < right {
                    b.push(left + 1);
                    vis[left + 1] = true;
                }
                left += 1;
                right -= 1;
            } else {
                b.push(right);
                vis[right] = true;
                if left + 1 < right {
                    vis[left + 1] = true;
                    b.push(left + 1);
                }
                left += 1;
                right -= 1;
            }
        } else {
            left += 1;
        }
    }
    for i in 0..n {
        if !vis[i] {
            b.push(i);
        }
    }
    let mut ans = a[b[0]];
    for i in 1..n {
        if a[b[i - 1]] + a[b[i]] <= m {
            ans += a[b[i]];
        } else {
            ans += 1 + a[b[i]];
        }
    }
    fast_writeln!("{}", ans + 1);

    writer.flush();
    Ok(())
}
