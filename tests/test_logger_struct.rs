//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////

//// HELPERS ///////////////////////////////////////////////////////////////////////////////////////
/// ## checks if the expected thing was printed to stdout
///
/// Source: [users.rust-lang.org](https://users.rust-lang.org/t/how-to-test-functions-that-use-
/// println/67188/5)
macro_rules! assert_stdout_eq {
    ($test:expr, $expected:literal) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stdout().unwrap();

        $test;

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        assert_eq!(&output, $expected);
    }};
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
/// ## Tests for basic logging functionality

mod test_logger_struct {
    use libpt::logger::*;

    fn setup() {
        // we don't want to log messages during our tests!
        Logger::init_target(false, env_logger::Target::Stdout)
    }

    /// ## Tests for basic logging
    ///
    /// This test tests if the loggers basic logging functionality works, that is it's methods:
    ///
    /// - [`Logger::trace`]
    /// - [`Logger::debug`]
    /// - [`Logger::info`]
    /// - [`Logger::warn`]
    /// - [`Logger::error`]
    #[test]
    #[ignore]
    fn test_log_basic() {
        std::env::set_var(LOGGER_ENV_KEY, "Trace");
        setup();
        let l = Logger::new();
        l.error("HELP");
        assert_stdout_eq!(l.trace("hello world"),   "\u{1b}[0m\u{1b}[38;5;8m[\u{1b}[0m2023-07-07T1\
            8:59:03Z \u{1b}[0m\u{1b}[36mTRACE\u{1b}[0m libpt::logger\u{1b}[0m\u{1b}[38;5;8m]\u{1b}[\
            0m hello world\n");
        assert_stdout_eq!(l.debug("hello world"),   "\u{1b}[0m\u{1b}[38;5;8m[\u{1b} [0m2023-07-07T1\
            8:59:03Z \u{1b}[0m\u{1b}[34mDEBUG\u{1b}[0m libpt::logger\u{1b}[0m\u{1b}[38;5;8m]\u{1b}[\
            0m hello world\n");
        assert_stdout_eq!(l.info("hello world"),   "\u{1b}[0m\u{1b}[38;5;8m[\u{1b} [0m2023-07-07T1\
            8:59:03Z \u{1b}[0m\u{1b}[34mINFO\u{1b}[0m libpt::logger\u{1b}[0m\u{1b}[38;5;8m]\u{1b}[\
            0m hello world\n");
        assert_stdout_eq!(l.warn("hello world"),   "\u{1b}[0m\u{1b}[38;5;8m[\u{1b} [0m2023-07-07T1\
            8:59:03Z \u{1b}[0m\u{1b}[34mWARN\u{1b}[0m libpt::logger\u{1b}[0m\u{1b}[38;5;8m]\u{1b}[\
            0m hello world\n");
        assert_stdout_eq!(l.error("hello world"),   "\u{1b}[0m\u{1b}[38;5;8m[\u{1b} [0m2023-07-07T1\
            8:59:03Z \u{1b}[0m\u{1b}[34mERROR\u{1b}[0m libpt::logger\u{1b}[0m\u{1b}[38;5;8m]\u{1b}[\
            0m hello world\n");
    }

    #[test]
    fn test_multi_initialize() {
        setup();
        let l = Logger::new();
        // these should be ignored due to the global flag
        Logger::init();
        Logger::init();
        Logger::init();
        Logger::init();
        l.info("Successfully ignored extra init");
    }
}
