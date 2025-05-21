use blind_wrapper_rs::{apis::{organizations_api::organizations_create, *}, models::*};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "blind-proxy-rs")]
#[command(bin_name = "blind-proxy-rs")]
struct BlindProxyCli {
    #[command(subcommand)]
    command: BlindProxyCommands,
}

#[derive(Debug, Subcommand)]
enum BlindProxyCommands {
    /// Commands for managing the blind proxy
    Keyring(KeyringArgs),
}

#[derive(Debug, Args)]
struct KeyringArgs {
    #[command(subcommand)]
    command: KeyringCommands,
}

#[derive(Debug, Subcommand)]
enum KeyringCommands {
    Create,
    Inspect,
}

fn main() {
    let args = BlindProxyCli::parse();

    match args.command {
        BlindProxyCommands::Keyring(keyring_args) => match keyring_args.command {
            KeyringCommands::Create => {
                println!("You called keyring create! Wowza!");
            }
            KeyringCommands::Inspect => {
                println!("You called keyring inspect! Holy cow!");
            }
        },
    }
}
