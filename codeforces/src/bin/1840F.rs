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

    use std::collections::BTreeMap;
    use std::collections::VecDeque;

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let m: usize = reader.read();
        let r: usize = reader.read();
        let mut shoots = BTreeMap::new();
        for _ in 0..r {
            let t: usize = reader.read();
            let d: i32 = reader.read();
            let c: usize = reader.read();
            shoots.entry(t).or_insert_with(|| vec![]).push((d, c));
        }
        let shoots = shoots.into_iter().map(|(k, v)| (k, v)).collect::<Vec<_>>();
        let mut que = VecDeque::new();
        que.push_back((0, 0, 0, 0, 0));
        let mut dp = vec![vec![vec![false; r + 1]; m + 1]; n + 1];
        let mut ans = -1;
        dp[0][0][0] = true;
        while let Some((x, y, w, s, t)) = que.pop_front() {
            if x == n && y == m {
                ans = s as i32;
                break;
            }
            for (dx, dy) in [(0, 0), (1, 0), (0, 1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx > n || ny > m {
                    continue;
                }
                if t < shoots.len() && shoots[t].0 == s + 1 {
                    let mut hit = false;
                    for &(d, p) in &shoots[t].1 {
                        if d == 1 && p == nx || d == 2 && p == ny {
                            hit = true;
                            break;
                        }
                    }
                    if !hit {
                        if dx == 0 && dy == 0 && w + 1 <= r {
                            if !dp[nx][ny][w + 1] {
                                dp[nx][ny][w + 1] = true;
                                que.push_back((nx, ny, w + 1, s + 1, t + 1));
                            }
                        } else if !dp[nx][ny][w] {
                            dp[nx][ny][w] = true;
                            que.push_back((nx, ny, w, s + 1, t + 1));
                        }
                    }
                } else {
                    if dx == 0 && dy == 0 && w + 1 <= r {
                        if !dp[nx][ny][w + 1] {
                            dp[nx][ny][w + 1] = true;
                            que.push_back((nx, ny, w + 1, s + 1, t));
                        }
                    } else if !dp[nx][ny][w] {
                        dp[nx][ny][w] = true;
                        que.push_back((nx, ny, w, s + 1, t));
                    }
                }
            }
        }
        fast_writeln!("{}", ans);
    }

    writer.flush();
    Ok(())
}
