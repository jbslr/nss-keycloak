use std::collections::BTreeMap;

#[derive(Debug, serde::Deserialize)]
pub(super) struct KeycloakUserResponse {
    pub(super) username: String,
    pub(super) enabled: bool,
    pub(super) attributes: Option<BTreeMap<String, Vec<String>>>,
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct KeycloakGroupResponse {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) attributes: Option<BTreeMap<String, Vec<String>>>,
}
