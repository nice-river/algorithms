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

static DIRS4: [i32; 5] = [-1, 0, 1, 0, -1];
static DIRS8: [i32; 9] = [-1, -1, 0, -1, 1, 0, 1, 1, -1];

fn main() -> std::io::Result<()> {
    let input = std::io::stdin();
    #[cfg(feature = "local")]
    let input = std::fs::File::open("src/input.txt")?;
    let mut reader = Reader::new(input);

    for _ in 0..reader.read() {
        let n: usize = reader.read();
        let mut gph = vec![vec![]; n + 1];
        for _ in 0..reader.read() {
            let u: usize = reader.read();
            let v: usize = reader.read();
            gph[u].push(v);
            gph[v].push(u);
        }
        let mut p = 0;
        let mut s = 0;
        let mut es = (0, 0);
        for i in 1..=n {
            s += gph[i].len();
            if gph[i].len() % 2 == 1 {
                p = i;
            } else if gph[i].len() > 0 {
                es = (i, gph[i][0]);
            }
        }
        if s / 2 % 2 == 0 {
            println!("1");
            for _ in 1..=n {
                print!("1 ");
            }
        } else if p != 0 {
            println!("2");
            for i in 1..=n {
                if i == p {
                    print!("2 ");
                } else {
                    print!("1 ");
                }
            }
        } else {
            println!("3");
            for i in 1..=n {
                if i == es.0 {
                    print!("2 ");
                } else if i == es.1 {
                    print!("3 ");
                } else {
                    print!("1 ");
                }
            }
        }
        println!();
    }

    Ok(())
}
