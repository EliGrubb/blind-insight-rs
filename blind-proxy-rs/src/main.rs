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
    Login,
    Organization(OrganizationArgs),
    Dataset(DatasetArgs),
    Schema(SchemaArgs),
    Record(RecordArgs),
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

#[derive(Debug, Args)]
struct OrganizationArgs {
    #[command(subcommand)]
    command: OrganizationCommands,
}

#[derive(Debug, Subcommand)]
enum OrganizationCommands {
    Inspect,
}

#[derive(Debug, Args)]
struct DatasetArgs {
    #[command(subcommand)]
    command: DatasetCommands,
}

#[derive(Debug, Subcommand)]
enum DatasetCommands {
    Inspect,
}

#[derive(Debug, Args)]
struct SchemaArgs {
    #[command(subcommand)]
    command: SchemaCommands,
}

#[derive(Debug, Subcommand)]
enum SchemaCommands {
    Inspect,
}

#[derive(Debug, Args)]
struct RecordArgs {
    #[command(subcommand)]
    command: RecordCommands,
}

#[derive(Debug, Subcommand)]
enum RecordCommands {
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
        BlindProxyCommands::Login => {
            println!("You called login! Let's get you in!");
        }
        BlindProxyCommands::Organization(org_args) => match org_args.command {
            OrganizationCommands::Inspect => {
                println!("You called organization inspect! Let's see the details!");
            }
        },
        BlindProxyCommands::Dataset(dataset_args) => match dataset_args.command {
            DatasetCommands::Inspect => {
                println!("You called dataset inspect! Let's check it out!");
            }
        },
        BlindProxyCommands::Schema(schema_args) => match schema_args.command {
            SchemaCommands::Inspect => {
                println!("You called schema inspect! Let's review the schema!");
            }
        },
        BlindProxyCommands::Record(record_args) => match record_args.command {
            RecordCommands::Create => {
                println!("You called record create! Let's create a new record!");
            }
            RecordCommands::Inspect => {
                println!("You called record inspect! Let's look at the record details!");
            }
        },
    }
}
