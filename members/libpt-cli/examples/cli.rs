use clap::Parser;
use libpt_cli::args::VerbosityLevel;
use libpt_cli::{clap, printing};
use libpt_log::{debug, Logger};

/// This is the help
///
/// This is more help
#[derive(Parser, Debug)]
struct Cli {
    // already has documentation
    #[command(flatten)]
    verbosity: VerbosityLevel,

    /// texts to be echoed
    #[arg(required = true)]
    text: Vec<String>,

    /// try to be more machine readable
    #[arg(short, long)]
    machine: bool,
}

fn main() {
    let cli = Cli::parse();
    let _logger = {
        let mut this = {
            let mut this = Logger::builder();
            let max_level = cli.verbosity.level();
            this.max_level = max_level;
            this
        };
        this.show_time = false;
        this
    }
    .build();

    debug!("logger initialized with level: {}", cli.verbosity.level());

    if !cli.machine {
        let text = cli.text.join(" ");
        printing::blockprint(text, console::Color::Green);
    } else {
        for text in cli.text {
            println!("{text}")
        }
    }
}
