#[cfg(test)]
mod test_runnel {
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::{RunnelIoe, RunnelIoeBuilder};
    //
    #[test]
    fn test_debug_runnel_ioe() {
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str("ABCDE\nefgh\n")),
            Box::<StringOut>::default(),
            Box::<StringErr>::default(),
        );
        let s = format!("{:?}", sioe);
        //
        let t = concat!(
            "RunnelIoe {",
            " pg_in: StringIn(LockableStringIn {",
            " inner: Mutex { data: Some(BufReader { reader: RawStringIn {",
            " buf: \"ABCDE\\nefgh\\n\", pos: 0, amt: 0 }, buffer: 0/1024 }),",
            " poisoned: false, .. } }),",
            " pg_out: StringOut(LockableStringOut {",
            " inner: Mutex { data: RawStringOut { buf: \"\" },",
            " poisoned: false, .. } }),",
            " pg_err: StringErr(LockableStringOut {",
            " inner: Mutex { data: RawStringOut { buf: \"\" },",
            " poisoned: false, .. } }) }",
        );
        assert_eq!(s, t);
    }
    #[test]
    fn test_debug_runnel_ioe_builder() {
        let sioe = RunnelIoeBuilder::new()
            .pg_in(StringIn::with_str("ABCDE\nefgh\n"))
            .build();
        let s = format!("{:?}", sioe);
        let t = concat!(
            "RunnelIoe {",
            " pg_in: StringIn(LockableStringIn {",
            " inner: Mutex { data: Some(BufReader {",
            " reader: RawStringIn {",
            " buf: \"ABCDE\\nefgh\\n\", pos: 0, amt: 0 },",
            " buffer: 0/1024 }),",
            " poisoned: false, .. } }),",
            " pg_out: StdOut(Stdout { .. }),",
            " pg_err: StdErr(Stderr { .. }) }",
        );
        assert_eq!(s, t);
    }
    #[test]
    fn test_stdio() {
        let sioe = RunnelIoeBuilder::new().build();
        let s = format!("{:?}", sioe);
        assert_eq!(
            s,
            concat!(
                "RunnelIoe {",
                " pg_in: StdIn(Stdin { .. }),",
                " pg_out: StdOut(Stdout { .. }),",
                " pg_err: StdErr(Stderr { .. }) }",
            )
        );
    }
    #[test]
    fn test_stringio() {
        use std::io::Write;
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n").build();
        // pluggable stream in
        let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        //
        // pluggable stream out
        #[rustfmt::skip]
        let res = sioe.pg_out().lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(
            sioe.pg_out().lock().buffer_to_string(),
            "1234\nACBDE\nefgh\n"
        );
        //
        // pluggable stream err
        #[rustfmt::skip]
        let res = sioe.pg_err().lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(
            sioe.pg_err().lock().buffer_to_string(),
            "1234\nACBDE\nefgh\n"
        );
    }
    #[test]
    fn test_pipeio() {
        use runnel::medium::pipeio::pipe;
        use std::io::Write;
        // create in memory pipe
        let (a_out, a_in) = pipe(1);
        //
        // a working thread
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_out(a_out)    // pluggable pipe out
            .build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                let mut out = sioe.pg_out().lock();
                out.write_fmt(format_args!("{}\n", line)).unwrap();
                out.flush().unwrap();
            }
        });
        //
        // a main thread
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("")
            .pg_in(a_in)      // pluggable pipe out
            .build();
        let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        //
        assert!(handler.join().is_ok());
    }
}
