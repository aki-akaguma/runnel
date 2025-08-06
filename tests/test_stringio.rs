#[cfg(test)]
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

#[cfg(test)]
mod test_stream_ioe_stringio {
    use runnel::*;
    use std::io::BufRead;
    use std::io::Write;

    #[test]
    fn test_ioe() {
        let sioe = RunnelIoeBuilder::new()
            .fill_stringio_with_str("ABCDE\nefgh\n")
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

#[cfg(test)]
mod test_stringio_more {
    use runnel::medium::stringio::*;
    use runnel::*;
    use std::io::{Read, Write};

    #[test]
    fn test_string_in_empty() {
        let mut sin = StringIn::with_str("");
        let mut buf = [0; 10];
        let res = sin.read(&mut buf);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn test_string_in_read_twice() {
        let mut sin = StringIn::with_str("ABCDE");
        let mut buf = [0; 3];
        let res = sin.read(&mut buf);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);
        assert_eq!(&buf[..], b"ABC");
        let res = sin.read(&mut buf);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 2);
        assert_eq!(&buf[..2], b"DE");
    }

    #[test]
    fn test_string_out_empty_write() {
        let sout = StringOut::default();
        let res = sout.lock().write(b"");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);
        assert_eq!(sout.lock().buffer_str(), "");
    }
}
