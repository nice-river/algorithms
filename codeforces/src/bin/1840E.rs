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

// const MODULES: [i64; 3] = [1000000007, 1700002529, 2300002931];
const MODULES: [i64; 1] = [1000000007];

struct HashStr {
    hashes: [i64; MODULES.len()],
    bases: Vec<Vec<i64>>,
}

impl HashStr {
    fn init(s: &[u8]) -> Self {
        let mut hashes = [0; MODULES.len()];
        for &b in s.iter().rev() {
            let b = (b - b'a') as i64;
            for i in 0..MODULES.len() {
                hashes[i] = hashes[i] * 26 + b;
                hashes[i] %= MODULES[i];
            }
        }
        let mut bases = vec![vec![1]; MODULES.len()];
        for _ in 1..s.len() {
            for i in 0..MODULES.len() {
                let t = *bases[i].last().unwrap();
                bases[i].push(t * 26 % MODULES[i])
            }
        }
        Self { hashes, bases }
    }

    fn clear(&mut self, pos: usize, b: u8) {
        for i in 0..MODULES.len() {
            let b = (b - b'a') as i64;
            let t = self.hashes[i] - b * self.bases[i][pos];
            self.hashes[i] = (t % MODULES[i] + MODULES[i]) % MODULES[i];
        }
    }

    fn set(&mut self, pos: usize, b: u8) {
        for i in 0..MODULES.len() {
            let b = (b - b'a') as i64;
            let t = self.hashes[i] + b * self.bases[i][pos];
            self.hashes[i] = t % MODULES[i];
        }
    }
}

impl PartialEq for HashStr {
    fn eq(&self, other: &Self) -> bool {
        self.hashes == other.hashes
    }
}

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

    for _ in 0..reader.read() {
        let s1: String = reader.read();
        let s2: String = reader.read();
        let t: i32 = reader.read();
        let mut s = [s1.into_bytes(), s2.into_bytes()];
        let mut hs = [HashStr::init(&s[0]), HashStr::init(&s[1])];
        let mut block_time: Vec<(i32, usize)> = vec![];
        let mut p_block = 0;
        for i in 0..reader.read() {
            let tp: i32 = reader.read();
            if p_block < block_time.len() && i >= block_time[p_block].0 + t {
                let p = block_time[p_block].1;
                hs[0].set(p - 1, s[0][p - 1]);
                hs[1].set(p - 1, s[1][p - 1]);
                p_block += 1;
            }
            if tp == 1 {
                let p: usize = reader.read();
                hs[0].clear(p - 1, s[0][p - 1]);
                hs[1].clear(p - 1, s[1][p - 1]);
                block_time.push((i, p));
            } else if tp == 2 {
                let w1 = reader.read::<usize>() - 1;
                let p1 = reader.read::<usize>();
                let w2 = reader.read::<usize>() - 1;
                let p2 = reader.read::<usize>();
                let a = s[w1][p1 - 1];
                let b = s[w2][p2 - 1];
                hs[w1].clear(p1 - 1, a);
                hs[w1].set(p1 - 1, b);
                hs[w2].clear(p2 - 1, b);
                hs[w2].set(p2 - 1, a);
                s[w1][p1 - 1] = b;
                s[w2][p2 - 1] = a;
            } else {
                if hs[0] == hs[1] {
                    fast_writeln!("YES");
                } else {
                    fast_writeln!("NO");
                }
            }
        }
    }

    writer.flush();
    Ok(())
}
