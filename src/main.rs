use clap::{Parser, Subcommand};

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
        }
    }
}