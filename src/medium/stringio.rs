use crate::*;

use std::io::BufReader;
use std::sync::{Mutex, MutexGuard};

use std::io::BufRead;
use std::io::Read;
use std::io::Write;

//----------------------------------------------------------------------
//{{{ StreamIn
pub struct StreamInStringIn(StringIn);
impl StreamInStringIn {
    pub fn new(a: StringIn) -> Self {
        Self(a)
    }
    pub fn with(a_string: String) -> Self {
        Self::new(StringIn::with(a_string))
    }
    pub fn with_str(a_str: &str) -> Self {
        Self::new(StringIn::with(a_str.to_string()))
    }
}
impl Default for StreamInStringIn {
    fn default() -> Self {
        Self::new(StringIn::default())
    }
}
impl StreamIn for StreamInStringIn {
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(StreamInLockStringIn(self.0.lock()))
    }
}

pub struct StreamInLockStringIn<'a>(StringInLock<'a>);
impl<'a> StreamInLock for StreamInLockStringIn<'a> {}
impl<'a> Read for StreamInLockStringIn<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl<'a> BufRead for StreamInLockStringIn<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ StreamOut
pub struct StreamOutStringOut(StringOut);
impl StreamOutStringOut {
    pub fn new(a: StringOut) -> Self {
        Self(a)
    }
}
impl Default for StreamOutStringOut {
    fn default() -> Self {
        Self::new(StringOut::default())
    }
}
impl StreamOut for StreamOutStringOut {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(StreamOutLockStringOut(self.0.lock()))
    }
}

pub struct StreamOutLockStringOut<'a>(StringOutLock<'a>);
impl<'a> StreamOutLock for StreamOutLockStringOut<'a> {
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    fn buffer_str(&self) -> &str {
        self.0.buffer_str()
    }
}
impl<'a> Write for StreamOutLockStringOut<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ StreamErr
pub struct StreamErrStringErr(StringOut);
impl StreamErrStringErr {
    pub fn new(a: StringOut) -> Self {
        Self(a)
    }
}
impl Default for StreamErrStringErr {
    fn default() -> Self {
        Self::new(StringOut::default())
    }
}
impl StreamErr for StreamErrStringErr {
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(StreamErrLockStringErr(self.0.lock()))
    }
}

pub struct StreamErrLockStringErr<'a>(StringOutLock<'a>);
impl<'a> StreamErrLock for StreamErrLockStringErr<'a> {
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    fn buffer_str(&self) -> &str {
        self.0.buffer_str()
    }
}
impl<'a> Write for StreamErrLockStringErr<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
//}}}

//----------------------------------------------------------------------
const LINE_BUF_SIZE: usize = 1024;

pub struct StringIn {
    inner: Mutex<BufReader<StringInRaw>>,
}
impl StringIn {
    pub fn with(a_string: String) -> Self {
        StringIn {
            inner: Mutex::new(BufReader::with_capacity(
                LINE_BUF_SIZE,
                StringInRaw::new(a_string),
            )),
        }
    }
    pub fn lock(&self) -> StringInLock<'_> {
        StringInLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}
impl Default for StringIn {
    fn default() -> Self {
        Self::with("".to_string())
    }
}

pub struct StringInLock<'a> {
    inner: MutexGuard<'a, BufReader<StringInRaw>>,
}
impl<'a> Read for StringInLock<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
impl<'a> BufRead for StringInLock<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

pub struct StringOut {
    inner: Mutex<StringOutRaw>,
}
impl StringOut {
    fn with(a: StringOutRaw) -> Self {
        StringOut {
            inner: Mutex::new(a),
        }
    }
    pub fn lock(&self) -> StringOutLock<'_> {
        StringOutLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}
impl Default for StringOut {
    fn default() -> Self {
        Self::with(StringOutRaw::default())
    }
}

pub struct StringOutLock<'a> {
    inner: MutexGuard<'a, StringOutRaw>,
}
impl<'a> StringOutLock<'a> {
    pub fn buffer(&self) -> &[u8] {
        self.inner.buffer()
    }
    pub fn buffer_str(&self) -> &str {
        self.inner.buffer_str()
    }
}
impl<'a> Write for StringOutLock<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

struct StringInRaw {
    buf: String,
    pos: usize,
    amt: usize,
}

impl StringInRaw {
    fn new(a_string: String) -> Self {
        Self {
            buf: a_string,
            pos: 0,
            amt: 0,
        }
    }
}

impl Read for StringInRaw {
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
impl BufRead for StringInRaw {
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
    fn consume(&mut self, amt: usize) {
        self.amt = amt;
    }
}

#[derive(Default)]
struct StringOutRaw {
    buf: String,
}
impl StringOutRaw {
    pub fn buffer(&self) -> &[u8] {
        self.buf.as_bytes()
    }
    pub fn buffer_str(&self) -> &str {
        self.buf.as_str()
    }
}
impl Write for StringOutRaw {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let src = String::from_utf8_lossy(buf).to_string();
        self.buf.push_str(&src);
        Ok(src.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
