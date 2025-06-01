/// keyring-related functions and structures
use rand::{rngs::OsRng, TryRngCore};
use crate::{
    bip39::generate_mnemonic_from_entropy,
    error::ProxyError,
};
use keyring::Entry;
use blind_wrapper_rs::apis::{accounts_api, configuration::Configuration};
use std::sync::Arc;
use reqwest::{cookie::{CookieStore, Jar}, Url};

const KEYRING_SERVICE: &str = "blind-proxy-rs";
const KEYRING_USERNAME: &str = "proxy-user";
const LOGIN_TARGET: &str = "blind-insight-login";
const PASSWORD_TARGET: &str = "blind-insight-password";
const SEED_TARGET: &str = "seed";
const COOKIES_TARGET: &str = "blind-insight-cookies";

/// generate a new keyring
pub fn generate_seed_phrase() -> Result<String, ProxyError> {
    let mut rng = OsRng;
    
    let mut entropy = [0u8; 32]; // 256 bits of entropy
    rng.try_fill_bytes(&mut entropy)?;

    // Generate a BIP39 mnemonic from the entropy
    generate_mnemonic_from_entropy(&entropy)
}

pub fn create_keyring() -> Result<(), ProxyError> {
    let seed_phrase = generate_seed_phrase()?;
    let entry = Entry::new_with_target(SEED_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    entry.set_password(&seed_phrase)?;
    
    Ok(())
}

pub fn store_blind_login_credentials(username: &str, password: &str) -> Result<(), ProxyError> {
    let entry = Entry::new_with_target(LOGIN_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    entry.set_password(&username)?;
    let entry = Entry::new_with_target(PASSWORD_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    entry.set_password(&password)?;
    
    Ok(())
}

pub fn get_blind_login_credentials() -> Result<(String, String), ProxyError> {
    let entry = Entry::new_with_target(LOGIN_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    let username = entry.get_password()?;
    
    let entry = Entry::new_with_target(PASSWORD_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    let password = entry.get_password()?;
    
    Ok((username, password))
}

pub fn store_cookies(cookies: &str) -> Result<(), ProxyError> {
    let entry = Entry::new_with_target(COOKIES_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    entry.set_password(cookies)?;
    
    Ok(())
}

pub fn get_cookies() -> Result<String, ProxyError> {
    let entry = Entry::new_with_target(COOKIES_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    Ok(entry.get_password()?)
}

pub fn inspect_keyring() -> Result<(), ProxyError> {
    let seed_entry = Entry::new_with_target(SEED_TARGET, KEYRING_SERVICE, KEYRING_USERNAME)?;
    match seed_entry.get_password() {
        Ok(seed) => println!("Your seed phrase: {}", seed),
        Err(_) => println!("No seed phrase set. Call keyring create to create one."),
    }

    match get_blind_login_credentials() {
        Ok((username, password)) => {
            println!("Your Credentials:");
            println!("    Email: {}", username);
            println!("    Password: {}", password);
        },
        Err(_) => println!("No login credentials found."),
    }
    
    Ok(())
}

fn get_blind_insight_configuration() -> Configuration {
    match get_cookies() {
        Ok(cookies) => {
            let jar = Arc::new(Jar::default());
            jar.add_cookie_str(&cookies, &Url::parse("https://api.beta.blindinsight.io").unwrap());
            Configuration::new_with_cookie_jar(jar)
        },
        Err(_) => Configuration::new(),
    }
}

pub async fn blind_login(username: &str, password: &str) -> Result<(), ProxyError> {

    // need to save the username and password in the keyring

    let jar = Arc::new(Jar::default());

    let mut configuration = Configuration::new();
    configuration.client = reqwest::Client::builder()
        .cookie_provider(jar.clone())
        .build()
        .expect("Failed to create reqwest client with cookie jar");
    let default_login = blind_wrapper_rs::models::DefaultLogin::new(username.to_string(), password.to_string());

    match accounts_api::accounts_login(&configuration, default_login).await {
        Ok(response) => {
            let url = Url::parse("https://api.beta.blindinsight.io").unwrap();
            
            if let Some(cookie_header) = jar.cookies(&url) {
                if let Ok(cookie_str) = cookie_header.to_str() {
                    store_cookies(cookie_str)?;
                }
            }
            
            store_blind_login_credentials(username, password)?;
            println!("Login successful: {:?}", response);
            Ok(())
        },
        Err(e) => {
            match e {
                blind_wrapper_rs::apis::Error::ResponseError(response) => {
                    if let Some(entity) = response.entity {
                        match entity {
                            blind_wrapper_rs::apis::accounts_api::AccountsLoginError::Status400(details) => {
                                if let Some(d) = details.detail {
                                    println!("{}", d);
                                };
                            },
                            _ => {
                                println!("Unexpected error: {:?}", entity);
                            }
                        }
                    }
                    Err(ProxyError::LoginFailure(response.status.to_string()))
                },
                _ => Err(ProxyError::Unknown(e.to_string())),
            }
        }
    }
}