use std::collections::BTreeMap;

use serde::Deserialize;
use reqwest::blocking::Client;
use anyhow::{anyhow, Ok, Result};

use crate::config::{KeycloakConfig, MappingConfig};

/// batch size for the Keycloak user list API
const BATCH_SIZE : usize = 100;

/// Data struct for a Keycloak user
#[derive(Debug)]
pub struct KeycloakUser {
    pub username: String,
    pub uid: libc::uid_t,
    pub gid: libc::gid_t,
    pub homedir: String,
    pub loginshell: String,
    pub gecos: String,
}

#[derive(Deserialize)]
struct KeycloakUserResponse {
    username: String,
    enabled: bool,
    attributes: Option<BTreeMap<String, Vec<String>>>,
}

struct MappedKeycloakUserResponse<'a> {
    response: &'a KeycloakUserResponse,
    mapping: &'a MappingConfig,
}

/// Get a single attribute from a Keycloak user response.
/// Return none if no such attribute is available
/// Return an error if multiple values are found
fn get_single_attribute<'a>(attributes: &'a Option<BTreeMap<String, Vec<String>>>, attr_name: &str) -> Result<Option<&'a String>> {
    match attributes {
        None => Ok(None),
        Some(ref attributes) => match attributes.get(attr_name) {
            Some(values) if values.len() == 1 => Ok(values.first()),
            Some(values) if values.len() > 1 => Err(anyhow!("multiple {} found", attr_name)),
            _ => Ok(None),
        }
    }
}

/// MappedKeycloakUserResponse is a wrapper around KeycloakUserResponse that provides
/// methods to access user attributes according to the mapping configuration
impl MappedKeycloakUserResponse<'_> {
    fn new<'a>(response: &'a KeycloakUserResponse, mapping: &'a MappingConfig) -> MappedKeycloakUserResponse<'a> {
        MappedKeycloakUserResponse {
            response,
            mapping,
        }
    }

    /// Get the user's home directory
    fn get_user_home(&self) -> Result<Option<&String>> {
        get_single_attribute(&self.response.attributes, &self.mapping.user_home)
    }

    /// Get the user's login shell
    fn get_user_shell(&self) -> Result<Option<&String>> {
        get_single_attribute(&self.response.attributes, &self.mapping.user_shell)
    }

    /// Get the user's gecos field
    fn get_user_gecos(&self) -> Result<Option<&String>> {
        get_single_attribute(&self.response.attributes, &self.mapping.user_gecos)
    }

    /// Get the user's uid
    fn get_user_uid(&self) -> Result<Option<&String>> {
        get_single_attribute(&self.response.attributes, &self.mapping.user_uid)
    }

    /// Get the user's gid
    fn get_user_gid(&self) -> Result<Option<&String>> {
        get_single_attribute(&self.response.attributes, &self.mapping.user_gid)
    }
}

/// Implement TryFrom for KeycloakUser to allow conversion from MappedKeycloakUserResponse
impl TryFrom<MappedKeycloakUserResponse<'_>> for KeycloakUser {
    type Error = anyhow::Error;

    fn try_from(value: MappedKeycloakUserResponse) -> Result<Self> {
        // defaults if no value is found
        let default_homedir = "/".to_string();
        let default_loginshell = "/sbin/nologin".to_string();
        let default_gecos = ",,,".to_string();

        let uid = value.get_user_uid()?;
        let gid = value.get_user_gid()?;
        let homedir = value.get_user_home()?;
        let loginshell = value.get_user_shell()?;
        let gecos = value.get_user_gecos()?;
        Ok(KeycloakUser {
            username: value.response.username.to_owned(),
            uid: uid.ok_or(anyhow!("uid not found"))?.parse()?,
            gid: gid.ok_or(anyhow!("gid not found"))?.parse()?,
            homedir: homedir.or(Some(&default_homedir)).unwrap().to_owned(),
            loginshell: loginshell.or(Some(&default_loginshell)).unwrap().to_owned(),
            gecos: gecos.or(Some(&default_gecos)).unwrap().to_owned(),
        })
    }
}

fn get_users_api_url(config: &KeycloakConfig) -> String {
    format!("{}/admin/realms/{}/users", config.url, config.realm)
}

fn get_user_count_api_url(config: &KeycloakConfig) -> String {
    format!("{}/admin/realms/{}/users/count", config.url, config.realm)
}

fn get_number_of_users(config: &KeycloakConfig, access_token: &str, client: &Client) -> Result<libc::uid_t> {
    let response = client
        .get(get_user_count_api_url(config))
        .bearer_auth(access_token)
        .send()?;
    Ok(response.text()?.parse()?)
}

/// Template function to make a request to the Keycloak API to get users
/// Specific request functionalities must be constructed via the 
/// query_args parameter.
/// Returns a vector of KeycloakUser instances or an error if the request fails
fn users_request(
    config: &KeycloakConfig, 
    attribute_mapping: &MappingConfig,
    access_token: &str, 
    query_args: &[(&str, &str)],
    client: &Client,
) -> Result<Vec<KeycloakUser>> {
    let response = client
        .get(get_users_api_url(config))
        .bearer_auth(access_token)
        .query(query_args)
        .send()?;
    let users: Vec<KeycloakUserResponse> = serde_json::from_str(&response.text()?)?;

    Ok(
        users
        .iter()
        .map(|user| MappedKeycloakUserResponse::new(&user, attribute_mapping))
        .map(|mapped_user| KeycloakUser::try_from(mapped_user))
        .filter_map(|res| res.ok())
        .collect::<Vec<KeycloakUser>>()
    )
}

/// List all users from Keycloak
/// This function will make multiple requests to the Keycloak API to get all users
/// Returns a vector of KeycloakUser instances or an error if the request fails
pub fn list_users(
    config: &KeycloakConfig, 
    attribute_mapping: &MappingConfig, 
    access_token: &str,
) -> Result<Vec<KeycloakUser>> {
    let client = Client::new();
    let nusers = get_number_of_users(config, access_token, &client)?;
    let mut users = Vec::with_capacity(nusers as usize);
    for first in (0..nusers).step_by(BATCH_SIZE) {
        users.append(&mut users_request(
            config, 
            attribute_mapping,
            access_token, 
            &[
                ("briefRepresentation", "false"),
                ("first", &format!("{}", first)), 
                ("max", &format!("{}", BATCH_SIZE)),
            ],
            &client,
        )?);
    }
    Ok(users)
}
