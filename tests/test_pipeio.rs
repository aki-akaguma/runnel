mod test_pipeio {
    use runnel::medium::pipeio::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeInLock>(), 16);
        assert_eq!(std::mem::size_of::<PipeOut>(), 48);
        assert_eq!(std::mem::size_of::<PipeOutLock>(), 16);
    }
}
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
            sender.send("ABCDE\n".to_string()).unwrap();
            sender.send("efgh\n".to_string()).unwrap();
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
        assert_eq!(receiver.recv().unwrap(), "ABCDE");
        assert_eq!(receiver.recv().unwrap(), "efgh");
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
        assert_eq!(receiver.recv().unwrap(), "ABCDE");
        assert_eq!(receiver.recv().unwrap(), "efgh");
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
