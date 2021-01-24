mod test_stdioe {
    use runnel::medium::stdioe::*;
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<StreamInStdin>(), 8);
        assert_eq!(std::mem::size_of::<StreamInLockStdin>(), 16);
        assert_eq!(std::mem::size_of::<StreamOutStdout>(), 8);
        //
        #[cfg(has_fat_stdout)]
        assert_eq!(std::mem::size_of::<StreamOutLockStdout>(), 16);
        #[cfg(not(has_fat_stdout))]
        assert_eq!(std::mem::size_of::<StreamOutLockStdout>(), 8);
        //
        assert_eq!(std::mem::size_of::<StreamErrStderr>(), 8);
        //
        #[cfg(has_fat_stdout)]
        assert_eq!(std::mem::size_of::<StreamErrLockStderr>(), 16);
        #[cfg(not(has_fat_stdout))]
        assert_eq!(std::mem::size_of::<StreamErrLockStderr>(), 8);
    }
}
