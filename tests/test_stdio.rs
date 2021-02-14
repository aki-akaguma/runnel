mod test_stdio {
    use runnel::medium::stdio::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<StdIn>(), 8);
        assert_eq!(std::mem::size_of::<StdInLock>(), 16);
        assert_eq!(std::mem::size_of::<StdOut>(), 8);
        //
        #[cfg(has_fat_stdout)]
        assert_eq!(std::mem::size_of::<StdOutLock>(), 16);
        #[cfg(not(has_fat_stdout))]
        assert_eq!(std::mem::size_of::<StdOutLock>(), 8);
        //
        assert_eq!(std::mem::size_of::<StdErr>(), 8);
        //
        #[cfg(has_fat_stdout)]
        assert_eq!(std::mem::size_of::<StdErrLock>(), 16);
        #[cfg(not(has_fat_stdout))]
        assert_eq!(std::mem::size_of::<StdErrLock>(), 8);
    }
}
