#[cfg(test)]
mod test_stream_ioe_pipeio {
    use runnel::medium::pipeio::*;
    use runnel::*;
    use std::io::BufRead;
    use std::io::Write;
    #[test]
    fn test_ioe_pipein() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        let sioe = RunnelIoeBuilder::new()
            .fill_stringio_with_str("")
            .pin(PipeIn::with(receiver))
            .build();
        let handler = std::thread::spawn(move || {
            sender.send("ABCDE\n".as_bytes().to_vec()).unwrap();
            sender.send("efgh\n".as_bytes().to_vec()).unwrap();
        });
        let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
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
            .pout(PipeOut::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
                let mut out = sioe.pout().lock();
                out.write_fmt(format_args!("{}", line)).unwrap();
                out.flush().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), "ABCDE".as_bytes().to_vec());
        assert_eq!(receiver.recv().unwrap(), "efgh".as_bytes().to_vec());
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_pipeerr() {
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .perr(PipeErr::with(sender)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
                let mut err = sioe.perr().lock();
                err.write_fmt(format_args!("{}", line)).unwrap();
                err.flush().unwrap();
            }
        });
        assert_eq!(receiver.recv().unwrap(), "ABCDE".as_bytes().to_vec());
        assert_eq!(receiver.recv().unwrap(), "efgh".as_bytes().to_vec());
        assert!(handler.join().is_ok());
    }
    #[test]
    fn test_ioe_pipeio() {
        let (sout, sin) = pipe(1);
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("ABCDE\nefgh\n")
            .pout(sout).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
                let mut out = sioe.pout().lock();
                out.write_fmt(format_args!("{}\n", line)).unwrap();
                out.flush().unwrap();
            }
        });
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("")
            .pin(sin).build();
        let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
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
            .perr(PipeErr::from(sout)).build();
        let handler = std::thread::spawn(move || {
            for line in sioe.pin().lock().lines().map(|l| l.unwrap()) {
                let mut err = sioe.perr().lock();
                err.write_fmt(format_args!("{}\n", line)).unwrap();
                err.flush().unwrap();
            }
        });
        //
        #[rustfmt::skip]
        let sioe = RunnelIoeBuilder::new().fill_stringio_with_str("")
            .pin(sin).build();
        let mut lines_iter = sioe.pin().lock().lines().map(|l| l.unwrap());
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
    use std::io::{BufRead, Write};

    #[test]
    fn test_pipe_send_empty() {
        let (mut sout, sin) = pipe(1);
        let handle = std::thread::spawn(move || {
            let res = sout.write(b"");
            assert!(res.is_ok());
            let res = sout.flush();
            assert!(res.is_ok());
        });
        let mut lines_iter = sin.lock().lines().map(|l| l.unwrap());
        assert_eq!(lines_iter.next(), None);
        assert!(handle.join().is_ok());
    }
}
