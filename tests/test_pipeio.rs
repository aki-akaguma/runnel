#[cfg(test)]
mod test_stream_ioe_pipeio {
    use runnel::medium::pipeio::*;
    use runnel::*;
    use std::io::Write;
    #[test]
    fn test_ioe_pipein() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        let sioe = RunnelIoeBuilder::new()
            .fill_stringio_with_str("")
            .pg_in(PipeIn::with(receiver))
            .build();
        let handler = std::thread::spawn(move || {
            sender.send("ABCDE\n".as_bytes().to_vec()).unwrap();
            sender.send("efgh\n".as_bytes().to_vec()).unwrap();
        });
        let mut lines_iter = sioe.pg_in().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), Some(String::from("ABCDE")));
        assert_eq!(lines_iter.next(), Some(String::from("efgh")));
        assert_eq!(lines_iter.next(), None);
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_pipeout() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_out(PipeOut::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                let mut out = sioe.pg_out().lock();
                out.write_fmt(format_args!("{}", line)).unwrap();
                out.flush().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), "ABCDE".as_bytes().to_vec());
        assert_eq!(receiver.recv().unwrap(), "efgh".as_bytes().to_vec());
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_pipeout_string() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_out(PipeOut::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                sioe.pg_out().write_line(line).unwrap();
                sioe.pg_out().flush_line().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), "ABCDE\n".as_bytes().to_vec());
        assert_eq!(receiver.recv().unwrap(), "efgh\n".as_bytes().to_vec());
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_pipeerr() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_err(PipeErr::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                let mut err = sioe.pg_err().lock();
                err.write_fmt(format_args!("{}", line)).unwrap();
                err.flush().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), "ABCDE".as_bytes().to_vec());
        assert_eq!(receiver.recv().unwrap(), "efgh".as_bytes().to_vec());
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_pipeerr_string() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_err(PipeErr::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                sioe.pg_err().write_line(line).unwrap();
                sioe.pg_err().flush_line().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), "ABCDE\n".as_bytes().to_vec());
        assert_eq!(receiver.recv().unwrap(), "efgh\n".as_bytes().to_vec());
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_pipeio() {
        let (sout, sin) = pipe(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_out(sout).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                let mut out = sioe.pg_out().lock();
                out.write_fmt(format_args!("{}\n", line)).unwrap();
                out.flush().unwrap();
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
    fn test_ioe_pipeio_string() {
        let (sout, sin) = pipe(1);
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
    fn test_ioe_pipeie() {
        let (sout, sin) = pipe(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_err(PipeErr::from(sout)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pg_in().lines().map(|l| l.unwrap()) {
                let mut err = sioe.pg_err().lock();
                err.write_fmt(format_args!("{}\n", line)).unwrap();
                err.flush().unwrap();
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
    fn test_ioe_pipeie_string() {
        let (sout, sin) = pipe(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pg_err(PipeErr::from(sout)).build();
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
mod test_pipeio_more {
    use runnel::medium::pipeio::*;
    use runnel::*;
    use std::io::Write;

    #[test]
    fn test_pipe_send_empty() {
        let (sout, sin) = pipe(1);
        let handle = std::thread::spawn(move || {
            let res = sout.lock().write(b"");
            assert!(res.is_ok());
            let res = sout.lock().flush();
            assert!(res.is_ok());
        });
        let mut lines_iter = sin.lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), None);
        assert!(handle.join().is_ok());
    }
}
