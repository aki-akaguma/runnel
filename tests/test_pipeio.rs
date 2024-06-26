#[cfg(target_arch = "x86_64")]
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod test_pipeio {
    use runnel::medium::pipeio::*;
    //
    #[test]
    fn test_size_of_1() {
        assert_eq!(std::mem::size_of::<PipeInLock>(), 16);
        assert_eq!(std::mem::size_of::<PipeOutLock>(), 16);
    }
    //
    #[rustversion::before(1.59)]
    #[test]
    fn test_size_of_2() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 72);
    }
    #[rustversion::all(since(1.59), before(1.62))]
    #[test]
    fn test_size_of_2() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 112);
        assert_eq!(std::mem::size_of::<PipeOut>(), 72);
    }
    #[rustversion::all(since(1.62), before(1.64))]
    #[test]
    fn test_size_of_2() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 64);
    }
    #[rustversion::all(since(1.64), before(1.65))]
    #[test]
    fn test_size_of_2() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 96);
        assert_eq!(std::mem::size_of::<PipeOut>(), 64);
    }
    #[rustversion::all(since(1.65), before(1.67))]
    #[test]
    fn test_size_of_2() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 64);
    }
    #[rustversion::since(1.67)]
    #[test]
    fn test_size_of_2() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 72);
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
