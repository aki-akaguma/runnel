mod test_stringio {
    use runnel::medium::stringio::*;
    use std::io::BufRead;
    use std::io::Write;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<StreamInStringIn>(), 88);
        assert_eq!(std::mem::size_of::<StreamInLockStringIn>(), 16);
        assert_eq!(std::mem::size_of::<StreamOutStringOut>(), 40);
        assert_eq!(std::mem::size_of::<StreamOutLockStringOut>(), 16);
        assert_eq!(std::mem::size_of::<StreamErrStringErr>(), 40);
        assert_eq!(std::mem::size_of::<StreamErrLockStringErr>(), 16);
    }
    #[test]
    fn test_in() {
        let sin = StringIn::with("ABCDE\nefgh\n".to_string());
        let mut lines_iter = sin.lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
    }
    #[test]
    fn test_out() {
        let sout = StringOut::default();
        let res = sout
            .lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
}

mod test_stream_stringio {
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::BufRead;
    use std::io::Write;
    #[test]
    fn test_in() {
        let sin = StreamInStringIn::with_str("ABCDE\nefgh\n");
        let mut lines_iter = sin.lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
    }
    #[test]
    fn test_out() {
        let sout = StreamOutStringOut::default();
        let res = sout
            .lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
    #[test]
    fn test_err() {
        let serr = StreamErrStringErr::default();
        let res = serr
            .lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(serr.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
}

mod test_stream_ioe_stringio {
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::BufRead;
    use std::io::Write;

    #[test]
    fn test_ioe() {
        let sioe = StreamIoe {
            sin: Box::new(StreamInStringIn::with_str("ABCDE\nefgh\n")),
            sout: Box::new(StreamOutStringOut::default()),
            serr: Box::new(StreamErrStringErr::default()),
        };
        {
            let mut lines_iter = sioe.sin.lock().lines().map(|l| l.unwrap());
            assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
            assert_eq!(lines_iter.next(), Some(String::from("efgh")));
            assert_eq!(lines_iter.next(), None);
            //
            let res = sioe
                .sout
                .lock()
                .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
            assert!(res.is_ok());
            //
            let res = sioe
                .serr
                .lock()
                .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
            assert!(res.is_ok());
        }
        assert_eq!(sioe.sout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
        assert_eq!(sioe.serr.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
}
