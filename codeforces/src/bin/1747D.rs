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

use std::collections::{HashMap, BTreeSet};

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

    let mut map = HashMap::new();

    let n: usize = reader.read();
    let q: usize = reader.read();
    let mut xor_vals = vec![0; n + 1];
    let mut sum_vals = vec![0; n + 1];
    let mut a = vec![0; n + 1];
    map.insert((0, 0), BTreeSet::from_iter(0..1));
    for i in 1..=n {
        a[i] = reader.read();
        xor_vals[i] = a[i] ^ xor_vals[i - 1];
        let entry = map.entry((xor_vals[i], i % 2)).or_insert_with(|| BTreeSet::new());
        entry.insert(i);
        sum_vals[i] = a[i] as i64 + sum_vals[i - 1];
    }
    for _ in 0..q {
        let l: usize = reader.read();
        let r: usize = reader.read();
        if sum_vals[r] - sum_vals[l - 1] == 0 {
            fast_writeln!("0");
            continue;
        }
        if xor_vals[r] ^ xor_vals[l - 1] != 0 {
            fast_writeln!("-1");
            continue;
        }
        if (r - l + 1) % 2 == 1 {
            fast_writeln!("1");
            continue;
        } else if (r - l + 1) == 2 {
            fast_writeln!("-1");
            continue;
        }
        if a[l] == 0 || a[r] == 0 {
           fast_writeln!("1");
            continue;
        }
        if let Some(tree) = map.get(&(xor_vals[l - 1], l % 2)) {
            if let Some(_) = tree.range(l + 2..r).next() {
                fast_writeln!("2");
                continue;
            }
        }
        fast_writeln!("-1");
    }

    writer.flush();
    Ok(())
}
