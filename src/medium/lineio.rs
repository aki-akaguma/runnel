use crate::*;

use std::io::BufReader;
use std::io::BufWriter;
use std::sync::{Mutex, MutexGuard};

use std::io::BufRead;
use std::io::Read;
use std::io::Write;
//use std::io::IoSliceMut;

//----------------------------------------------------------------------
//{{{ StreamIn
pub struct StreamInLineIn(LineIn);
impl StreamInLineIn {
    pub fn new(a: LineIn) -> Self {
        Self(a)
    }
}
impl Default for StreamInLineIn {
    fn default() -> Self {
        Self::new(LineIn::default())
    }
}
impl StreamIn for StreamInLineIn {
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(StreamInLockLineIn(self.0.lock()))
    }
}

pub struct StreamInLockLineIn<'a>(LineInLock<'a>);
impl<'a> StreamInLock for StreamInLockLineIn<'a> {}
impl<'a> Read for StreamInLockLineIn<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl<'a> BufRead for StreamInLockLineIn<'a> {
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
pub struct StreamOutLineOut(LineOut);
impl StreamOutLineOut {
    pub fn new(a: LineOut) -> Self {
        Self(a)
    }
}
impl StreamOut for StreamOutLineOut {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(StreamOutLockLineOut(self.0.lock()))
    }
}

pub struct StreamOutLockLineOut<'a>(LineOutLock<'a>);
impl<'a> StreamOutLock for StreamOutLockLineOut<'a> {
    fn buffer(&self) -> &[u8] {
        b""
    }
    fn buffer_str(&self) -> &str {
        ""
    }
}
impl<'a> Write for StreamOutLockLineOut<'a> {
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
pub struct StreamErrLineErr(LineErr);
impl StreamErrLineErr {
    pub fn new(a: LineErr) -> Self {
        Self(a)
    }
}
impl StreamErr for StreamErrLineErr {
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(StreamErrLockLineErr(self.0.lock()))
    }
}

pub struct StreamErrLockLineErr<'a>(LineErrLock<'a>);
impl<'a> StreamErrLock for StreamErrLockLineErr<'a> {
    fn buffer(&self) -> &[u8] {
        b""
    }
    fn buffer_str(&self) -> &str {
        ""
    }
}
impl<'a> Write for StreamErrLockLineErr<'a> {
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

pub struct LineIn {
    inner: Mutex<BufReader<LineInRaw>>,
}
impl LineIn {
    pub fn new() -> Self {
        LineIn {
            inner: Mutex::new(BufReader::with_capacity(
                LINE_BUF_SIZE,
                LineInRaw::default(),
            )),
        }
    }
    pub fn lock(&self) -> LineInLock<'_> {
        LineInLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}
impl Default for LineIn {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LineInLock<'a> {
    inner: MutexGuard<'a, BufReader<LineInRaw>>,
}
impl<'a> Read for LineInLock<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
impl<'a> BufRead for LineInLock<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

pub struct LineOut {
    inner: Mutex<BufWriter<LineOutRaw>>,
}
impl LineOut {
    pub fn lock(&self) -> LineOutLock<'_> {
        LineOutLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}

pub struct LineOutLock<'a> {
    inner: MutexGuard<'a, BufWriter<LineOutRaw>>,
}
impl<'a> Write for LineOutLock<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

pub struct LineErr {
    inner: Mutex<BufWriter<LineErrRaw>>,
}
impl LineErr {
    pub fn lock(&self) -> LineErrLock<'_> {
        LineErrLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}

pub struct LineErrLock<'a> {
    inner: MutexGuard<'a, BufWriter<LineErrRaw>>,
}
impl<'a> Write for LineErrLock<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[derive(Default)]
struct LineInRaw {
    buf: Vec<u8>,
}
#[derive(Default)]
struct LineOutRaw {}
#[derive(Default)]
struct LineErrRaw {}

impl Read for LineInRaw {
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}
impl BufRead for LineInRaw {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        Ok(&self.buf)
    }
    fn consume(&mut self, _amt: usize) {}
}

impl Write for LineOutRaw {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl Write for LineErrRaw {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
