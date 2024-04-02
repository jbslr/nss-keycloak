use serde::Deserialize;
use std::cmp::Eq;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct KeycloakConfig {
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub url: String,
    // optional parameters. If provided, will request password grant type
    // else, request client credentials grant type
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MappingConfig {
    pub user_home: String,
    pub user_shell: String,
    pub user_gecos: String,
    pub user_uid: String,
    pub user_gid: String,
    pub group_gid: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub keycloak: KeycloakConfig,
    pub mapping: MappingConfig,
}
