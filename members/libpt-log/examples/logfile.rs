use libpt_log::Logger;
use tracing::info;

fn main() {
    let _logger = Logger::builder()
        .log_to_file(true)
        .log_dir("/tmp/llll".into())
        .build()
        .unwrap();
    info!("foo bar qux");
}
