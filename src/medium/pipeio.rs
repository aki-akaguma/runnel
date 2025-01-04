//!
//! The in-memory fifo stream, like linux pipe. You can use for communication between threads.
//!
use crate::*;

use std::io::{BufRead, BufReader, Read, Write};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Mutex, MutexGuard};

//----------------------------------------------------------------------
/// create in-memory fifo stream and return ([`PipeOut`], [`PipeIn`]).
///
/// [`PipeOut`]: PipeOut
/// [`PipeIn`]: PipeIn
///
#[inline(always)]
pub fn pipe(sz: usize) -> (PipeOut, PipeIn) {
    let (sender, receiver) = std::sync::mpsc::sync_channel(sz);
    (PipeOut::with(sender), PipeIn::with(receiver))
}

//----------------------------------------------------------------------
//{{{ impl StreamIn
/// The in-memory fifo input stream.
#[derive(Debug)]
pub struct PipeIn(LockablePipeIn);
impl PipeIn {
    pub fn with(a: Receiver<Vec<u8>>) -> Self {
        Self(LockablePipeIn::with(a))
    }
}
impl StreamIn for PipeIn {
    #[inline(always)]
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(PipeInLock(self.0.lock()))
    }
}
impl Read for PipeIn {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.lock().read(buf)
    }
}

/// A locked reference to `PipeIn`
#[derive(Debug)]
pub struct PipeInLock<'a>(LockablePipeInLock<'a>);
impl StreamInLock for PipeInLock<'_> {}
impl Read for PipeInLock<'_> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl BufRead for PipeInLock<'_> {
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
/// The in-memory fifo output stream.
#[derive(Debug)]
pub struct PipeOut(LockablePipeOut);
impl PipeOut {
    pub fn with(sender: SyncSender<Vec<u8>>) -> Self {
        Self(LockablePipeOut::with(RawPipeOut::with(sender)))
    }
}
impl StreamOut for PipeOut {
    #[inline(always)]
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(PipeOutLock(self.0.lock()))
    }
}
impl Write for PipeOut {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}

/// A locked reference to `PipeOut`
#[derive(Debug)]
pub struct PipeOutLock<'a>(LockablePipeOutLock<'a>);
impl StreamOutLock for PipeOutLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        self.0.buffer_str()
    }
}
impl Write for PipeOutLock<'_> {
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
/// The in-memory fifo error stream.
#[derive(Debug)]
pub struct PipeErr(LockablePipeOut);
impl PipeErr {
    pub fn with(sender: SyncSender<Vec<u8>>) -> Self {
        Self(LockablePipeOut::with(RawPipeOut::with(sender)))
    }
}
impl StreamErr for PipeErr {
    #[inline(always)]
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(PipeErrLock(self.0.lock()))
    }
}
impl Write for PipeErr {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.lock().write(buf)
    }
    fn flush(&mut self) -> Result<()> {
        self.lock().flush()
    }
}
impl std::convert::From<PipeOut> for PipeErr {
    #[inline(always)]
    fn from(a: PipeOut) -> Self {
        Self(a.0)
    }
}

/// A locked reference to `PipeErr`
#[derive(Debug)]
pub struct PipeErrLock<'a>(LockablePipeOutLock<'a>);
impl StreamErrLock for PipeErrLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    #[inline(always)]
    fn buffer_str(&mut self) -> &str {
        self.0.buffer_str()
    }
}
impl Write for PipeErrLock<'_> {
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
struct LockablePipeIn {
    inner: Mutex<BufReader<RawPipeIn>>,
}
impl LockablePipeIn {
    pub fn with(a: Receiver<Vec<u8>>) -> Self {
        LockablePipeIn {
            inner: Mutex::new(BufReader::with_capacity(LINE_BUF_SIZE, RawPipeIn::new(a))),
        }
    }
    pub fn lock(&self) -> LockablePipeInLock<'_> {
        LockablePipeInLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}

#[derive(Debug)]
struct LockablePipeInLock<'a> {
    inner: MutexGuard<'a, BufReader<RawPipeIn>>,
}
impl Read for LockablePipeInLock<'_> {
    #[inline(always)]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
impl BufRead for LockablePipeInLock<'_> {
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
struct LockablePipeOut {
    inner: Mutex<RawPipeOut>,
}
impl LockablePipeOut {
    fn with(a: RawPipeOut) -> Self {
        LockablePipeOut {
            inner: Mutex::new(a),
        }
    }
    pub fn lock(&self) -> LockablePipeOutLock<'_> {
        LockablePipeOutLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}

#[derive(Debug)]
struct LockablePipeOutLock<'a> {
    inner: MutexGuard<'a, RawPipeOut>,
}
impl LockablePipeOutLock<'_> {
    #[inline(always)]
    pub fn buffer(&self) -> &[u8] {
        self.inner.buffer()
    }
    #[inline(always)]
    pub fn buffer_str(&mut self) -> &str {
        self.inner.buffer_str()
    }
}
impl Write for LockablePipeOutLock<'_> {
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
struct RawPipeIn {
    buf: Vec<u8>,
    pos: usize,
    amt: usize,
    reciever: Receiver<Vec<u8>>,
}
impl RawPipeIn {
    fn new(a: Receiver<Vec<u8>>) -> Self {
        Self {
            buf: Vec::new(),
            pos: 0,
            amt: 0,
            reciever: a,
        }
    }
}
impl Read for RawPipeIn {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buf.is_empty() {
            self.buf = match self.reciever.recv() {
                Ok(s) => s,
                Err(_) => return Ok(0),
            };
        }
        //
        let len = {
            let src = self.buf.as_slice();
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
            len
        };
        //
        if self.pos >= self.buf.as_slice().len() {
            self.buf.clear();
            self.pos = 0;
            self.amt = 0;
        }
        //
        Ok(len)
    }
}
impl BufRead for RawPipeIn {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.pos >= self.buf.as_slice().len() {
            self.buf.clear();
            self.pos = 0;
            self.amt = 0;
        }
        if self.buf.is_empty() {
            self.buf = self.reciever.recv().unwrap();
        }
        //
        let src = {
            let src = self.buf.as_slice();
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
            src
        };
        //
        Ok(src)
    }
    #[inline(always)]
    fn consume(&mut self, amt: usize) {
        self.amt = amt;
    }
}

#[derive(Debug)]
struct RawPipeOut {
    buf: Vec<u8>,
    sender: SyncSender<Vec<u8>>,
    tmp: String,
}
impl RawPipeOut {
    pub fn with(a: SyncSender<Vec<u8>>) -> Self {
        Self {
            buf: Vec::new(),
            sender: a,
            tmp: String::new(),
        }
    }
    #[inline(always)]
    pub fn buffer(&self) -> &[u8] {
        self.buf.as_slice()
    }
    #[inline(always)]
    pub fn buffer_str(&mut self) -> &str {
        self.tmp = String::from_utf8_lossy(&self.buf).to_string();
        self.tmp.as_str()
    }
}
impl Write for RawPipeOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let src_len = buf.len();
        // auto flush
        //const BUF_SZ:usize = 1 * 1024;
        const BUF_SZ: usize = 4 * 4 * 1024;
        //const BUF_SZ:usize = 8 * 4 * 1024;
        if self.buf.len() >= BUF_SZ {
            self.flush()?;
        }
        self.buf.extend_from_slice(buf);
        Ok(src_len)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        let r = self.sender.send(self.buf.clone());
        if let Err(err) = r {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
        }
        self.buf.clear();
        Ok(())
    }
}
