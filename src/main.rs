use clap::{Parser, Subcommand, Args};

mod commands;
mod utils;
mod config;
mod output;
mod runtime;

#[derive(Parser)]
#[command(name = "batl")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "The multi-repo development tool")]
struct Cli {
    #[command(subcommand)]
    subcmd: SubCommand
}

#[derive(Subcommand)]
enum SubCommand {
    Workspace(SubCmdArgs<commands::workspace::Commands>),
    Link(SubCmdArgs<commands::link::Commands>),
    Repository(SubCmdArgs<commands::repository::Commands>),
    Setup
}

#[derive(Args)]
struct SubCmdArgs<T: Subcommand> {
    #[command(subcommand)]
    subcmd: T
}


fn main() {
    let cli = Cli::parse();

    match cli.subcmd {
        SubCommand::Workspace(args) => {
            let result = commands::workspace::run(args.subcmd);

            if let Err(err) = result {
                output::error(err.to_string().as_str());
            }
        },
        SubCommand::Link(args) => {
            let result = commands::link::run(args.subcmd);

            if let Err(err) = result {
                output::error(err.to_string().as_str());
            }
        },
        SubCommand::Repository(args) => {
            let result = commands::repository::run(args.subcmd);

            if let Err(err) = result {
                output::error(err.to_string().as_str());
            }
        },
        SubCommand::Setup => {
            let result = commands::cmd_setup();

            if let Err(err) = result {
                output::error(err.to_string().as_str());
            }
        }
    }
}