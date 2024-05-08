use clap::{Parser, Subcommand, Args};

mod commands;
mod config;
mod env;
mod output;
mod utils;

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

	let result = match cli.subcmd {
		SubCommand::Workspace(args) => commands::workspace::run(args.subcmd),
		SubCommand::Link(args) => commands::link::run(args.subcmd),
		SubCommand::Repository(args) => commands::repository::run(args.subcmd),
		SubCommand::Setup => commands::cmd_setup()
	};

	if let Err(err) = result {
		output::error(err.to_string().as_str());
	}
}
