use std::cmp::Eq;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct KeycloakConfig {
    pub realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub url: String,
    pub username: String,
    pub password: String,
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
