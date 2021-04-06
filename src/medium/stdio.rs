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
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(StdInLock(self.0.lock()))
    }
}
impl Read for StdIn {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.lock().read(buf)
    }
}

/// A locked reference to `StdIn`
pub struct StdInLock<'a>(std::io::StdinLock<'a>);
impl<'a> StreamInLock for StdInLock<'a> {}
impl<'a> Read for StdInLock<'a> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl<'a> BufRead for StdInLock<'a> {
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
}
impl Write for StdOut {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StdOut`
pub struct StdOutLock<'a>(std::io::StdoutLock<'a>);
impl<'a> StreamOutLock for StdOutLock<'a> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        b""
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        ""
    }
}
impl<'a> Write for StdOutLock<'a> {
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
}
impl Write for StdErr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StdErr`
pub struct StdErrLock<'a>(std::io::StderrLock<'a>);
impl<'a> StreamErrLock for StdErrLock<'a> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        b""
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        ""
    }
}
impl<'a> Write for StdErrLock<'a> {
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
