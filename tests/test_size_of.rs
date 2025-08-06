#[cfg(test)]
mod test_size_of {
    use runnel::{RunnelIoe, RunnelIoeBuilder};
    //
    #[test]
    fn test_size_of_runnel_ioe() {
        #[cfg(target_arch = "x86_64")]
        {
            assert_eq!(std::mem::size_of::<RunnelIoe>(), 48);
            assert_eq!(std::mem::size_of::<RunnelIoeBuilder>(), 48);
        }
        #[cfg(target_arch = "x86")]
        {
            assert_eq!(std::mem::size_of::<RunnelIoe>(), 24);
            assert_eq!(std::mem::size_of::<RunnelIoeBuilder>(), 24);
        }
    }
}

#[cfg(test)]
#[cfg(target_arch = "x86_64")]
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod test_size_of_lineio {
    use runnel::medium::lineio::*;
    //
    #[test]
    fn test_size_of_line_in_out_lock() {
        assert_eq!(std::mem::size_of::<LineInLock>(), 16);
        assert_eq!(std::mem::size_of::<LineOutLock>(), 16);
    }
    //
    #[rustversion::since(1.67)]
    #[test]
    fn test_size_of_line_in_out() {
        assert_eq!(std::mem::size_of::<LineIn>(), 72);
        assert_eq!(std::mem::size_of::<LineOut>(), 40);
    }
}

#[cfg(test)]
#[cfg(target_arch = "x86_64")]
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod test_size_of_pipeio {
    use runnel::medium::pipeio::*;
    //
    #[test]
    fn test_size_of_pipe_in_out_lock() {
        assert_eq!(std::mem::size_of::<PipeInLock>(), 16);
        assert_eq!(std::mem::size_of::<PipeOutLock>(), 16);
    }
    //
    #[rustversion::before(1.59)]
    #[test]
    fn test_size_of_pipe_in_out() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 72);
    }
    #[rustversion::all(since(1.59), before(1.62))]
    #[test]
    fn test_size_of_pipe_in_out() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 112);
        assert_eq!(std::mem::size_of::<PipeOut>(), 72);
    }
    #[rustversion::all(since(1.62), before(1.64))]
    #[test]
    fn test_size_of_pipe_in_out() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 64);
    }
    #[rustversion::all(since(1.64), before(1.65))]
    #[test]
    fn test_size_of_pipe_in_out() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 96);
        assert_eq!(std::mem::size_of::<PipeOut>(), 64);
    }
    #[rustversion::all(since(1.65), before(1.67))]
    #[test]
    fn test_size_of_pipe_in_out() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 64);
    }
    #[rustversion::since(1.67)]
    #[test]
    fn test_size_of_pipe_in_out() {
        assert_eq!(std::mem::size_of::<PipeIn>(), 104);
        assert_eq!(std::mem::size_of::<PipeOut>(), 72);
    }
}

#[cfg(test)]
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod test_size_of_stdio {
    use runnel::medium::stdio::*;
    #[test]
    fn test_size_of_std_in_out_err() {
        #[cfg(target_arch = "x86_64")]
        {
            assert_eq!(std::mem::size_of::<StdIn>(), 8);
            assert_eq!(std::mem::size_of::<StdOut>(), 8);
            assert_eq!(std::mem::size_of::<StdErr>(), 8);
            //
            assert_eq!(std::mem::size_of::<StdInLock>(), 16);
            assert_eq!(std::mem::size_of::<StdOutLock>(), 8);
            assert_eq!(std::mem::size_of::<StdErrLock>(), 8);
        }
        #[cfg(target_arch = "x86")]
        {
            assert_eq!(std::mem::size_of::<StdIn>(), 4);
            assert_eq!(std::mem::size_of::<StdOut>(), 4);
            assert_eq!(std::mem::size_of::<StdErr>(), 4);
            //
            assert_eq!(std::mem::size_of::<StdInLock>(), 8);
            assert_eq!(std::mem::size_of::<StdOutLock>(), 4);
            assert_eq!(std::mem::size_of::<StdErrLock>(), 4);
        }
    }
}

#[cfg(test)]
#[cfg(target_arch = "x86_64")]
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod test_size_of_stringio {
    use runnel::medium::stringio::*;
    //
    #[test]
    fn test_size_of_string_in_out_err_lock() {
        assert_eq!(std::mem::size_of::<StringInLock>(), 16);
        assert_eq!(std::mem::size_of::<StringOutLock>(), 16);
        assert_eq!(std::mem::size_of::<StringErrLock>(), 16);
    }
    //
    #[rustversion::before(1.59)]
    #[test]
    fn test_size_of_string_in_out_err() {
        assert_eq!(std::mem::size_of::<StringIn>(), 88);
        assert_eq!(std::mem::size_of::<StringOut>(), 40);
        assert_eq!(std::mem::size_of::<StringErr>(), 40);
    }
    #[rustversion::all(since(1.59), before(1.62))]
    #[test]
    fn test_size_of_string_in_out_err() {
        assert_eq!(std::mem::size_of::<StringIn>(), 96);
        assert_eq!(std::mem::size_of::<StringOut>(), 40);
        assert_eq!(std::mem::size_of::<StringErr>(), 40);
    }
    #[rustversion::all(since(1.62), before(1.64))]
    #[test]
    fn test_size_of_string_in_out_err() {
        assert_eq!(std::mem::size_of::<StringIn>(), 88);
        assert_eq!(std::mem::size_of::<StringOut>(), 32);
        assert_eq!(std::mem::size_of::<StringErr>(), 32);
    }
    #[rustversion::all(since(1.64), before(1.65))]
    #[test]
    fn test_size_of_string_in_out_err() {
        assert_eq!(std::mem::size_of::<StringIn>(), 80);
        assert_eq!(std::mem::size_of::<StringOut>(), 32);
        assert_eq!(std::mem::size_of::<StringErr>(), 32);
    }
    #[rustversion::since(1.65)]
    #[test]
    fn test_size_of_string_in_out_err() {
        assert_eq!(std::mem::size_of::<StringIn>(), 88);
        assert_eq!(std::mem::size_of::<StringOut>(), 32);
        assert_eq!(std::mem::size_of::<StringErr>(), 32);
    }
}
