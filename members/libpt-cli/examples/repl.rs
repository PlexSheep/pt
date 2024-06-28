use libpt_cli::repl::REPL_HELP_TEMPLATE;
use libpt_cli::{clap, dialoguer, printing};
use libpt_log::{debug, trace, Level, Logger};

use clap::{Parser, Subcommand};

/// This is the help menu of the repl
///
/// More text here
#[derive(Parser, Debug)]
#[command(multicall = true)]
pub struct Repl {
    /// the command you want to execute, along with its args
    #[command(subcommand)]
    command: ReplCommand,
}

#[derive(Subcommand, Debug)]
enum ReplCommand {
    /// wait for LEN seconds
    Wait {
        /// wait so long
        len: u64,
    },
    /// echo the given texts
    Echo {
        /// the text you want to print
        text: Vec<String>,
        /// print with a fancy border and colors
        #[arg(short, long)]
        fancy: bool,
    },
    /// hello world
    Hello,
    /// leave the repl
    Exit,
}

// TODO: somehow autogenerate this!!!
pub struct MyCompletion {
    options: Vec<String>,
}
impl Default for MyCompletion {
    fn default() -> Self {
        MyCompletion {
            options: vec![
                "help".to_string(),
                "?".to_string(),
                "list".to_string(),
                "publish".to_string(),
                "unpublish".to_string(),
                "delete".to_string(),
                "read".to_string(),
                "show".to_string(),
                "new".to_string(),
                "ls".to_string(),
            ],
        }
    }
}

impl dialoguer::Completion for MyCompletion {
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}

fn main() -> anyhow::Result<()> {
    let _logger = Logger::builder()
        .show_time(false)
        .max_level(Level::DEBUG)
        .build();

    let mut buf: String = String::new();
    let mut buf_preparsed: Vec<String>;
    let completion = MyCompletion::default();
    let mut history = dialoguer::BasicHistory::new();

    debug!("entering the repl");
    loop {
        buf.clear();

        buf = dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .completion_with(&completion)
            .history_with(&mut history)
            .interact_text()?;

        buf_preparsed = Vec::new();
        buf_preparsed.extend(shlex::split(&buf).unwrap_or_default());

        trace!("read input: {buf_preparsed:?}");

        let options = match Repl::try_parse_from(buf_preparsed) {
            Ok(c) => c,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match options.command {
            ReplCommand::Exit => break,
            ReplCommand::Wait { len } => {
                debug!("len: {len}");
                let spinner = indicatif::ProgressBar::new_spinner();
                spinner.enable_steady_tick(std::time::Duration::from_millis(100));
                std::thread::sleep(std::time::Duration::from_secs(len));
                spinner.finish();
            }
            ReplCommand::Hello => println!("Hello!"),
            ReplCommand::Echo { text, fancy } => {
                if !fancy {
                    println!("{}", text.concat())
                }
                else {
                    printing::blockprint(text.concat(), console::Color::Cyan)
                }
            }
        }
    }
    Ok(())
}
