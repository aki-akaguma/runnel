use crate::*;

use std::io::BufRead;
use std::io::Read;
use std::io::Write;

//----------------------------------------------------------------------
//{{{ StreamIn
pub struct StreamInStdin(std::io::Stdin);
impl StreamInStdin {
    pub fn new(a: std::io::Stdin) -> Self {
        Self(a)
    }
}
impl Default for StreamInStdin {
    fn default() -> Self {
        Self::new(std::io::stdin())
    }
}
impl StreamIn for StreamInStdin {
    fn lock(&self) -> Box<dyn StreamInLock + '_> {
        Box::new(StreamInLockStdin(self.0.lock()))
    }
}

pub struct StreamInLockStdin<'a>(std::io::StdinLock<'a>);
impl<'a> StreamInLock for StreamInLockStdin<'a> {}
impl<'a> Read for StreamInLockStdin<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl<'a> BufRead for StreamInLockStdin<'a> {
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
pub struct StreamOutStdout(std::io::Stdout);
impl StreamOutStdout {
    pub fn new(a: std::io::Stdout) -> Self {
        Self(a)
    }
}
impl Default for StreamOutStdout {
    fn default() -> Self {
        Self::new(std::io::stdout())
    }
}
impl StreamOut for StreamOutStdout {
    fn lock(&self) -> Box<dyn StreamOutLock + '_> {
        Box::new(StreamOutLockStdout(self.0.lock()))
    }
}

pub struct StreamOutLockStdout<'a>(std::io::StdoutLock<'a>);
impl<'a> StreamOutLock for StreamOutLockStdout<'a> {
    fn buffer(&self) -> &[u8] {
        b""
    }
    fn buffer_str(&self) -> &str {
        ""
    }
}
impl<'a> Write for StreamOutLockStdout<'a> {
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
pub struct StreamErrStderr(std::io::Stderr);
impl StreamErrStderr {
    pub fn new(a: std::io::Stderr) -> Self {
        Self(a)
    }
}
impl Default for StreamErrStderr {
    fn default() -> Self {
        Self::new(std::io::stderr())
    }
}
impl StreamErr for StreamErrStderr {
    fn lock(&self) -> Box<dyn StreamErrLock + '_> {
        Box::new(StreamErrLockStderr(self.0.lock()))
    }
}

pub struct StreamErrLockStderr<'a>(std::io::StderrLock<'a>);
impl<'a> StreamErrLock for StreamErrLockStderr<'a> {
    fn buffer(&self) -> &[u8] {
        b""
    }
    fn buffer_str(&self) -> &str {
        ""
    }
}
impl<'a> Write for StreamErrLockStderr<'a> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}
//}}}
