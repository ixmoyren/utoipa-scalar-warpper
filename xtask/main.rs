use crate::task::upgrade_scalar_api_reference;
use clap::{Parser, Subcommand};

mod task;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    #[command(about = "Convert the json file to a binary file in rkyv format.")]
    UpgradeScalarApiReference,
}

fn main() {
    let cli = Cli::parse();
    match cli.action {
        Action::UpgradeScalarApiReference => upgrade_scalar_api_reference(),
    }
}
