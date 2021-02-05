//! The pluggable io stream.
//! now support: stdio, string io, in memory pipe.
pub mod medium;

use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::panic::{RefUnwindSafe, UnwindSafe};

//----------------------------------------------------------------------
/// set of stream in, out and err.
pub struct StreamIoe {
    pub sin: Box<dyn StreamIn>,
    pub sout: Box<dyn StreamOut>,
    pub serr: Box<dyn StreamErr>,
}

/// stream in
pub trait StreamIn: Send + Sync + UnwindSafe + RefUnwindSafe {
    fn lock(&self) -> Box<dyn StreamInLock + '_>;
}
/// stream in lock
pub trait StreamInLock: Read + BufRead {}

/// stream out
pub trait StreamOut: Send + Sync + UnwindSafe + RefUnwindSafe {
    fn lock(&self) -> Box<dyn StreamOutLock + '_>;
}
/// stream out lock
pub trait StreamOutLock: Write {
    fn buffer(&self) -> &[u8];
    fn buffer_str(&self) -> &str;
}

/// stream err
pub trait StreamErr: Send + Sync + UnwindSafe + RefUnwindSafe {
    fn lock(&self) -> Box<dyn StreamErrLock + '_>;
}
/// stream err lock
pub trait StreamErrLock: Write {
    fn buffer(&self) -> &[u8];
    fn buffer_str(&self) -> &str;
}
