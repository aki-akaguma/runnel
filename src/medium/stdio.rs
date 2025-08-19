//!
//! The stdio stream. This is thin-wrap of [`std::io::stdin()`],
//! [`std::io::stdout()`], [`std::io::stderr()`].
//!
use crate::*;

use std::io::{BufRead, Read, Write};

//----------------------------------------------------------------------
//{{{ impl StreamIn
/// The standard input stream.
#[derive(Debug)]
pub struct StdIn(std::io::Stdin);
impl StdIn {
    pub fn with(a: std::io::Stdin) -> Self {
        Self(a)
    }
}
impl Default for StdIn {
    fn default() -> Self {
        Self::with(std::io::stdin())
    }
}
impl StreamIn for StdIn {
    fn lock_bufread(&self) -> Box<dyn BufRead + '_> {
        Box::new(StdInLock(self.0.lock()))
    }
    fn is_line_pipe(&self) -> bool {
        false
    }
    fn lines(&self) -> Box<dyn NextLine + '_> {
        Box::new(Lines {
            buf: self.0.lock().lines(),
        })
    }
}

pub struct Lines<'a> {
    buf: std::io::Lines<std::io::StdinLock<'a>>,
}
impl<'a> Iterator for Lines<'a> {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Result<String>> {
        self.buf.next()
    }
}
impl<'a> NextLine for Lines<'a> {}

/// A locked reference to `StdIn`
pub struct StdInLock<'a>(std::io::StdinLock<'a>);
impl Read for StdInLock<'_> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl BufRead for StdInLock<'_> {
    #[inline(always)]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf()
    }
    #[inline(always)]
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ impl StreamOut
/// The standard output stream.
#[derive(Debug)]
pub struct StdOut(std::io::Stdout);
impl StdOut {
    pub fn with(a: std::io::Stdout) -> Self {
        Self(a)
    }
}
impl Default for StdOut {
    fn default() -> Self {
        Self::with(std::io::stdout())
    }
}
impl StreamOut for StdOut {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(StdOutLock(self.0.lock()))
    }
    fn is_line_pipe(&self) -> bool {
        false
    }
    fn write_line(&self, string: String) -> Result<()> {
        self.lock().write_fmt(format_args!("{string}\n"))
    }
    fn flush_line(&self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StdOut`
pub struct StdOutLock<'a>(std::io::StdoutLock<'a>);
impl StreamOutLock for StdOutLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        b""
    }
}
impl Write for StdOutLock<'_> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ impl StreamErr
/// The standard error stream.
#[derive(Debug)]
pub struct StdErr(std::io::Stderr);
impl StdErr {
    pub fn with(a: std::io::Stderr) -> Self {
        Self(a)
    }
}
impl Default for StdErr {
    fn default() -> Self {
        Self::with(std::io::stderr())
    }
}
impl StreamErr for StdErr {
    #[inline(always)]
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(StdErrLock(self.0.lock()))
    }
    fn is_line_pipe(&self) -> bool {
        false
    }
    fn write_line(&self, string: String) -> Result<()> {
        self.lock().write_fmt(format_args!("{string}\n"))
    }
    fn flush_line(&self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StdErr`
pub struct StdErrLock<'a>(std::io::StderrLock<'a>);
impl StreamErrLock for StdErrLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        b""
    }
}
impl Write for StdErrLock<'_> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
//}}}
