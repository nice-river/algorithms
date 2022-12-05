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

use std::collections::HashSet;
use std::collections::VecDeque;

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
        let a: usize = reader.read();
        let b: usize = reader.read();
        let mut gph = vec![vec![]; n + 1];
        for i in 1..n {
            let u: usize = reader.read();
            let v: usize = reader.read();
            let w: usize = reader.read();
            gph[u].push((v, w));
            gph[v].push((u, w));
        }

        let mut tob = HashSet::new();

        let mut que = VecDeque::new();
        que.push_back((b, 0, 0));
        while let Some((cur, cur_w, parent)) = que.pop_front() {
            for &(nxt, w) in &gph[cur] {
                if nxt != parent {
                    que.push_back((nxt, cur_w ^ w, cur));
                    tob.insert(cur_w ^ w);
                }
            }
        }

        que.push_back((a, 0, 0));
        let mut can_reach = false;
        while let Some((cur, cur_w, parent)) = que.pop_front() {
            if cur == b {
                if cur_w == 0 {
                    can_reach = true;
                    break;
                }
                continue;
            }
            if tob.contains(&cur_w) {
                can_reach = true;
                break;
            }

            for &(nxt, w) in &gph[cur] {
                if nxt != parent {
                    que.push_back((nxt, cur_w ^ w, cur));
                }
            }
        }
        if can_reach {
            fast_writeln!("YES");
        } else {
            fast_writeln!("NO");
        }
    }

    writer.flush();
    Ok(())
}
