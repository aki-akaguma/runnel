//!
//! The line buffer stream. Currently under planning.
//!
//! This idea is to make line-by-line processing more efficient.
//!
use crate::*;

use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::sync::{Mutex, MutexGuard};

//----------------------------------------------------------------------
//{{{ impl StreamIn
/// The line buffer input stream.
#[derive(Debug, Default)]
pub struct LineIn(LockableLineIn);
impl LineIn {}
impl StreamIn for LineIn {
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(LineInLock(self.0.lock()))
    }
}
impl Read for LineIn {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.lock().read(buf)
    }
}

/// A locked reference to `LineIn`
pub struct LineInLock<'a>(LockableLineInLock<'a>);
impl StreamInLock for LineInLock<'_> {}
impl Read for LineInLock<'_> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl BufRead for LineInLock<'_> {
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
/// The line buffer output stream.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct LineOut(LockableLineOut);
impl LineOut {}
impl StreamOut for LineOut {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(LineOutLock(self.0.lock()))
    }
}
impl Write for LineOut {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `LineOut`
#[allow(dead_code)]
pub struct LineOutLock<'a>(LockableLineOutLock<'a>);
impl StreamOutLock for LineOutLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        b""
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        ""
    }
}
impl Write for LineOutLock<'_> {
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
/// The line buffer error stream.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct LineErr(LockableLineErr);
impl LineErr {}
impl StreamErr for LineErr {
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(LineErrLock(self.0.lock()))
    }
}
impl Write for LineErr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `LineErr`
#[allow(dead_code)]
pub struct LineErrLock<'a>(LockableLineErrLock<'a>);
impl StreamErrLock for LineErrLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        b""
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        ""
    }
}
impl Write for LineErrLock<'_> {
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
struct LockableLineIn {
    inner: Mutex<BufReader<RawLineIn>>,
}
impl LockableLineIn {
    pub fn new() -> Self {
        LockableLineIn {
            inner: Mutex::new(BufReader::with_capacity(
                LINE_BUF_SIZE,
                RawLineIn::default(),
            )),
        }
    }
    pub fn lock(&self) -> LockableLineInLock<'_> {
        LockableLineInLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}
impl Default for LockableLineIn {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct LockableLineInLock<'a> {
    inner: MutexGuard<'a, BufReader<RawLineIn>>,
}
impl Read for LockableLineInLock<'_> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
impl BufRead for LockableLineInLock<'_> {
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
struct LockableLineOut {
    inner: Mutex<BufWriter<RawLineOut>>,
}
impl LockableLineOut {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(BufWriter::with_capacity(
                LINE_BUF_SIZE,
                RawLineOut::default(),
            )),
        }
    }
    pub fn lock(&self) -> LockableLineOutLock<'_> {
        LockableLineOutLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}
impl Default for LockableLineOut {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct LockableLineOutLock<'a> {
    inner: MutexGuard<'a, BufWriter<RawLineOut>>,
}
impl Write for LockableLineOutLock<'_> {
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
struct LockableLineErr {
    inner: Mutex<BufWriter<RawLineErr>>,
}
impl LockableLineErr {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(BufWriter::with_capacity(
                LINE_BUF_SIZE,
                RawLineErr::default(),
            )),
        }
    }
    pub fn lock(&self) -> LockableLineErrLock<'_> {
        LockableLineErrLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}
impl Default for LockableLineErr {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct LockableLineErrLock<'a> {
    inner: MutexGuard<'a, BufWriter<RawLineErr>>,
}
impl Write for LockableLineErrLock<'_> {
    #[inline(always)]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

#[derive(Debug, Default)]
struct RawLineIn {
    buf: Vec<u8>,
}
#[derive(Debug, Default)]
struct RawLineOut {}
#[derive(Debug, Default)]
struct RawLineErr {}

impl Read for RawLineIn {
    #[inline(always)]
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        //unimplemented!();
        Ok(0)
    }
}
impl BufRead for RawLineIn {
    #[inline(always)]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        //unimplemented!();
        Ok(&self.buf)
    }
    #[inline(always)]
    fn consume(&mut self, _amt: usize) {
        //unimplemented!();
    }
}

impl Write for RawLineOut {
    #[inline(always)]
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        //unimplemented!();
        Ok(0)
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        //unimplemented!();
        Ok(())
    }
}
impl Write for RawLineErr {
    #[inline(always)]
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        //unimplemented!();
        Ok(0)
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        //unimplemented!();
        Ok(())
    }
}
