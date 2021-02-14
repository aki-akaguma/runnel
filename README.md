# runnel

*runnel* is the pluggable io stream. now support: stdio, string io, in memory pipe.

## Features

- support common operation: stdin, stdout, stderr, stringin, stringout, pipein and pipeout.
- thin interface
- support testing stream io

## Examples

Example of stdio :
```rust
use runnel::medium::stdio::{StdErr, StdIn, StdOut};
use runnel::StreamIoe;

let sioe = StreamIoe {
    pin: Box::new(StdIn::default()),
    pout: Box::new(StdOut::default()),
    perr: Box::new(StdErr::default()),
};
```

Example of stringio :
```rust
use runnel::StreamIoe;
use runnel::medium::stringio::{StringErr, StringIn, StringOut};
use std::io::{BufRead, Write};

let sioe = StreamIoe {
    pin: Box::new(StringIn::with_str("ABCDE\nefgh\n")),
    pout: Box::new(StringOut::default()),
    perr: Box::new(StringErr::default()),
};

// pluggable stream in
let mut lines_iter = sioe.pin.lock().lines().map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
assert_eq!(lines_iter.next(), Some(String::from("efgh")));
assert_eq!(lines_iter.next(), None);

// pluggable stream out
#[rustfmt::skip]
let res = sioe.pout.lock()
    .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
assert!(res.is_ok());
assert_eq!(sioe.pout.lock().buffer_str(), "1234\nACBDE\nefgh\n");

// pluggable stream err
#[rustfmt::skip]
let res = sioe.perr.lock()
    .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
assert!(res.is_ok());
assert_eq!(sioe.perr.lock().buffer_str(), "1234\nACBDE\nefgh\n");
```

Example of pipeio :
```rust
use runnel::medium::pipeio::pipe;
use runnel::medium::stringio::{StringErr, StringIn, StringOut};
use runnel::StreamIoe;
use std::io::{BufRead, Write};

// create in memory pipe
let (a_out, a_in) = pipe(1);

// a working thread
let sioe = StreamIoe {
    pin: Box::new(StringIn::with_str("ABCDE\nefgh\n")),
    pout: Box::new(a_out), // pluggable pipe out
    perr: Box::new(StringErr::default()),
};
let handler = std::thread::spawn(move || {
    for line in sioe.pin.lock().lines().map(|l| l.unwrap()) {
        let mut out = sioe.pout.lock();
        out.write_fmt(format_args!("{}\n", line)).unwrap();
        out.flush().unwrap();
    }
});

// a main thread
let sioe = StreamIoe {
    pin: Box::new(a_in), // pluggable pipe in
    pout: Box::new(StringOut::default()),
    perr: Box::new(StringErr::default()),
};
let mut lines_iter = sioe.pin.lock().lines().map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
assert_eq!(lines_iter.next(), Some(String::from("efgh")));
assert_eq!(lines_iter.next(), None);

assert!(handler.join().is_ok());
```
