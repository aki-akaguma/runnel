#[cfg(not(any(target_os = "windows", target_os = "macos")))]
mod test_stdio {
    use runnel::medium::stdio::*;
    #[test]
    fn test_size() {
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
