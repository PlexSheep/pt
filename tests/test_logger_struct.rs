//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////

//// HELPERS ///////////////////////////////////////////////////////////////////////////////////////
/// ## checks if the expected thing was printed to stdout
///
/// Source: [users.rust-lang.org](https://users.rust-lang.org/t/how-to-test-functions-that-use-
/// println/67188/5)
macro_rules! get_stdout_for {
    ($test:expr) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stdout().unwrap();

        $test;

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        output
    }};
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
/// ## Tests for basic logging functionality

mod test_logger_struct {
    use libpt::logger::*;

    use regex::Regex;

    fn setup() {
        // we don't want to log messages during our tests!
        Logger::init_specialized(false, false, env_logger::Target::Stdout);
        println!()
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
    fn test_log_basic() {
        std::env::set_var(LOGGER_ENV_KEY, "Trace");
        setup();
        let l = Logger::new();
        let trace_out = get_stdout_for!(l.trace("MSG"));
        let debug_out = get_stdout_for!(l.debug("MSG"));
        let info_out = get_stdout_for!(l.info("MSG"));
        let warn_out = get_stdout_for!(l.warn("MSG"));
        let error_out = get_stdout_for!(l.error("MSG"));
        let combined = format!(
            "{}{}{}{}{}",
            trace_out, debug_out, info_out, warn_out, error_out
        );
        print!("{}", combined);

        // too long, so i split into two lines.
        let regex = Regex::new(concat!(
            r"(?m)\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ",
            r"(TRACE|DEBUG|INFO|WARN|ERROR) +libpt::logger\] MSG"
        ))
        .unwrap();

        assert_eq!(regex.captures_iter(&combined).count(), 5);
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
