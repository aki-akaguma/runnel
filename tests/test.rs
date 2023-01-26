mod test_runnel {
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::{RunnelIoe, RunnelIoeBuilder};
    //
    #[test]
    fn test_size() {
        #[cfg(target_arch="x86_64")]
        {
            assert_eq!(std::mem::size_of::<RunnelIoe>(), 48);
            assert_eq!(std::mem::size_of::<RunnelIoeBuilder>(), 48);
        }
        #[cfg(target_arch="x86")]
        {
            assert_eq!(std::mem::size_of::<RunnelIoe>(), 24);
            assert_eq!(std::mem::size_of::<RunnelIoeBuilder>(), 24);
        }
    }
    #[test]
    fn test_debug_runnel_ioe() {
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str("ABCDE\nefgh\n")),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let s = format!("{:?}", sioe);
        //
        let t = concat!(
            "RunnelIoe {",
            " pin: StringIn(LockableStringIn {",
            " inner: Mutex { data: BufReader { reader: RawStringIn {",
            " buf: \"ABCDE\\nefgh\\n\", pos: 0, amt: 0 }, buffer: 0/1024 },",
            " poisoned: false, .. } }),",
            " pout: StringOut(LockableStringOut {",
            " inner: Mutex { data: RawStringOut { buf: \"\" },",
            " poisoned: false, .. } }),",
            " perr: StringErr(LockableStringOut {",
            " inner: Mutex { data: RawStringOut { buf: \"\" },",
            " poisoned: false, .. } }) }",
        );
        assert_eq!(s, t);
    }
    #[test]
    fn test_debug_runnel_ioe_builder() {
        let sioe = RunnelIoeBuilder::new()
            .pin(StringIn::with_str("ABCDE\nefgh\n"))
            .build();
        let s = format!("{:?}", sioe);
        let t = concat!(
            "RunnelIoe {",
            " pin: StringIn(LockableStringIn {",
            " inner: Mutex { data: BufReader {",
            " reader: RawStringIn {",
            " buf: \"ABCDE\\nefgh\\n\", pos: 0, amt: 0 },",
            " buffer: 0/1024 },",
            " poisoned: false, .. } }),",
            " pout: StdOut(Stdout { .. }),",
            " perr: StdErr(Stderr { .. }) }",
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
                " pin: StdIn(Stdin { .. }),",
                " pout: StdOut(Stdout { .. }),",
                " perr: StdErr(Stderr { .. }) }",
            )
        );
    }
    #[test]
    fn test_stringio() {
        use std::io::{BufRead, Write};
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n").build();
        // pluggable stream in
        let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        //
        // pluggable stream out
        #[rustfmt::skip]
        let res = sioe.pout().lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sioe.pout().lock().buffer_str(), "1234\nACBDE\nefgh\n");
        //
        // pluggable stream err
        #[rustfmt::skip]
        let res = sioe.perr().lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sioe.perr().lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
    #[test]
    fn test_pipeio() {
        use runnel::medium::pipeio::pipe;
        use std::io::{BufRead, Write};
        // create in memory pipe
        let (a_out, a_in) = pipe(1);
        //
        // a working thread
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pout(a_out)    // pluggable pipe out
            .build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
                let mut out = sioe.pout().lock();
                out.write_fmt(format_args!("{}\n", line)).unwrap();
                out.flush().unwrap();
            }
        });
        //
        // a main thread
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("")
            .pin(a_in)      // pluggable pipe out
            .build();
        let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        //
        assert!(handler.join().is_ok());
    }
}
