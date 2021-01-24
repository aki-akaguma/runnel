use crate::*;

use std::io::BufReader;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{Mutex, MutexGuard};

use std::io::BufRead;
use std::io::Read;
use std::io::Write;

//----------------------------------------------------------------------
pub fn pipe(sz: usize) -> (StreamOutPipeOut, StreamInPipeIn) {
    let (sender, receiver) = std::sync::mpsc::sync_channel(sz);
    (
        StreamOutPipeOut::with(sender),
        StreamInPipeIn::with(receiver),
    )
}

//{{{ StreamIn
pub struct StreamInPipeIn(PipeIn);
impl StreamInPipeIn {
    pub fn new(a: PipeIn) -> Self {
        Self(a)
    }
    pub fn with(a: Receiver<String>) -> Self {
        Self::new(PipeIn::with(a))
    }
}
impl StreamIn for StreamInPipeIn {
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(StreamInLockPipeIn(self.0.lock()))
    }
}

pub struct StreamInLockPipeIn<'a>(PipeInLock<'a>);
impl<'a> StreamInLock for StreamInLockPipeIn<'a> {}
impl<'a> Read for StreamInLockPipeIn<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl<'a> BufRead for StreamInLockPipeIn<'a> {
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
pub struct StreamOutPipeOut(PipeOut);
impl StreamOutPipeOut {
    pub fn new(a: PipeOut) -> Self {
        Self(a)
    }
    pub fn with(sender: SyncSender<String>) -> Self {
        Self::new(PipeOut::with(PipeOutRaw::with(sender)))
    }
}
impl StreamOut for StreamOutPipeOut {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(StreamOutLockPipeOut(self.0.lock()))
    }
}

pub struct StreamOutLockPipeOut<'a>(PipeOutLock<'a>);
impl<'a> StreamOutLock for StreamOutLockPipeOut<'a> {
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    fn buffer_str(&self) -> &str {
        self.0.buffer_str()
    }
}
impl<'a> Write for StreamOutLockPipeOut<'a> {
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
pub struct StreamErrPipeErr(PipeOut);
impl StreamErrPipeErr {
    pub fn new(a: PipeOut) -> Self {
        Self(a)
    }
    pub fn with(sender: SyncSender<String>) -> Self {
        Self::new(PipeOut::with(PipeOutRaw::with(sender)))
    }
}
impl StreamErr for StreamErrPipeErr {
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(StreamErrLockPipeErr(self.0.lock()))
    }
}
impl std::convert::From<StreamOutPipeOut> for StreamErrPipeErr {
    fn from(a: StreamOutPipeOut) -> Self {
        Self::new(a.0)
    }
}

pub struct StreamErrLockPipeErr<'a>(PipeOutLock<'a>);
impl<'a> StreamErrLock for StreamErrLockPipeErr<'a> {
    fn buffer(&self) -> &[u8] {
        self.0.buffer()
    }
    fn buffer_str(&self) -> &str {
        self.0.buffer_str()
    }
}
impl<'a> Write for StreamErrLockPipeErr<'a> {
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

pub struct PipeIn {
    inner: Mutex<BufReader<PipeInRaw>>,
}
impl PipeIn {
    pub fn with(a: Receiver<String>) -> Self {
        PipeIn {
            inner: Mutex::new(BufReader::with_capacity(LINE_BUF_SIZE, PipeInRaw::new(a))),
        }
    }
    pub fn lock(&self) -> PipeInLock<'_> {
        PipeInLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}

pub struct PipeInLock<'a> {
    inner: MutexGuard<'a, BufReader<PipeInRaw>>,
}
impl<'a> Read for PipeInLock<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}
impl<'a> BufRead for PipeInLock<'a> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.inner.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

pub struct PipeOut {
    inner: Mutex<PipeOutRaw>,
}
impl PipeOut {
    fn with(a: PipeOutRaw) -> Self {
        PipeOut {
            inner: Mutex::new(a),
        }
    }
    pub fn lock(&self) -> PipeOutLock<'_> {
        PipeOutLock {
            inner: self.inner.lock().unwrap_or_else(|e| e.into_inner()),
        }
    }
}

pub struct PipeOutLock<'a> {
    inner: MutexGuard<'a, PipeOutRaw>,
}
impl<'a> PipeOutLock<'a> {
    pub fn buffer(&self) -> &[u8] {
        self.inner.buffer()
    }
    pub fn buffer_str(&self) -> &str {
        self.inner.buffer_str()
    }
}
impl<'a> Write for PipeOutLock<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.inner.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

struct PipeInRaw {
    buf: String,
    pos: usize,
    amt: usize,
    reciever: Receiver<String>,
}

impl PipeInRaw {
    fn new(a: Receiver<String>) -> Self {
        Self {
            buf: String::new(),
            pos: 0,
            amt: 0,
            reciever: a,
        }
    }
}

impl Read for PipeInRaw {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.buf.is_empty() {
            self.buf = match self.reciever.recv() {
                Ok(s) => s,
                Err(_) => return Ok(0),
            };
        }
        //
        let len = {
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
            len
        };
        //
        if self.pos >= self.buf.as_bytes().len() {
            self.buf.clear();
            self.pos = 0;
            self.amt = 0;
        }
        //
        Ok(len)
    }
}
impl BufRead for PipeInRaw {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if self.pos >= self.buf.as_bytes().len() {
            self.buf.clear();
            self.pos = 0;
            self.amt = 0;
        }
        if self.buf.is_empty() {
            self.buf = self.reciever.recv().unwrap();
        }
        //
        let src = {
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
            src
        };
        //
        Ok(src)
    }
    fn consume(&mut self, amt: usize) {
        self.amt = amt;
    }
}

struct PipeOutRaw {
    buf: String,
    sender: SyncSender<String>,
}
impl PipeOutRaw {
    pub fn with(a: SyncSender<String>) -> Self {
        Self {
            buf: String::new(),
            sender: a,
        }
    }
    pub fn buffer(&self) -> &[u8] {
        self.buf.as_bytes()
    }
    pub fn buffer_str(&self) -> &str {
        self.buf.as_str()
    }
}
impl Write for PipeOutRaw {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let src = String::from_utf8_lossy(buf).to_string();
        self.buf.push_str(&src);
        Ok(src.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.sender.send(self.buf.clone()).unwrap();
        self.buf.clear();
        Ok(())
    }
}
