use clap::Parser;
use cli::*;
use inquire::{Confirm, Password, PasswordDisplayMode, Text};
use tokio::runtime::Runtime;

mod cli;
mod bip39;
mod keyring;
mod error;
mod organizations;

/// Primary entry point and logic for the blind proxy CLI application.
fn main() {
    let args = BlindProxyCli::parse();
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    match args.command {
        BlindProxyCommands::Keyring(keyring_args) => match keyring_args.command {
            KeyringCommands::Create => {
                let ans = Confirm::new("Would you like to generate a new seed phrase?")
                    .with_default(true)
                    .prompt();
                match ans {
                    Ok(true) => keyring::create_keyring().unwrap(),
                    Ok(false) => {
                        println!("Keyring creation cancelled.");
                        return;
                    },
                    Err(_) => {
                        eprintln!("Error: Failed to get user confirmation for keyring creation.");
                        return;
                    }
                }
            }
            KeyringCommands::Inspect => {
                let ans = Confirm::new("Are you sure you want to inspect the keyring?")
                    .with_default(false)
                    .with_help_message("This will reveal secrets in the keyring.")
                    .prompt();
                match ans {
                    Ok(true) => keyring::inspect_keyring().unwrap(),
                    Ok(false) => {
                        println!("Keyring inspection cancelled.");
                        return;
                    },
                    Err(_) => {
                        eprintln!("Error: Failed to get user confirmation for keyring inspection.");
                        return;
                    }
                }
            }
        },
        BlindProxyCommands::Login => {
            //accounts_api::accounts_login(configuration, default_login);
            let username = Text::new("Email")
                .with_help_message("This doubles as your username.")
                .prompt();
            let password = Password::new("Password")
                .with_display_toggle_enabled()
                .with_display_mode(PasswordDisplayMode::Masked)
                .with_help_message("Enter your password.")
                .prompt();

            match (username, password) {
                (Ok(user), Ok(pass)) => {
                    rt.block_on(async {
                        if let Err(e) = keyring::blind_login(&user, &pass).await {
                            eprintln!("Login failed: {}", e);
                            return;
                        }
                    });
                }
                (Err(e), _) => {
                    eprintln!("Error reading username: {}", e);
                    return;
                }
                (_, Err(e)) => {
                    eprintln!("Error reading password: {}", e);
                    return;
                }                
            }

            println!("Credentials saved.");
        }
        BlindProxyCommands::Organization(org_args) => match org_args.command {
            OrganizationCommands::List => {
                rt.block_on(async {
                    match organizations::list_all_organizations().await  {
                        Ok(orgs) => {
                            if orgs.is_empty() {
                                println!("No organizations found.");
                            } else {
                                for org in orgs {
                                    println!("{}", org);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("List organizations failed: {}", e);
                            return;
                        }
                    }
                });
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
