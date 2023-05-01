use clap::{Parser, Subcommand, Args};

mod commands;
mod utils;
mod config;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    subcli: SubCli
}

#[derive(Subcommand)]
enum SubCli {
    Ls {
        #[arg(long = "all")]
        all: bool
    },
    Init {
        #[arg(short = 'w', long = "workspace")]
        workspace: bool,
        name: String
    },
    Link {
        name: String
    },
    Run {
        #[arg(short = 'r', long = "repo")]
        repo: String,
        #[arg(last = true)]
        cmd: Vec<String>
    },
    Alias(AliasArgs)
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct AliasArgs {
    #[command(subcommand)]
    subcli: AliasSubCli
}

#[derive(Subcommand)]
enum AliasSubCli {
    Rename {
        old: String,
        new: String
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.subcli {
        SubCli::Ls { all } => {
            commands::ls(all);
        },
        SubCli::Init { workspace, name } => {
            commands::init(workspace, name);
        },
        SubCli::Link { name } => {
            commands::link(name);
        },
        SubCli::Run { repo, cmd } => {
            commands::run(repo, cmd);
        },
        SubCli::Alias(args) => {
            match args.subcli {
                AliasSubCli::Rename { old, new } => {
                    commands::alias_rename(old, new);
                }
            }
        }
    }
}