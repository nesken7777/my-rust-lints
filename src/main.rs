//! 全部の思想lintに従った結果がこれ

use std::io::{self, stdout, Write};

fn print<S: AsRef<str>>(s: S) -> io::Result<()> {
    fn print_internal(s: &str) -> io::Result<()> {
        stdout().write_all(s.as_bytes())
    }
    print_internal(s.as_ref())
}

fn main() -> io::Result<()> {
    print("Hello, world!")
}
