use clap::{Parser, Subcommand, Args};

mod commands;
mod utils;
mod config;

use commands::CmdResult;

#[derive(Parser)]
#[command(name = "batl")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "The multi-repository manager", long_about = None)]
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
    Purge {
        #[arg(short = 'w', long = "workspace")]
        workspace: bool,
        name: String
    },
    Link {
        #[arg(short = 'n', long = "name")]
        name: Option<String>,
        repo: String
    },
    Unlink {
        name: String
    },
    Run {
        #[arg(short = 'r', long = "repo")]
        repo: String,
        #[arg(last = true)]
        cmd: Vec<String>
    },
    Build {
        name: String
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
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> CmdResult<()> {
    let cli = Cli::parse();

    match cli.subcli {
        SubCli::Ls { all } => {
            commands::ls(all)
        },
        SubCli::Init { workspace, name } => {
            commands::init(workspace, name)
        },
        SubCli::Purge { workspace, name } => {
            commands::purge(workspace, name)
        },
        SubCli::Link { name, repo } => {
            commands::link(name, repo)
        },
        SubCli::Unlink { name } => {
            commands::unlink(name)
        },
        SubCli::Run { repo, cmd } => {
            commands::run(repo, cmd)
        },
        SubCli::Build { name } => {
            commands::build(name)
        },
        SubCli::Alias(args) => {
            match args.subcli {
                AliasSubCli::Rename { old, new } => {
                    commands::alias_rename(old, new)
                }
            }
        }
    }
}