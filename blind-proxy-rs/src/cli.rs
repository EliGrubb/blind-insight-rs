/// Data structures for handling CLI commands
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "blind-proxy-rs")]
#[command(bin_name = "blind-proxy-rs")]
pub struct BlindProxyCli {
    #[command(subcommand)]
    pub command: BlindProxyCommands,
}

#[derive(Debug, Subcommand)]
pub enum BlindProxyCommands {
    /// Commands for managing the blind proxy
    Keyring(KeyringArgs),
    Login,
    Organization(OrganizationArgs),
    Dataset(DatasetArgs),
    Schema(SchemaArgs),
    Record(RecordArgs),
}

#[derive(Debug, Args)]
pub struct KeyringArgs {
    #[command(subcommand)]
    pub command: KeyringCommands,
}

#[derive(Debug, Subcommand)]
pub enum KeyringCommands {
    Create,
    Inspect,
}

#[derive(Debug, Args)]
pub struct OrganizationArgs {
    #[command(subcommand)]
    pub command: OrganizationCommands,
}

#[derive(Debug, Subcommand)]
pub enum OrganizationCommands {
    Inspect,
}

#[derive(Debug, Args)]
pub struct DatasetArgs {
    #[command(subcommand)]
    pub command: DatasetCommands,
}

#[derive(Debug, Subcommand)]
pub enum DatasetCommands {
    Inspect,
}

#[derive(Debug, Args)]
pub struct SchemaArgs {
    #[command(subcommand)]
    pub command: SchemaCommands,
}

#[derive(Debug, Subcommand)]
pub enum SchemaCommands {
    Inspect,
}

#[derive(Debug, Args)]
pub struct RecordArgs {
    #[command(subcommand)]
    pub command: RecordCommands,
}

#[derive(Debug, Subcommand)]
pub enum RecordCommands {
    Create,
    Inspect,
}