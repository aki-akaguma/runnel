/*!
The pluggable io stream. now support: stdio, string io, in memory pipe, in memory line pipe.

# Features

- support common operation: stdin, stdout, stderr, stringin, stringout, pipein, pipeout, linepipein and linepipeout.
- thin interface
- support testing io stream
- minimum support rustc 1.60.0 (7737e0b5c 2022-04-04)

# Examples

## Example of stdio :

```rust
use runnel::RunnelIoeBuilder;
let sioe = RunnelIoeBuilder::new().build();
```

## Example of stringio :

```rust
use runnel::RunnelIoeBuilder;
use std::io::{BufRead, Write};

let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .build();

// pluggable input stream
let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
assert_eq!(lines_iter.next(), Some(String::from("efgh")));
assert_eq!(lines_iter.next(), None);

// pluggable output stream
#[rustfmt::skip]
let res = sioe.pg_out().lock()
    .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
assert!(res.is_ok());
assert_eq!(sioe.pg_out().lock().buffer_to_string(), "1234\nACBDE\nefgh\n");

// pluggable error stream
#[rustfmt::skip]
let res = sioe.pg_err().lock()
    .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
assert!(res.is_ok());
assert_eq!(sioe.pg_err().lock().buffer_to_string(), "1234\nACBDE\nefgh\n");
```

## Example of pipeio :

```rust
use runnel::RunnelIoeBuilder;
use runnel::medium::pipeio::pipe;
use std::io::{BufRead, Write};

// create in memory pipe
let (a_out, a_in) = pipe(1);

// a working thread
let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .pg_out(a_out)    // pluggable pipe out
    .build();
let handler = std::thread::spawn(move || {
    for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
        let mut out = sioe.pg_out().lock();
        let _ = out.write_fmt(format_args!("{}\n", line));
        let _ = out.flush();
    }
});

// a main thread
let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .pg_in(a_in)      // pluggable pipe in
    .build();
let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
assert_eq!(lines_iter.next(), Some(String::from("efgh")));
assert_eq!(lines_iter.next(), None);

assert!(handler.join().is_ok());
```

## Example of linepipeio :

```rust
use runnel::RunnelIoeBuilder;
use runnel::medium::linepipeio::line_pipe;
use std::io::{BufRead, Write};

// create in memory line pipe
let (a_out, a_in) = line_pipe(1);

// a working thread
let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .pg_out(a_out)    // pluggable pipe out
    .build();
let handler = std::thread::spawn(move || {
    for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
        let _ = sioe.pg_out().write_line(line);
        let _ = sioe.pg_out().flush_line();
    }
});

// a main thread
let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .pg_in(a_in)      // pluggable pipe in
    .build();
let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
assert_eq!(lines_iter.next(), Some(String::from("efgh")));
assert_eq!(lines_iter.next(), None);

assert!(handler.join().is_ok());
```
*/
pub mod medium;

use std::borrow::Borrow;
use std::fmt::Debug;
use std::io::{BufRead, Result, Write};
use std::panic::{RefUnwindSafe, UnwindSafe};

//----------------------------------------------------------------------
/// An iterator over the lines of a stream.
pub trait NextLine: Iterator<Item = Result<String>> {}

//----------------------------------------------------------------------
/// A trait for readable streams.
pub trait StreamIn: Send + Sync + UnwindSafe + RefUnwindSafe + Debug {
    /// Locks the stream and returns a `sync::MutexGuard` object with a trait
    /// `io::BufRead`.
    fn lock_bufread(&self) -> Box<dyn BufRead + '_>;

    /// Returns true if the stream is a line pipe.
    fn is_line_pipe(&self) -> bool;

    /// Returns an iterator over the lines of the stream.
    /// The iterator returned from this function will yield instances of
    /// `io::Result<String>`. Each string returned will *not* have a newline
    /// byte (the `0xA` byte) or `CRLF` (`0xD`, `0xA` bytes) at the end.
    /// This behaves the same as `std::io::BufRead::lines()`.
    fn lines(&self) -> Box<dyn NextLine + '_>;
}

/// A trait for writable streams.
pub trait StreamOut: Send + Sync + UnwindSafe + RefUnwindSafe + Debug {
    /// Locks the stream and returns a `sync::MutexGuard` object with a trait
    /// `StreamOutLock` object.
    fn lock(&self) -> Box<dyn StreamOutLock + '_>;

    /// Returns true if the stream is a line pipe.
    fn is_line_pipe(&self) -> bool;

    /// Writes a line to the stream.
    /// Each string should *not* have a newline byte(the `0xA` byte) or
    /// `CRLF` (`0xD`, `0xA` bytes) at the end.
    fn write_line(&self, string: String) -> Result<()>;

    /// Flushes the stream.
    fn flush_line(&self) -> Result<()>;
}

/// A locked reference to a `StreamOut` object.
pub trait StreamOutLock: Write {
    /// Returns the buffer of the stream.
    fn buffer(&self) -> &[u8];

    /// Returns the buffer of the stream as a string.
    fn buffer_to_string(&self) -> String {
        String::from_utf8_lossy(self.buffer()).to_string()
    }
}

/// A trait for writable error streams.
pub trait StreamErr: Send + Sync + UnwindSafe + RefUnwindSafe + Debug {
    /// Locks the stream and returns a `sync::MutexGuard` object with a trait
    /// `StreamErrLock` object.
    fn lock(&self) -> Box<dyn StreamErrLock + '_>;

    /// Returns true if the stream is a line pipe.
    fn is_line_pipe(&self) -> bool;

    /// Writes a line to the stream.
    /// Each string should *not* have a newline byte(the `0xA` byte) or
    /// `CRLF` (`0xD`, `0xA` bytes) at the end.
    fn write_line(&self, string: String) -> Result<()>;

    /// Flushes the stream.
    fn flush_line(&self) -> Result<()>;
}

/// A locked reference to a `StreamErr` object.
pub trait StreamErrLock: Write {
    /// Returns the buffer of the stream.
    fn buffer(&self) -> &[u8];

    /// Returns the buffer of the stream as a string.
    fn buffer_to_string(&self) -> String {
        String::from_utf8_lossy(self.buffer()).to_string()
    }
}

//----------------------------------------------------------------------
/// A struct that holds the three streams.
#[derive(Debug)]
pub struct RunnelIoe {
    pg_in: Box<dyn StreamIn>,
    pg_out: Box<dyn StreamOut>,
    pg_err: Box<dyn StreamErr>,
}

impl RunnelIoe {
    /// Creates a new `RunnelIoe` object.
    pub fn new(
        a_in: Box<dyn StreamIn>,
        a_out: Box<dyn StreamOut>,
        a_err: Box<dyn StreamErr>,
    ) -> RunnelIoe {
        RunnelIoe {
            pg_in: a_in,
            pg_out: a_out,
            pg_err: a_err,
        }
    }
    /// Returns a reference to the input stream. This is a pluggable input stream.
    pub fn pg_in(&self) -> &dyn StreamIn {
        self.pg_in.borrow()
    }
    /// Returns a reference to the output stream. This is a pluggable output stream.
    pub fn pg_out(&self) -> &dyn StreamOut {
        self.pg_out.borrow()
    }
    /// Returns a reference to the error stream. This is a pluggable error stream.
    pub fn pg_err(&self) -> &dyn StreamErr {
        self.pg_err.borrow()
    }
}

//----------------------------------------------------------------------
/// The builder of RunnelIoe
///
/// # Examples
///
/// ## Example: fill stdio
///
/// build RunnelIoe has [std::io::stdin()], [std::io::stdout()], [std::io::stderr()],
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
/// let sioe = RunnelIoeBuilder::new().build();
/// ```
///
/// ## Example: fill stringio
///
/// build RunnelIoe has [medium::stringio::StringIn],
/// [medium::stringio::StringOut], [medium::stringio::StringErr],
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
/// use runnel::medium::stringio::{StringIn, StringOut, StringErr};
/// let sioe = RunnelIoeBuilder::new()
///     .pg_in(StringIn::with_str("abcdefg"))
///     .pg_out(StringOut::default())
///     .pg_err(StringErr::default())
///     .build();
/// ```
///
/// ## Example: fill stringio by fill_stringio_with_str()
///
/// build RunnelIoe has [medium::stringio::StringIn],
/// [medium::stringio::StringOut], [medium::stringio::StringErr],
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
/// let sioe = RunnelIoeBuilder::new()
///     .fill_stringio_with_str("abcdefg")
///     .build();
/// ```
///
/// ## Example: stdio and pipe
///
/// This case is multi-threads.
/// read stdin on working thread, write stdout on main thread.
/// The data is through in-memory [pipe].
///
/// [pipe]: medium::pipeio::pipe
///
/// ```rust
/// use runnel::RunnelIoeBuilder;
/// use runnel::medium::pipeio::pipe;
/// use std::io::{BufRead, Write};
///
/// fn run() -> std::io::Result<()> {
///     let (a_out, a_in) = pipe(1);
///
///     // a working thread
///     #[rustfmt::skip]
///     let sioe = RunnelIoeBuilder::new().pg_out(a_out).build();
///     let handler = std::thread::spawn(move || {
///         for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
///             let mut out = sioe.pg_out().lock();
///             out.write_fmt(format_args!("{}\n", line)).unwrap();
///             out.flush().unwrap();
///         }
///     });
///
///     // a main thread
///     #[rustfmt::skip]
///     let sioe = RunnelIoeBuilder::new().pg_in(a_in).build();
///     for line in sioe.pg_in().lines() {
///         let line_s = line?;
///         let mut out = sioe.pg_out().lock();
///         out.write_fmt(format_args!("{}\n", line_s))?;
///         out.flush()?;
///     }
///     Ok(())
/// }
/// ```
///
#[derive(Debug)]
pub struct RunnelIoeBuilder {
    pg_in: Option<Box<dyn StreamIn>>,
    pg_out: Option<Box<dyn StreamOut>>,
    pg_err: Option<Box<dyn StreamErr>>,
}

impl RunnelIoeBuilder {
    /// create builder
    pub fn new() -> Self {
        RunnelIoeBuilder {
            pg_in: None,
            pg_out: None,
            pg_err: None,
        }
    }
    /// set pluggable input stream
    pub fn pg_in<T: 'static + StreamIn>(mut self, a: T) -> Self {
        self.pg_in = Some(Box::new(a));
        self
    }
    /// set pluggable output stream
    pub fn pg_out<T: 'static + StreamOut>(mut self, a: T) -> Self {
        self.pg_out = Some(Box::new(a));
        self
    }
    /// set pluggable error stream
    pub fn pg_err<T: 'static + StreamErr>(mut self, a: T) -> Self {
        self.pg_err = Some(Box::new(a));
        self
    }
    /// build to RunnelIoe
    pub fn build(self) -> RunnelIoe {
        let a_in = if let Some(a) = self.pg_in {
            a
        } else {
            Box::<medium::stdio::StdIn>::default()
        };
        let a_out = if let Some(a) = self.pg_out {
            a
        } else {
            Box::<medium::stdio::StdOut>::default()
        };
        let a_err = if let Some(a) = self.pg_err {
            a
        } else {
            Box::<medium::stdio::StdErr>::default()
        };
        RunnelIoe::new(a_in, a_out, a_err)
    }
    /// fill with stringio, arg as input
    pub fn fill_stringio_with_str(self, arg: &str) -> Self {
        use crate::medium::stringio::*;
        self.pg_in(StringIn::with_str(arg))
            .pg_out(StringOut::default())
            .pg_err(StringErr::default())
    }
    /// fill with stringio, arg as input
    pub fn fill_stringio(self, arg: String) -> Self {
        use crate::medium::stringio::*;
        self.pg_in(StringIn::with(arg))
            .pg_out(StringOut::default())
            .pg_err(StringErr::default())
    }
}

impl Default for RunnelIoeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
