use std::process::exit;

use clap::{ArgAction, Parser, Subcommand};
use xshell::{Shell, cmd};

use crate::utills::initialize_shell;

mod download_tools;
mod utills;

#[derive(Parser)]
struct CliArgs {
    /// Run in verbose mode.
    #[arg(global = true, long, action = ArgAction::SetTrue)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Subcommand)]
enum Command {
    PreCommit,
    FmtCheck,
    Fmt,
    Clippy,
    DxCheck,
    DenyCheck,
    /// Ensure all all dev cli tools are installed
    EnsureToolsDownloaded,
    Machete,
}
impl Command {
    pub fn execute(self, sh: &Shell) -> Result<(), xshell::Error> {
        match self {
            Command::DenyCheck => {
                cmd!(sh, "./bin/cargo-deny --workspace check").run()?;
            }
            Command::DxCheck => {
                cmd!(sh, "./bin/dx check --package remindy-web").run()?;
            }
            Command::Machete => {
                cmd!(sh, "./bin/cargo-machete").run()?;
            }
            Command::EnsureToolsDownloaded => download_tools::download_all_tools(sh)?,
            Command::Fmt => {
                cmd!(sh, "cargo fmt").run()?;
                cmd!(sh, "./bin/dx fmt").run()?;
            }
            Command::FmtCheck => {
                cmd!(sh, "cargo fmt --check").run()?;
                cmd!(sh, "./bin/dx fmt --check").run()?;
            }
            Command::Clippy => {
                cmd!(sh, "cargo clippy --workspace --all-targets -- -D warnings").run()?;
            }
            Command::PreCommit => {
                Self::EnsureToolsDownloaded.execute(sh)?;
                Self::DxCheck.execute(sh)?;
                Self::DenyCheck.execute(sh)?;
                Self::FmtCheck.execute(sh)?;
                Self::Machete.execute(sh)?;
                Self::Clippy.execute(sh)?;
            }
        }
        Ok(())
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = CliArgs::parse();

    match initialize_shell() {
        Ok(sh) => {
            if let Err(err) = args.command.execute(&sh) {
                eprintln!("failed to run xshell {err}");
                exit(1);
            }
        }
        Err(err) => {
            eprintln!("failed to run xshell {err}");
            exit(1);
        }
    }
}
