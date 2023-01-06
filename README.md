# runnel

The pluggable io stream. now support: stdio, string io, in memory pipe.

## Features

- support common operation: stdin, stdout, stderr, stringin, stringout, pipein and pipeout.
- thin interface
- support testing stream io
- minimum support rustc 1.57.0 (f1edd0429 2021-11-29)

## Examples

### Example of stdio :

```rust
use runnel::RunnelIoeBuilder;
let sioe = RunnelIoeBuilder::new().build();
```

### Example of stringio :

```rust
use runnel::RunnelIoeBuilder;
use std::io::{BufRead, Write};

let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .build();

// pluggable stream in
let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
assert_eq!(lines_iter.next(), Some(String::from("efgh")));
assert_eq!(lines_iter.next(), None);

// pluggable stream out
#[rustfmt::skip]
let res = sioe.pout().lock()
    .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
assert!(res.is_ok());
assert_eq!(sioe.pout().lock().buffer_str(), "1234\nACBDE\nefgh\n");

// pluggable stream err
#[rustfmt::skip]
let res = sioe.perr().lock()
    .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
assert!(res.is_ok());
assert_eq!(sioe.perr().lock().buffer_str(), "1234\nACBDE\nefgh\n");
```

### Example of pipeio :

```rust
use runnel::RunnelIoeBuilder;
use runnel::medium::pipeio::pipe;
use std::io::{BufRead, Write};

// create in memory pipe
let (a_out, a_in) = pipe(1);

// a working thread
let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .pout(a_out)    // pluggable pipe out
    .build();
let handler = std::thread::spawn(move || {
    for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
        let mut out = sioe.pout().lock();
        let _ = out.write_fmt(format_args!("{}\n", line));
        let _ = out.flush();
    }
});

// a main thread
let sioe = RunnelIoeBuilder::new()
    .fill_stringio_with_str("ABCDE\nefgh\n")
    .pin(a_in)      // pluggable pipe in
    .build();
let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
assert_eq!(lines_iter.next(), Some(String::from("efgh")));
assert_eq!(lines_iter.next(), None);

assert!(handler.join().is_ok());
```

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/runnel/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
