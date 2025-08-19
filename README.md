# runnel

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

The pluggable io stream. now support: stdio, string io, in memory pipe, in memory line pipe.

## Features

- support common operation: stdin, stdout, stderr, stringin, stringout, pipein, pipeout, linepipein and linepipeout.
- thin interface
- support testing io stream
- minimum support rustc 1.60.0 (7737e0b5c 2022-04-04)

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

### Example of linepipeio :

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

# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/runnel/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/runnel.svg
[crate-link]: https://crates.io/crates/runnel
[docs-image]: https://docs.rs/runnel/badge.svg
[docs-link]: https://docs.rs/runnel/
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/runnel/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/runnel/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/runnel/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/runnel/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/runnel/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/runnel/actions/workflows/test-windows.yml
