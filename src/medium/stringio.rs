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
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(StringInLock(self.0.lock()))
    }
}
impl Read for StringIn {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.lock().read(buf)
    }
}

/// A locked reference to `StringIn`
#[derive(Debug)]
pub struct StringInLock<'a>(LockableStringInLock<'a>);
impl<'a> StreamInLock for StringInLock<'a> {}
impl<'a> Read for StringInLock<'a> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl<'a> BufRead for StringInLock<'a> {
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
/// The string buffer output stream.
#[derive(Debug, Default)]
pub struct StringOut(LockableStringOut);
impl StringOut {}
impl StreamOut for StringOut {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(StringOutLock(self.0.lock()))
    }
}
impl Write for StringOut {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StringOut`
#[derive(Debug)]
pub struct StringOutLock<'a>(LockableStringOutLock<'a>);
impl<'a> StreamOutLock for StringOutLock<'a> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        self.0.buffer_str()
    }
}
impl<'a> Write for StringOutLock<'a> {
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
/// The string buffer err stream.
#[derive(Debug, Default)]
pub struct StringErr(LockableStringOut);
impl StringErr {}
impl StreamErr for StringErr {
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(StringErrLock(self.0.lock()))
    }
}
impl Write for StringErr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `StringErr`
#[derive(Debug)]
pub struct StringErrLock<'a>(LockableStringOutLock<'a>);
impl<'a> StreamErrLock for StringErrLock<'a> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        self.0.buffer_str()
    }
}
impl<'a> Write for StringErrLock<'a> {
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
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
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
impl<'a> Read for LockableStringInLock<'a> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
impl<'a> BufRead for LockableStringInLock<'a> {
    #[inline(always)]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }
    #[inline(always)]
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
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
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
impl<'a> LockableStringOutLock<'a> {
    #[inline(always)]
    pub fn buffer(&self) -> &[u8] {
        self.inner.buffer()
    }
    #[inline(always)]
    pub fn buffer_str(&self) -> &str {
        self.inner.buffer_str()
    }
}
impl<'a> Write for LockableStringOutLock<'a> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[derive(Debug)]
struct RawStringIn {
    buf: String,
    pos: usize,
    amt: usize,
}
impl RawStringIn {
    fn new(a_string: String) -> Self {
        Self {
            buf: a_string,
            pos: 0,
            amt: 0,
        }
    }
}
impl Read for RawStringIn {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let src = self.buf.as_bytes();
        let src_len = src.len() - self.pos;
        let dst_len = buf.len();
        //
        let (len, dst, src) = if src_len >= dst_len {
            let len = dst_len;
            (len, buf, &src[self.pos..(self.pos + len)])
        } else {
            let len = src_len;
            (len, &mut buf[0..len], &src[self.pos..(self.pos + len)])
        };
        dst.copy_from_slice(src);
        self.pos += len;
        //
        Ok(len)
    }
}
impl BufRead for RawStringIn {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        let src = self.buf.as_bytes();
        let src_len = src.len() - self.pos;
        let dst_len = self.amt;
        //
        let (len, src) = if src_len >= dst_len {
            let len = dst_len;
            (len, &src[self.pos..(self.pos + len)])
        } else {
            let len = src_len;
            (len, &src[self.pos..(self.pos + len)])
        };
        self.pos += len;
        //
        Ok(src)
    }
    #[inline(always)]
    fn consume(&mut self, amt: usize) {
        self.amt = amt;
    }
}

#[derive(Debug, Default)]
struct RawStringOut {
    buf: String,
}
impl RawStringOut {
    #[inline(always)]
    pub fn buffer(&self) -> &[u8] {
        self.buf.as_bytes()
    }
    #[inline(always)]
    pub fn buffer_str(&self) -> &str {
        self.buf.as_str()
    }
}
impl Write for RawStringOut {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let src = String::from_utf8_lossy(buf).to_string();
        self.buf.push_str(&src);
        Ok(src.len())
    }
    #[inline(always)]
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
        assert_eq!(sout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
}
