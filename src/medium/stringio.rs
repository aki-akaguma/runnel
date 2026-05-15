//!
//! The string buffer stream. You can use for test.
//!
use crate::*;

use std::io::{BufRead, BufReader, Read, Write};
use std::sync::{Mutex, MutexGuard};

//----------------------------------------------------------------------
//{{{ impl StreamIn
/// The string buffer input stream.
#[derive(Debug, Default)]
pub struct StringIn(LockableStringIn);
impl StringIn {
    pub fn with(a_string: String) -> Self {
        Self(LockableStringIn::with(a_string))
    }
    pub fn with_str(a_str: &str) -> Self {
        Self(LockableStringIn::with(a_str.to_string()))
    }
}
impl StreamIn for StringIn {
    fn lock_bufread(&self) -> Box<dyn BufRead + '_> {
        Box::new(StringInLock(self.0.lock()))
    }
    fn is_line_pipe(&self) -> bool {
        false
    }
    fn lines(&self) -> Box<dyn NextLine + '_> {
        Box::new(Lines {
            buf: self.lock_bufread().lines(),
        })
    }
}

/// A locked reference to `StringIn`
#[derive(Debug)]
pub struct StringInLock<'a>(LockableStringInLock<'a>);
impl Read for StringInLock<'_> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl BufRead for StringInLock<'_> {
    #[inline]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf()
    }
    #[inline]
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ impl StreamOut
/// The string buffer output stream.
#[derive(Debug, Default)]
pub struct StringOut(LockableStringOut);
impl StringOut {}
impl StreamOut for StringOut {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(StringOutLock(self.0.lock()))
    }
    fn is_line_pipe(&self) -> bool {
        false
    }
    fn write_line(&self, string: String) -> Result<()> {
        self.lock().write_fmt(format_args!("{}\n", string))
    }
    fn flush_line(&self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StringOut`
#[derive(Debug)]
pub struct StringOutLock<'a>(LockableStringOutLock<'a>);
impl StreamOutLock for StringOutLock<'_> {
    #[inline]
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
}
impl Write for StringOutLock<'_> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ impl StreamErr
/// The string buffer err stream.
#[derive(Debug, Default)]
pub struct StringErr(LockableStringOut);
impl StringErr {}
impl StreamErr for StringErr {
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(StringErrLock(self.0.lock()))
    }
    fn is_line_pipe(&self) -> bool {
        false
    }
    fn write_line(&self, string: String) -> Result<()> {
        self.lock().write_fmt(format_args!("{}\n", string))
    }
    fn flush_line(&self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StringErr`
#[derive(Debug)]
pub struct StringErrLock<'a>(LockableStringOutLock<'a>);
impl StreamErrLock for StringErrLock<'_> {
    #[inline]
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
}
impl Write for StringErrLock<'_> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
//}}}

//----------------------------------------------------------------------
const LINE_BUF_SIZE: usize = 1024;

#[derive(Debug)]
struct LockableStringIn {
    inner: Mutex<BufReader<RawStringIn>>,
}
impl LockableStringIn {
    pub fn with(a_string: String) -> Self {
        LockableStringIn {
            inner: Mutex::new(BufReader::with_capacity(
                LINE_BUF_SIZE,
                RawStringIn::new(a_string),
            )),
        }
    }
    pub fn lock(&self) -> LockableStringInLock<'_> {
        LockableStringInLock {
            inner: self.inner.lock_any(),
        }
    }
}
impl Default for LockableStringIn {
    fn default() -> Self {
        Self::with("".to_string())
    }
}

#[derive(Debug)]
struct LockableStringInLock<'a> {
    inner: MutexGuard<'a, BufReader<RawStringIn>>,
}
impl Read for LockableStringInLock<'_> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
impl BufRead for LockableStringInLock<'_> {
    #[inline]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }
    #[inline]
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

#[derive(Debug)]
struct LockableStringOut {
    inner: Mutex<RawStringOut>,
}
impl LockableStringOut {
    fn with(a: RawStringOut) -> Self {
        LockableStringOut {
            inner: Mutex::new(a),
        }
    }
    pub fn lock(&self) -> LockableStringOutLock<'_> {
        LockableStringOutLock {
            inner: self.inner.lock_any(),
        }
    }
}
impl Default for LockableStringOut {
    fn default() -> Self {
        Self::with(RawStringOut::default())
    }
}

#[derive(Debug)]
struct LockableStringOutLock<'a> {
    inner: MutexGuard<'a, RawStringOut>,
}
impl LockableStringOutLock<'_> {
    #[inline]
    pub fn buffer(&self) -> &[u8] {
        self.inner.buffer()
    }
}
impl Write for LockableStringOutLock<'_> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

pub struct Lines<'a> {
    buf: std::io::Lines<Box<dyn BufRead + 'a>>,
}
impl<'a> Iterator for Lines<'a> {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Result<String>> {
        self.buf.next()
    }
}
impl<'a> NextLine for Lines<'a> {}

#[derive(Debug)]
struct RawStringIn {
    buf: String,
    pos: usize,
}
impl RawStringIn {
    fn new(a_string: String) -> Self {
        Self {
            buf: a_string,
            pos: 0,
        }
    }
}
impl Read for RawStringIn {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.fill_buf()?.read(buf)?;
        self.consume(n);
        Ok(n)
    }
}
impl BufRead for RawStringIn {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(&self.buf.as_bytes()[self.pos..])
    }
    #[inline]
    fn consume(&mut self, amt: usize) {
        self.pos = std::cmp::min(self.pos + amt, self.buf.len());
    }
}

#[derive(Debug, Default)]
struct RawStringOut {
    buf: String,
}
impl RawStringOut {
    #[inline]
    pub fn buffer(&self) -> &[u8] {
        self.buf.as_bytes()
    }
}
impl Write for RawStringOut {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let src = String::from_utf8_lossy(buf);
        self.buf.push_str(&src);
        Ok(src.len())
    }
    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

//----------------------------------------------------------------------
#[cfg(test)]
mod test_stringio {
    use super::{LockableStringIn, LockableStringOut};
    use std::io::BufRead;
    use std::io::Write;
    #[test]
    fn test_in() {
        let sin = LockableStringIn::with("ABCDE\nefgh\n".to_string());
        let mut lines_iter = sin.lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
    }
    #[test]
    fn test_out() {
        let sout = LockableStringOut::default();
        let res = sout
            .lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sout.lock().buffer(), b"1234\nACBDE\nefgh\n");
    }
}
