#[cfg(test)]
mod test_lineio {
    use runnel::medium::lineio::*;
    use std::io::{Read, Write};

    #[test]
    fn test_line_in() {
        let mut line_in = LineIn::default();
        let mut buf = [0; 10];
        let res = line_in.read(&mut buf);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 0);
    }

    #[test]
    fn test_line_out() {
        let mut line_out = LineOut::default();
        let res = line_out.write(b"test");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_line_err() {
        let mut line_err = LineErr::default();
        let res = line_err.write(b"test");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 4);
    }
}
