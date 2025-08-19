#[cfg(test)]
mod test_stream_ioe_linepipeio {
    use runnel::medium::linepipeio::*;
    use runnel::*;
    #[test]
    fn test_ioe_linein() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        let sioe = RunnelIoeBuilder::new()
            .fill_stringio_with_str("")
            .pg_in(LinePipeIn::with(receiver))
            .build();
        let handler = std::thread::spawn(move || {
            sender.send(vec!["ABCDE".to_string()]).unwrap();
            sender.send(vec!["efgh".to_string()]).unwrap();
        });
        let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_lineout() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_out(LinePipeOut::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                sioe.pg_out().write_line(line).unwrap();
                sioe.pg_out().flush_line().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), vec!["ABCDE".to_string()]);
        assert_eq!(receiver.recv().unwrap(), vec!["efgh".to_string()]);
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_lineerr() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_err(LinePipeErr::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                sioe.pg_err().write_line(line).unwrap();
                sioe.pg_err().flush_line().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), vec!["ABCDE".to_string()]);
        assert_eq!(receiver.recv().unwrap(), vec!["efgh".to_string()]);
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_linepipeio() {
        let (sout, sin) = line_pipe(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_out(sout).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                sioe.pg_out().write_line(line).unwrap();
                sioe.pg_out().flush_line().unwrap();
            }
        });
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("")
            .pg_in(sin).build();
        let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_lineie() {
        let (sout, sin) = line_pipe(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_err(LinePipeErr::from(sout)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                sioe.pg_err().write_line(line).unwrap();
                sioe.pg_err().flush_line().unwrap();
            }
        });
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("")
            .pg_in(sin).build();
        let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        assert!(handler.join().is_ok());
    }
}

#[cfg(test)]
mod test_linepipeio_more {
    use runnel::medium::linepipeio::*;
    use runnel::*;

    #[test]
    fn test_line_send_empty() {
        let (sout, sin) = line_pipe(1);
        let handle = std::thread::spawn(move || {
            let res = sout.write_line("".to_string());
            assert!(res.is_ok());
            let res = sout.flush_line();
            assert!(res.is_ok());
        });
        let mut lines_iter = sin.lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some("".to_string()));
        assert_eq!(lines_iter.next(), None);
        assert!(handle.join().is_ok());
    }
}
