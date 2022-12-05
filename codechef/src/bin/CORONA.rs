#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{
    fmt::Debug,
    io::{BufReader, BufWriter, Read, Write},
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

struct Writer<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> Writer<W> {
    fn new(inner: W) -> Self {
        Self {
            writer: BufWriter::new(inner),
        }
    }

    fn get_mut_inner(&mut self) -> &'_ mut BufWriter<W> {
        &mut self.writer
    }

    fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
}

static DIRS4: [i32; 5] = [-1, 0, 1, 0, -1];
static DIRS8: [i32; 9] = [-1, -1, 0, -1, 1, 0, 1, 1, -1];

use std::collections::BinaryHeap;

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    let input = input.lock();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
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
        let m: usize = reader.read();
        let mut gph = vec![vec![]; n + 1];
        let mut heap = BinaryHeap::new();
        let mut ans = vec![i64::MIN; n + 1];
        for _ in 0..reader.read() {
            let x: usize = reader.read();
            let c: i64 = reader.read();
            heap.push((-c, x));
            ans[x] = -c;
        }
        for _ in 0..m {
            let u: usize = reader.read();
            let v: usize = reader.read();
            let c: i64 = reader.read();
            gph[u].push((v, -c));
            gph[v].push((u, -c));
        }
        while let Some((val, cur)) = heap.pop() {
            if val < ans[cur] {
                continue;
            }
            for &(nxt, cost) in &gph[cur] {
                if val + cost > ans[nxt] {
                    heap.push((val + cost, nxt));
                    ans[nxt] = val + cost;
                }
            }
        }
        for i in 1..=n {
            fast_write!("{}", -ans[i]);
            fast_write!(" ");
        }
        fast_writeln!();
    }

    writer.flush();
    Ok(())
}
