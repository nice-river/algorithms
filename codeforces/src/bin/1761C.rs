#![allow(
    non_snake_case,
    unused_imports,
    unused_variables,
    dead_code,
    unused_macros
)]

use std::{
    fmt::Debug,
    fs::read,
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

use std::collections::HashSet;

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

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let mut mat = Vec::with_capacity(n);
        for _ in 0..n {
            let s: String = reader.read();
            mat.push(s.into_bytes());
        }
        let mut dp = vec![HashSet::<usize>::new(); n];
        for i in 0..n {
            dfs(&mat, i, &mut dp);
            fast_write!("{} ", dp[i].len());
            for v in dp[i].iter() {
                fast_write!("{} ", v);
            }
            fast_writeln!();
        }
    }

    writer.flush();
    Ok(())
}

fn dfs(mat: &Vec<Vec<u8>>, idx: usize, dp: &mut Vec<HashSet<usize>>) {
    if dp[idx].len() != 0 {
        return;
    }
    let mut s = HashSet::new();
    s.insert(idx + 1);
    for r in 0..mat.len() {
        if mat[r][idx] == b'1' {
            dfs(mat, r, dp);
            for &v in dp[r].iter() {
                s.insert(v);
            }
        }
    }
    dp[idx] = s;
}
