pub mod medium;

use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::panic::{RefUnwindSafe, UnwindSafe};

//----------------------------------------------------------------------
pub struct StreamIoe {
    pub sin: Box<dyn StreamIn>,
    pub sout: Box<dyn StreamOut>,
    pub serr: Box<dyn StreamErr>,
}

pub trait StreamIn: Send + Sync + UnwindSafe + RefUnwindSafe {
    fn lock(&self) -> Box<dyn StreamInLock + '_>;
}
pub trait StreamInLock: Read + BufRead {}

pub trait StreamOut: Send + Sync + UnwindSafe + RefUnwindSafe {
    fn lock(&self) -> Box<dyn StreamOutLock + '_>;
}
pub trait StreamOutLock: Write {
    fn buffer(&self) -> &[u8];
    fn buffer_str(&self) -> &str;
}

pub trait StreamErr: Send + Sync + UnwindSafe + RefUnwindSafe {
    fn lock(&self) -> Box<dyn StreamErrLock + '_>;
}
pub trait StreamErrLock: Write {
    fn buffer(&self) -> &[u8];
    fn buffer_str(&self) -> &str;
}
