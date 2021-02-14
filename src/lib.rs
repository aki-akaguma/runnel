//!
//! The pluggable io stream. now support: stdio, string io, in memory pipe.
//!
//! # Examples
//!
//! Example of stdio :
//! ```rust
//! use runnel::medium::stdio::{StdErr, StdIn, StdOut};
//! use runnel::StreamIoe;
//!
//! let sioe = StreamIoe {
//!     pin: Box::new(StdIn::default()),
//!     pout: Box::new(StdOut::default()),
//!     perr: Box::new(StdErr::default()),
//! };
//! ```
//! Example of stringio :
//! ```rust
//! use runnel::StreamIoe;
//! use runnel::medium::stringio::{StringErr, StringIn, StringOut};
//! use std::io::{BufRead, Write};
//!
//! let sioe = StreamIoe {
//!     pin: Box::new(StringIn::with_str("ABCDE\nefgh\n")),
//!     pout: Box::new(StringOut::default()),
//!     perr: Box::new(StringErr::default()),
//! };
//!
//! // pluggable stream in
//! let mut lines_iter = sioe.pin.lock().lines().map(|l| l.unwrap());
//! assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
//! assert_eq!(lines_iter.next(), Some(String::from("efgh")));
//! assert_eq!(lines_iter.next(), None);
//!
//! // pluggable stream out
//! #[rustfmt::skip]
//! let res = sioe.pout.lock()
//!     .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
//! assert!(res.is_ok());
//! assert_eq!(sioe.pout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
//!
//! // pluggable stream err
//! #[rustfmt::skip]
//! let res = sioe.perr.lock()
//!     .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
//! assert!(res.is_ok());
//! assert_eq!(sioe.perr.lock().buffer_str(), "1234\nACBDE\nefgh\n");
//! ```
//! Example of pipeio :
//! ```rust
//! use runnel::medium::pipeio::pipe;
//! use runnel::medium::stringio::{StringErr, StringIn, StringOut};
//! use runnel::StreamIoe;
//! use std::io::{BufRead, Write};
//!
//! // create in memory pipe
//! let (a_out, a_in) = pipe(1);
//!
//! // a working thread
//! let sioe = StreamIoe {
//!     pin: Box::new(StringIn::with_str("ABCDE\nefgh\n")),
//!     pout: Box::new(a_out), // pluggable pipe out
//!     perr: Box::new(StringErr::default()),
//! };
//! let handler = std::thread::spawn(move || {
//!     for line in sioe.pin.lock().lines().map(|l| l.unwrap()) {
//!         let mut out = sioe.pout.lock();
//!         out.write_fmt(format_args!("{}\n", line)).unwrap();
//!         out.flush().unwrap();
//!     }
//! });
//!
//! // a main thread
//! let sioe = StreamIoe {
//!     pin: Box::new(a_in), // pluggable pipe in
//!     pout: Box::new(StringOut::default()),
//!     perr: Box::new(StringErr::default()),
//! };
//! let mut lines_iter = sioe.pin.lock().lines().map(|l| l.unwrap());
//! assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
//! assert_eq!(lines_iter.next(), Some(String::from("efgh")));
//! assert_eq!(lines_iter.next(), None);
//!
//! assert!(handler.join().is_ok());
//! ```
//!
pub mod medium;

use std::fmt::Debug;
use std::io::{BufRead, Read, Write};
use std::panic::{RefUnwindSafe, UnwindSafe};

//----------------------------------------------------------------------
/// The set of `StreamIn`, `StreamOut`, `StreamErr`.
#[derive(Debug)]
pub struct StreamIoe {
    /// pluggable stream in
    pub pin: Box<dyn StreamIn>,
    /// pluggable stream out
    pub pout: Box<dyn StreamOut>,
    /// pluggable stream err
    pub perr: Box<dyn StreamErr>,
}

/// A stream in
pub trait StreamIn: Send + Sync + UnwindSafe + RefUnwindSafe + Debug {
    fn lock(&self) -> Box<dyn StreamInLock + '_>;
}
/// A locked reference to `StreamIn`
pub trait StreamInLock: Read + BufRead {}

/// A stream out
pub trait StreamOut: Send + Sync + UnwindSafe + RefUnwindSafe + Debug {
    fn lock(&self) -> Box<dyn StreamOutLock + '_>;
}
/// A locked reference to `StreamOut`
pub trait StreamOutLock: Write {
    fn buffer(&self) -> &[u8];
    fn buffer_str(&self) -> &str;
}

/// A stream err
pub trait StreamErr: Send + Sync + UnwindSafe + RefUnwindSafe + Debug {
    fn lock(&self) -> Box<dyn StreamErrLock + '_>;
}
/// A locked reference to `StreamErr`
pub trait StreamErrLock: Write {
    fn buffer(&self) -> &[u8];
    fn buffer_str(&self) -> &str;
}
