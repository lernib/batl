use clap::{Parser, Subcommand, Args};

mod commands;
mod utils;
mod config;

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
    Repository(SubCmdArgs<commands::repository::Commands>)
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
                println!("Error: {}", err);
            }
        },
        SubCommand::Link(args) => {
            let result = commands::link::run(args.subcmd);

            if let Err(err) = result {
                println!("Error: {}", err);
            }
        },
        SubCommand::Repository(args) => {
            let result = commands::repository::run(args.subcmd);

            if let Err(err) = result {
                println!("Error: {}", err);
            }
        }
    }
}