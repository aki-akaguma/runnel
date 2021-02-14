mod test_runnel {
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<StreamIoe>(), 48);
    }
    #[test]
    fn test_debug() {
        let sioe = StreamIoe {
            pin: Box::new(StringIn::with_str("ABCDE\nefgh\n")),
            pout: Box::new(StringOut::default()),
            perr: Box::new(StringErr::default()),
        };
        let s = format!("{:?}", sioe);
        assert_eq!(
            s,
            concat!(
                "StreamIoe {",
                " pin: StringIn(LockableStringIn {",
                " inner: Mutex { data: BufReader { reader: RawStringIn {",
                " buf: \"ABCDE\\nefgh\\n\", pos: 0, amt: 0 }, buffer: 0/1024 } } }),",
                " pout: StringOut(LockableStringOut {",
                " inner: Mutex { data: RawStringOut { buf: \"\" } } }),",
                " perr: StringErr(LockableStringOut {",
                " inner: Mutex { data: RawStringOut { buf: \"\" } } })",
                " }"
            )
        );
    }
    #[test]
    fn test_stdio() {
        use runnel::medium::stdio::{StdErr, StdIn, StdOut};
        use runnel::StreamIoe;
        //use std::io::{BufRead, Write};
        //
        let _sioe = StreamIoe {
            pin: Box::new(StdIn::default()),
            pout: Box::new(StdOut::default()),
            perr: Box::new(StdErr::default()),
        };
        /*
        // pluggable stream in
        let mut lines_iter = sioe.pin.lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        */
        /*
        // pluggable stream out
        #[rustfmt::skip]
        let res = sioe.pout.lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sioe.pout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
        */
        /*
        // pluggable stream err
        #[rustfmt::skip]
        let res = sioe.perr.lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sioe.perr.lock().buffer_str(), "1234\nACBDE\nefgh\n");
        */
    }
    #[test]
    fn test_stringio() {
        use runnel::medium::stringio::{StringErr, StringIn, StringOut};
        use runnel::StreamIoe;
        use std::io::{BufRead, Write};
        //
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
        //
        // pluggable stream out
        #[rustfmt::skip]
        let res = sioe.pout.lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sioe.pout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
        //
        // pluggable stream err
        #[rustfmt::skip]
        let res = sioe.perr.lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sioe.perr.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
    #[test]
    fn test_pipeio() {
        use runnel::medium::pipeio::pipe;
        use runnel::medium::stringio::{StringErr, StringIn, StringOut};
        use runnel::StreamIoe;
        use std::io::{BufRead, Write};
        // create in memory pipe
        let (a_out, a_in) = pipe(1);
        //
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
        //
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
        //
        assert!(handler.join().is_ok());
    }
}