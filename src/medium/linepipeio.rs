//!
//! The in-memory fifo text line stream, like linux pipe. You can use for communication between threads.
//!
//! This idea is to make line-by-line processing more efficient.
//!
use crate::*;

use std::io::{BufRead, Read, Write};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Mutex, MutexGuard};

//----------------------------------------------------------------------
/// create in-memory fifo text line stream and return ([`LinePipeOut`], [`LinePipeIn`]).
///
/// [`LinePipeOut`]: LinePipeOut
/// [`LinePipeIn`]: LinePipeIn
///
#[inline(always)]
pub fn line_pipe(sz: usize) -> (LinePipeOut, LinePipeIn) {
    let (sender, receiver) = std::sync::mpsc::sync_channel(sz);
    (LinePipeOut::with(sender), LinePipeIn::with(receiver))
}

trait WriteString {
    fn write_line(&mut self, string: String) -> Result<()>;
    fn flush_line(&mut self) -> Result<()>;
}

const MSG_CHUNK_SZ: usize = 2 * 512;

//----------------------------------------------------------------------
//{{{ impl StreamIn
/// The in-memory fifo line buffer input stream.
#[derive(Debug)]
pub struct LinePipeIn(LockableLinePipeIn);
impl LinePipeIn {
    pub fn with(a: Receiver<Vec<String>>) -> Self {
        Self(LockableLinePipeIn::with(a))
    }
}
impl StreamIn for LinePipeIn {
    #[inline(always)]
    fn lock_bufread(&self) -> Box<dyn BufRead + '_> {
        unimplemented!()
    }
    #[inline(always)]
    fn is_line_pipe(&self) -> bool {
        true
    }
    fn lines(&self) -> Box<dyn NextLine + '_> {
        let a = self.0.inner.lock().unwrap().take().unwrap();
        Box::new(Lines { buf: a })
    }
}

/// A locked reference to `LinePipeIn`
#[allow(dead_code)]
pub struct LinePipeInLock<'a>(LockableLinePipeInLock<'a>);
impl Read for LinePipeInLock<'_> {
    #[inline(always)]
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        //self.0.read(buf)
        unimplemented!()
    }
}
impl BufRead for LinePipeInLock<'_> {
    #[inline(always)]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        //self.0.fill_buf()
        unimplemented!()
    }
    #[inline(always)]
    fn consume(&mut self, _amt: usize) {
        //self.0.consume(amt)
        unimplemented!()
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ impl StreamOut
/// The in-memory fifo line string buffer output stream.
#[derive(Debug)]
pub struct LinePipeOut(LockableLinePipeOut);
impl LinePipeOut {
    pub fn with(sender: SyncSender<Vec<String>>) -> Self {
        Self(LockableLinePipeOut::with(RawLinePipeOut::with(sender)))
    }
}
impl StreamOut for LinePipeOut {
    #[inline(always)]
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        unimplemented!()
        //Box::new(LinePipeOutLock(self.0.lock()))
    }
    #[inline(always)]
    fn is_line_pipe(&self) -> bool {
        true
    }
    fn write_line(&self, string: String) -> Result<()> {
        self.0.lock().write_line(string)
    }
    fn flush_line(&self) -> Result<()> {
        self.0.lock().flush_line()
    }
}

/// A locked reference to `LinePipeOut`
#[derive(Debug)]
pub struct LinePipeOutLock<'a>(LockableLinePipeOutLock<'a>);
impl StreamOutLock for LinePipeOutLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        unimplemented!()
    }
}
impl Write for LinePipeOutLock<'_> {
    #[inline(always)]
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
impl WriteString for LinePipeOutLock<'_> {
    #[inline(always)]
    fn write_line(&mut self, string: String) -> std::io::Result<()> {
        self.0.write_line(string)
    }
    #[inline(always)]
    fn flush_line(&mut self) -> std::io::Result<()> {
        self.0.flush_line()
    }
}
//}}}

//----------------------------------------------------------------------
//{{{ impl StreamErr
/// The in-memory fifo line string buffer error stream.
#[derive(Debug)]
pub struct LinePipeErr(LockableLinePipeOut);
impl LinePipeErr {
    pub fn with(sender: SyncSender<Vec<String>>) -> Self {
        Self(LockableLinePipeOut::with(RawLinePipeOut::with(sender)))
    }
}
impl StreamErr for LinePipeErr {
    #[inline(always)]
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        unimplemented!()
    }
    #[inline(always)]
    fn is_line_pipe(&self) -> bool {
        true
    }
    fn write_line(&self, string: String) -> Result<()> {
        self.0.lock().write_line(string)
    }
    fn flush_line(&self) -> Result<()> {
        self.0.lock().flush_line()
    }
}

impl std::convert::From<LinePipeOut> for LinePipeErr {
    #[inline(always)]
    fn from(a: LinePipeOut) -> Self {
        Self(a.0)
    }
}

/// A locked reference to `LinePipeErr`
#[allow(dead_code)]
#[derive(Debug)]
pub struct LinePipeErrLock<'a>(LockableLinePipeOutLock<'a>);
impl StreamErrLock for LinePipeErrLock<'_> {
    #[inline(always)]
    fn buffer(&self) -> &[u8] {
        unimplemented!()
    }
}
impl Write for LinePipeErrLock<'_> {
    #[inline(always)]
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
//}}}

//----------------------------------------------------------------------

#[derive(Debug)]
struct LockableLinePipeIn {
    inner: Mutex<Option<RawLinePipeIn>>,
}
impl LockableLinePipeIn {
    pub fn with(a: Receiver<Vec<String>>) -> Self {
        LockableLinePipeIn {
            inner: Mutex::new(Some(RawLinePipeIn::new(a))),
        }
    }
}

#[derive(Debug)]
struct LockableLinePipeInLock<'a> {
    _inner: MutexGuard<'a, Option<RawLinePipeIn>>,
}
impl Read for LockableLinePipeInLock<'_> {
    #[inline(always)]
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
}
impl BufRead for LockableLinePipeInLock<'_> {
    #[inline(always)]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        //self.inner.as_mut().unwrap().fill_buf()
        unimplemented!()
    }
    #[inline(always)]
    fn consume(&mut self, _amt: usize) {
        //self.inner.as_mut().unwrap().consume(amt)
        unimplemented!()
    }
}

#[derive(Debug)]
struct LockableLinePipeOut {
    inner: Mutex<RawLinePipeOut>,
}
impl LockableLinePipeOut {
    fn with(a: RawLinePipeOut) -> Self {
        LockableLinePipeOut {
            inner: Mutex::new(a),
        }
    }
    pub fn lock(&self) -> LockableLinePipeOutLock<'_> {
        LockableLinePipeOutLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}

#[derive(Debug)]
struct LockableLinePipeOutLock<'a> {
    inner: MutexGuard<'a, RawLinePipeOut>,
}
impl Write for LockableLinePipeOutLock<'_> {
    #[inline(always)]
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!()
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!()
    }
}
impl WriteString for LockableLinePipeOutLock<'_> {
    #[inline(always)]
    fn write_line(&mut self, string: String) -> std::io::Result<()> {
        self.inner.write_line(string)
    }
    #[inline(always)]
    fn flush_line(&mut self) -> std::io::Result<()> {
        self.inner.flush_line()
    }
}

pub struct Lines {
    buf: RawLinePipeIn,
}
impl Iterator for Lines {
    type Item = Result<String>;
    fn next(&mut self) -> Option<Result<String>> {
        self.buf.next()
    }
}
impl NextLine for Lines {}

#[derive(Debug)]
struct RawLinePipeIn {
    buf: Vec<String>,
    receiver: Receiver<Vec<String>>,
}
impl RawLinePipeIn {
    fn new(a: Receiver<Vec<String>>) -> Self {
        Self {
            buf: Vec::with_capacity(MSG_CHUNK_SZ),
            receiver: a,
        }
    }
    fn next(&mut self) -> Option<Result<String>> {
        if self.buf.is_empty() {
            let mut b = match self.receiver.recv() {
                Ok(s) => s,
                Err(_) => return None,
            };
            b.reverse();
            self.buf = b;
        }
        Some(Ok(self.buf.pop().unwrap()))
    }
}
impl Read for RawLinePipeIn {
    #[inline(always)]
    fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
        unimplemented!();
    }
}
impl BufRead for RawLinePipeIn {
    #[inline(always)]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        unimplemented!();
    }
    #[inline(always)]
    fn consume(&mut self, _amt: usize) {
        unimplemented!();
    }
}

#[derive(Debug)]
struct RawLinePipeOut {
    buf: Vec<String>,
    sender: SyncSender<Vec<String>>,
}
impl RawLinePipeOut {
    pub fn with(a: SyncSender<Vec<String>>) -> Self {
        Self {
            buf: Vec::new(),
            sender: a,
        }
    }
}
impl Write for RawLinePipeOut {
    #[inline(always)]
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        unimplemented!();
    }
    #[inline(always)]
    fn flush(&mut self) -> std::io::Result<()> {
        unimplemented!();
    }
}
impl WriteString for RawLinePipeOut {
    fn write_line(&mut self, string: String) -> Result<()> {
        self.buf.push(string);
        if self.buf.len() > MSG_CHUNK_SZ {
            self.flush_line()?;
        }
        Ok(())
    }
    fn flush_line(&mut self) -> Result<()> {
        let mut v = Vec::with_capacity(self.buf.len());
        v.append(&mut self.buf); // move String instance
        let r = self.sender.send(v);
        if let Err(err) = r {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err));
        }
        self.buf.clear();
        Ok(())
    }
}
