//!
//! The pluggable io stream. now support: stdio, string io, in memory pipe.
//!
//! # Examples
//!
//! ## Example of stdio :
//!
//! ```rust
//! use runnel::RunnelIoeBuilder;
//! let sioe = RunnelIoeBuilder::new().build();
//! ```
//!
//! ## Example of stringio :
//!
//! ```rust
//! use runnel::RunnelIoeBuilder;
//! use std::io::{BufRead, Write};
//!
//! let sioe = RunnelIoeBuilder::new()
//!     .fill_stringio_wit_str("ABCDE\nefgh\n")
//!     .build();
//!
//! // pluggable stream in
//! let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
//! assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
//! assert_eq!(lines_iter.next(), Some(String::from("efgh")));
//! assert_eq!(lines_iter.next(), None);
//!
//! // pluggable stream out
//! #[rustfmt::skip]
//! let res = sioe.pout().lock()
//!     .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
//! assert!(res.is_ok());
//! assert_eq!(sioe.pout().lock().buffer_str(), "1234\nACBDE\nefgh\n");
//!
//! // pluggable stream err
//! #[rustfmt::skip]
//! let res = sioe.perr().lock()
//!     .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
//! assert!(res.is_ok());
//! assert_eq!(sioe.perr().lock().buffer_str(), "1234\nACBDE\nefgh\n");
//! ```
//!
//! ## Example of pipeio :
//!
//! ```rust
//! use runnel::RunnelIoeBuilder;
//! use runnel::medium::pipeio::pipe;
//! use runnel::medium::stringio::{StringErr, StringIn, StringOut};
//! use std::io::{BufRead, Write};
//!
//! // create in memory pipe
//! let (a_out, a_in) = pipe(1);
//!
//! // a working thread
//! let sioe = RunnelIoeBuilder::new()
//!     .fill_stringio_wit_str("ABCDE\nefgh\n")
//!     .pout(a_out)    // pluggable pipe out
//!     .build();
//! let handler = std::thread::spawn(move || {
//!     for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
//!         let mut out = sioe.pout().lock();
//!         out.write_fmt(format_args!("{}\n", line)).unwrap();
//!         out.flush().unwrap();
//!     }
//! });
//!
//! // a main thread
//! let sioe = RunnelIoeBuilder::new()
//!     .fill_stringio_wit_str("ABCDE\nefgh\n")
//!     .pin(a_in)      // pluggable pipe in
//!     .build();
//! let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
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

//----------------------------------------------------------------------
/// The set of `StreamIn`, `StreamOut`, `StreamErr`.
#[derive(Debug)]
pub struct RunnelIoe {
    pin: Box<dyn StreamIn>,
    pout: Box<dyn StreamOut>,
    perr: Box<dyn StreamErr>,
}

/// auto flush on drop: pout and perr.
impl std::ops::Drop for RunnelIoe {
    fn drop(&mut self) {
        let _ = self.pout.lock().flush();
        let _ = self.perr.lock().flush();
    }
}

impl RunnelIoe {
    /// create RunnelIoe. use [RunnelIoeBuilder].
    pub fn new(
        a_in: Box<dyn StreamIn>,
        a_out: Box<dyn StreamOut>,
        a_err: Box<dyn StreamErr>,
    ) -> RunnelIoe {
        RunnelIoe {
            pin: a_in,
            pout: a_out,
            perr: a_err,
        }
    }
    /// get pluggable stream in
    pub fn pin(&self) -> &Box<dyn StreamIn> {
        &self.pin
    }
    /// get pluggable stream out
    pub fn pout(&self) -> &Box<dyn StreamOut> {
        &self.pout
    }
    /// get pluggable stream err
    pub fn perr(&self) -> &Box<dyn StreamErr> {
        &self.perr
    }
}

/// The builder for RunnelIoe
///
/// # Examples
///
/// ## Example: fill stdio
///
/// build RunnelIoe has std::in::stdin(), std::in::stdout(), std::in::stderr(),
/// ```rust
/// use runnel::RunnelIoeBuilder;
/// let sioe = RunnelIoeBuilder::new().build();
/// ```
///
/// ## Example: fill stringio
///
/// build RunnelIoe has medium::stringio,
/// ```rust
/// use runnel::RunnelIoeBuilder;
/// use runnel::medium::stringio::{StringIn, StringOut, StringErr};
/// let sioe = RunnelIoeBuilder::new()
///     .pin(StringIn::with_str("abcdefg"))
///     .pout(StringOut::default())
///     .perr(StringErr::default())
///     .build();
/// ```
///
/// ## Example: fill stringio by fill_stringio_wit_str()
///
/// build RunnelIoe has medium::stringio,
/// ```rust
/// use runnel::RunnelIoeBuilder;
/// let sioe = RunnelIoeBuilder::new()
///     .fill_stringio_wit_str("abcdefg")
///     .build();
/// ```
///
pub struct RunnelIoeBuilder {
    pin: Option<Box<dyn StreamIn>>,
    pout: Option<Box<dyn StreamOut>>,
    perr: Option<Box<dyn StreamErr>>,
}

impl RunnelIoeBuilder {
    /// create builder
    pub fn new() -> Self {
        RunnelIoeBuilder {
            pin: None,
            pout: None,
            perr: None,
        }
    }
    /// set pluggable stream in
    pub fn pin<T: 'static + StreamIn>(mut self, a: T) -> Self {
        self.pin = Some(Box::new(a));
        self
    }
    /// set pluggable stream out
    pub fn pout<T: 'static + StreamOut>(mut self, a: T) -> Self {
        self.pout = Some(Box::new(a));
        self
    }
    /// set pluggable stream err
    pub fn perr<T: 'static + StreamErr>(mut self, a: T) -> Self {
        self.perr = Some(Box::new(a));
        self
    }
    /// build to RunnelIoe
    pub fn build(self) -> RunnelIoe {
        let a_in = if let Some(a) = self.pin {
            a
        } else {
            Box::new(crate::medium::stdio::StdIn::default())
        };
        let a_out = if let Some(a) = self.pout {
            a
        } else {
            Box::new(crate::medium::stdio::StdOut::default())
        };
        let a_err = if let Some(a) = self.perr {
            a
        } else {
            Box::new(crate::medium::stdio::StdErr::default())
        };
        //
        RunnelIoe::new(a_in, a_out, a_err)
    }
    /// fill with stringio, arg as input
    pub fn fill_stringio_wit_str(self, arg: &str) -> Self {
        use crate::medium::stringio::*;
        self.pin(StringIn::with_str(arg))
            .pout(StringOut::default())
            .perr(StringErr::default())
    }
    /// fill with stringio, arg as input
    pub fn fill_stringio(self, arg: String) -> Self {
        use crate::medium::stringio::*;
        self.pin(StringIn::with(arg))
            .pout(StringOut::default())
            .perr(StringErr::default())
    }
}

impl Default for RunnelIoeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
