use std::io;
use std::process::ExitCode;

use clap::Parser;

use task_manager::{
    cli::{commands::Cli, execute},
    error::Result,
    service::TaskService,
    storage::JsonRepository,
};

fn run() -> Result<()> {
    let cli = Cli::parse();
    let repo = JsonRepository::open(cli.file)?;
    let mut service = TaskService::new(repo);
    execute(&mut service, cli.command, &mut io::stdout())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Hata: {e}");
            ExitCode::FAILURE
        }
    }
}
