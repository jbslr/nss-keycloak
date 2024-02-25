use std::alloc::System;
use std::ops::{Add, Sub};
use std::time::Duration;

#[cfg(feature = "mock")]
use mock_instant::SystemTime;
#[cfg(not(feature = "mock"))]
use std::time::SystemTime;

use anyhow::{Ok, Result};
use serde::Deserialize;

use crate::config::KeycloakConfig;

// some time buffer to avoid token expiration issues
// between the time we check validate the token and the time we use it
const TIMEBUFFER: Duration = Duration::from_secs(3);

/// test if the access token is still valid.
fn access_token_is_valid(token: &KeycloakToken) -> bool {
    token.access_token_expiration > SystemTime::now()
}

/// test if the refresh token is still valid.
fn refresh_token_is_valid(token: &KeycloakToken) -> bool {
    token.refresh_token_expiration > SystemTime::now()
}

/// parse the Keycloak response and format it into a KeycloakToken
fn format_token(json_response: &str, request_time: &SystemTime) -> Result<KeycloakToken> {
    /// data structure for the response of the token endpoint
    /// contains the access token, its expiration time, the refresh token and its expiration time
    #[derive(Deserialize)]
    struct KeycloakTokenResponse {
        access_token: String,
        expires_in: u64,
        refresh_token: String,
        refresh_expires_in: u64,
    }
    let token_response = serde_json::from_str::<KeycloakTokenResponse>(json_response)?;
    // calculate the expiration time of the access token and the refresh token, and 
    // subtract a time buffer to expiration issues near the expiration time
    let access_token_expiration = request_time
        .add(Duration::from_secs(token_response.expires_in))
        .sub(TIMEBUFFER);
    let refresh_token_expiration = request_time
        .add(Duration::from_secs(token_response.refresh_expires_in))
        .sub(TIMEBUFFER);
    Ok(KeycloakToken {
        access_token: token_response.access_token,
        access_token_expiration,
        refresh_token: token_response.refresh_token,
        refresh_token_expiration,
    })
}

/// fetch a new access token from Keycloak using the given config
fn get_token(config: &KeycloakConfig) -> Result<KeycloakToken> {
    let client = reqwest::blocking::Client::new();
    let request_time = SystemTime::now();
    let response = client
        .post(&format!("{}/realms/{}/protocol/openid-connect/token", config.url, config.realm))
        .form(&[
            ("grant_type", "password"),
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
            ("username", &config.username),
            ("password", &config.password),
        ])
        .send()?;
    format_token(&response.text()?, &request_time)
}

/// refresh the access token using the refresh token
fn refresh_token(config: &KeycloakConfig, token: &KeycloakToken) -> Result<KeycloakToken> {
    let client = reqwest::blocking::Client::new();
    let request_time = SystemTime::now();
    let response = client
        .post(&format!("{}/realms/{}/protocol/openid-connect/token", config.url, config.realm))
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
            ("refresh_token", &token.refresh_token),
        ])
        .send()?;
    format_token(&response.text()?, &request_time)
}

/// data structure for a Keycloak token
/// contains the access token and its expiration date
/// and the refresh token and its expiration date
struct KeycloakToken {
    access_token: String,
    access_token_expiration: SystemTime,
    refresh_token: String,
    refresh_token_expiration: SystemTime,
}

pub struct KeycloakAuth<'a> {
    keycloak_config: &'a KeycloakConfig,
    token: Option<KeycloakToken>,
}

/// Base trait for Keycloak authentication
/// defined the get_token method, which should always check
/// the validity of the token before returning it
pub trait TokenProvider {
    fn get_access_token(&mut self) -> Result<&String>;
    fn has_valid_token(&self) -> bool;
}

impl TokenProvider for KeycloakAuth<'_> {

    /// Will check if the KeycloakAuth instance has a valid token
    /// if yes, return the access token of that token
    /// if no, try to get a new token using the refresh token if it is still valid
    /// or get a new token using the direct access grant flow
    fn get_access_token(&mut self) -> Result<&String> {
        return {
            if self.token.is_some() && access_token_is_valid(self.token.as_ref().unwrap()) {
            // token is valid, return access token
                Ok(&self.token.as_ref().unwrap().access_token)
            } else if self.token.is_some() && refresh_token_is_valid(self.token.as_ref().unwrap()) {
                // refresh token is valid, get a new access token, then return it
                self.token = Some(refresh_token(
                    self.keycloak_config,
                    self.token.as_ref().unwrap(),
                )?);
                Ok(&self.token.as_ref().unwrap().access_token)
            } else {
                // no token or no valid token, get a new token using the direct access grant flow
                self.token = Some(get_token(self.keycloak_config)?);
                Ok(&self.token.as_ref().unwrap().access_token)
            }
        }
    }

    fn has_valid_token(&self) -> bool {
        match self.token {
            Some(ref token) => access_token_is_valid(token) || refresh_token_is_valid(token),
            None => false,
        }
    }
}

impl KeycloakAuth<'_> {

    /// create a new KeycloakAuth instance
    /// try to get a token from the Keycloak server using the direct access grant flow
    pub fn new(keycloak_config: &KeycloakConfig) -> Result<KeycloakAuth> {
        Ok(KeycloakAuth {
            keycloak_config,
            token: None,
        })
    }

    /// get the expiration time of the access token
    /// mainly for testing purposes
    pub fn access_token_expires_in(&self) -> Option<Duration> {
        match self.token {
            Some(ref token) => token.access_token_expiration.duration_since(SystemTime::now()).ok(),
            None => None,
        }
    }

    /// get the expiration time of the refresh token
    /// mainly for testing purposes
    pub fn refresh_token_expires_in(&self) -> Option<Duration> {
        match self.token {
            Some(ref token) => token.refresh_token_expiration.duration_since(SystemTime::now()).ok(),
            None => None,
        }
    }
}
