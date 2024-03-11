use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use reqwest::blocking::Client;

use crate::config::{KeycloakConfig, MappingConfig};

use super::model::{KeycloakGroupResponse, KeycloakUserResponse};

/// Data struct representing a group from Keycloak
#[derive(Debug)]
pub(crate) struct KeycloakGroup {
    pub(crate) name: String,
    pub(crate) gid: libc::gid_t,
    pub(crate) members: Vec<String>,
}

/// Get a single attribute from a Keycloak user response.
/// Return none if no such attribute is available
/// Return an error if multiple values are found
fn get_single_attribute<'a>(
    attributes: &'a Option<BTreeMap<String, Vec<String>>>,
    attr_name: &str,
) -> Result<Option<&'a String>> {
    match attributes {
        None => Ok(None),
        Some(ref attributes) => match attributes.get(attr_name) {
            Some(values) if values.len() == 1 => Ok(values.first()),
            Some(values) if values.len() > 1 => Err(anyhow!("multiple {} found", attr_name)),
            _ => Ok(None),
        },
    }
}

/// Get Keycloak API URL for retrieving groups from Keycloak.
fn get_groups_url(config: &KeycloakConfig) -> String {
    format!("{}/admin/realms/{}/groups", config.url, config.realm)
}

/// Get the URL for retrieving members of a specific group from Keycloak.
fn get_group_members_url(config: &KeycloakConfig, group_id: &str) -> String {
    format!(
        "{}/admin/realms/{}/groups/{}/members",
        config.url, config.realm, group_id
    )
}

/// Send a request to retrieve the members of a specific group from Keycloak.
fn group_member_request(
    config: &KeycloakConfig,
    client: &Client,
    access_token: &str,
    group_id: &str,
) -> Result<Vec<String>> {
    let url = get_group_members_url(config, group_id);
    let response = client.get(url).bearer_auth(access_token).send()?;
    let members = serde_json::from_str::<Vec<KeycloakUserResponse>>(&response.text()?)?;
    Ok(members.into_iter().map(|member| member.username).collect())
}

/// Send a request to retrieve groups from Keycloak.
fn groups_request(
    keycloak_config: &KeycloakConfig,
    attribute_mapping: &MappingConfig,
    access_token: &str,
    params: &[(&str, &str)],
    client: &Client,
) -> Result<Vec<KeycloakGroupResponse>> {
    let url = get_groups_url(keycloak_config);
    let mut response = client
        .get(url)
        .query(params)
        .bearer_auth(access_token)
        .send()?;
    Ok(serde_json::from_str::<Vec<KeycloakGroupResponse>>(
        &response.text()?,
    )?)
}

fn add_group_members(
    config: &KeycloakConfig,
    client: &Client,
    access_token: &str,
    attribute_mapping: &MappingConfig,
    group: KeycloakGroupResponse,
) -> Result<KeycloakGroup> {
    let members = group_member_request(config, &client, access_token, &group.id);
    Ok(KeycloakGroup {
        name: group.name,
        gid: get_single_attribute(&group.attributes, &attribute_mapping.group_gid)?
            .ok_or(anyhow!(
                "Missing required attribute {}",
                attribute_mapping.group_gid
            ))?
            .parse()?,
        members: members?,
    })
}

/// List all groups from Keycloak.
pub(crate) fn list_groups(
    config: &KeycloakConfig,
    attribute_mapping: &MappingConfig,
    access_token: &str,
) -> Result<Vec<KeycloakGroup>> {
    let client = Client::new();
    Ok(groups_request(
        config,
        attribute_mapping,
        access_token,
        &[("briefRepresentation", "false")],
        &client,
    )?
    .into_iter()
    .map(|group| add_group_members(config, &client, access_token, attribute_mapping, group))
    .filter_map(|g| g.ok())
    .collect())
}

pub(crate) fn get_group_by_name(
    config: &KeycloakConfig,
    attribute_mapping: &MappingConfig,
    access_token: &str,
    name: &str,
) -> Result<Option<KeycloakGroup>> {
    let client = Client::new();
    Ok(groups_request(
        config,
        attribute_mapping,
        access_token,
        &[("search", name), ("briefRepresentation", "false")],
        &client,
    )?
    .into_iter()
    .map(|group| add_group_members(config, &client, access_token, attribute_mapping, group))
    .filter_map(|g| g.ok())
    .next())
}

pub(crate) fn get_group_by_gid(
    config: &KeycloakConfig,
    attribute_mapping: &MappingConfig,
    access_token: &str,
    gid: libc::gid_t,
) -> Result<Option<KeycloakGroup>> {
    let client = Client::new();
    let gid_str = gid.to_string();
    Ok(groups_request(
        config,
        attribute_mapping,
        access_token,
        &[("briefRepresentation", "false")],
        &client,
    )?
    .into_iter()
    .filter(
        |group| match get_single_attribute(&group.attributes, &attribute_mapping.group_gid) {
            Ok(Some(gid)) => *gid == gid_str,
            _ => false,
        },
    )
    .map(|group| add_group_members(config, &client, access_token, attribute_mapping, group))
    .filter_map(|g| g.ok())
    .next())
}
