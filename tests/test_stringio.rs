mod test_stringio {
    use runnel::medium::stringio::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<StringIn>(), 88);
        assert_eq!(std::mem::size_of::<StringInLock>(), 16);
        assert_eq!(std::mem::size_of::<StringOut>(), 40);
        assert_eq!(std::mem::size_of::<StringOutLock>(), 16);
        assert_eq!(std::mem::size_of::<StringErr>(), 40);
        assert_eq!(std::mem::size_of::<StringErrLock>(), 16);
    }
}
mod test_stream_stringio {
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::BufRead;
    use std::io::Write;
    #[test]
    fn test_in() {
        let sin = StringIn::with_str("ABCDE\nefgh\n");
        let mut lines_iter = sin.lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
    }
    #[test]
    fn test_out() {
        let sout = StringOut::default();
        #[rustfmt::skip]
        let res = sout.lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(sout.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
    #[test]
    fn test_err() {
        let serr = StringErr::default();
        #[rustfmt::skip]
        let res = serr.lock()
            .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
        assert!(res.is_ok());
        assert_eq!(serr.lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
}

mod test_stream_ioe_stringio {
    use runnel::*;
    use std::io::BufRead;
    use std::io::Write;

    #[test]
    fn test_ioe() {
        let sioe = RunnelIoeBuilder::new()
            .fill_stringio_wit_str("ABCDE\nefgh\n")
            .build();
        {
            let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
            assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
            assert_eq!(lines_iter.next(), Some(String::from("efgh")));
            assert_eq!(lines_iter.next(), None);
            //
            #[rustfmt::skip]
            let res = sioe.pout().lock()
                .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
            assert!(res.is_ok());
            //
            #[rustfmt::skip]
            let res = sioe.perr().lock()
                .write_fmt(format_args!("{}\nACBDE\nefgh\n", 1234));
            assert!(res.is_ok());
        }
        assert_eq!(sioe.pout().lock().buffer_str(), "1234\nACBDE\nefgh\n");
        assert_eq!(sioe.perr().lock().buffer_str(), "1234\nACBDE\nefgh\n");
    }
}
