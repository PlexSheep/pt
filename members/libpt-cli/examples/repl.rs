use console::style;
use libpt_cli::printing;
use libpt_cli::repl::{DefaultRepl, Repl};
use libpt_log::{debug, Logger};

use clap::Subcommand;
use strum::EnumIter;

// this is where you define what data/commands/arguments the REPL accepts
#[derive(Subcommand, Debug, EnumIter, Clone)]
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

fn main() -> anyhow::Result<()> {
    // You would normally make a proper cli interface with clap before entering the repl. This is
    // omitted here for brevity
    let _logger = Logger::builder().display_time(false).build();

    // the compiler can infer that we want to use the ReplCommand enum.
    let mut repl = DefaultRepl::<ReplCommand>::default();

    debug!("entering the repl");
    loop {
        // repl.step() should be at the start of your loop
        // It is here that the repl will get the user input, validate it, and so on
        match repl.step() {
            Ok(c) => c,
            Err(e) => {
                // if the user requested the help, print in blue, otherwise in red as it's just an
                // error
                if let libpt_cli::repl::error::Error::Parsing(e) = &e {
                    if e.kind() == clap::error::ErrorKind::DisplayHelp {
                        println!("{}", style(e).cyan());
                        continue;
                    }
                }
                println!("{}", style(e).red().bold());
                continue;
            }
        };

        // now we can match our defined commands
        //
        // only None if the repl has not stepped yet
        match repl.command().to_owned().unwrap() {
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
                    println!("{}", text.join(" "))
                } else {
                    printing::blockprint(&text.join(" "), console::Color::Cyan)
                }
            }
        }
    }
    Ok(())
}
