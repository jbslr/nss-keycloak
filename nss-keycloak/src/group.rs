use libnss::group::{Group, GroupHooks};
use libnss::interop::Response;

use crate::keycloak::auth::TokenProvider;
use crate::keycloak::groups::{get_group_by_gid, get_group_by_name, list_groups, KeycloakGroup};

pub struct KeycloakNssGroup;

impl From<KeycloakGroup> for Group {
    fn from(group: KeycloakGroup) -> Self {
        Group {
            name: group.name,
            passwd: "x".to_string(),
            gid: group.gid,
            members: group.members,
        }
    }
}

impl GroupHooks for KeycloakNssGroup {
    /// Get all groups from Keycloak
    /// calls keycloak::list_groups underneath
    fn get_all_entries() -> Response<Vec<Group>> {
        let access_token = match crate::AUTH.lock().unwrap().get_access_token() {
            Ok(token) => token.clone(),
            Err(err) => {
                log::error!("Failed to get access token: {:?}", err);
                return Response::TryAgain;
            }
        };
        match list_groups(
            &crate::CONFIG.keycloak,
            &crate::CONFIG.mapping,
            &access_token,
        ) {
            Ok(groups) => {
                Response::Success(groups.into_iter().map(|group| Group::from(group)).collect())
            }
            Err(err) => {
                log::error!("Failed to get all groups: {:?}", err);
                Response::TryAgain
            }
        }
    }

    /// Get a group by gid
    /// calls keycloak::get_group_by_gid underneath
    /// Returns Response::Success if group is found
    /// Returns Response::NotFound if group is not found
    /// Returns Response::Unavail if there was an error
    fn get_entry_by_gid(gid: libc::gid_t) -> Response<Group> {
        let access_token = match crate::AUTH.lock().unwrap().get_access_token() {
            Ok(token) => token.clone(),
            Err(err) => {
                log::error!("Failed to get access token: {:?}", err);
                return Response::TryAgain;
            }
        };
        match get_group_by_gid(
            &crate::CONFIG.keycloak,
            &crate::CONFIG.mapping,
            &access_token,
            gid,
        ) {
            Ok(None) => Response::NotFound,
            Ok(Some(group)) => Response::Success(Group::from(group)),
            Err(err) => {
                log::error!("Failed to get group by gid: {:?}", err);
                Response::TryAgain
            }
        }
    }

    /// Get a group by name
    /// calls keycloak::get_group_by_name underneath
    /// Returns Response::Success if group is found
    /// Returns Response::NotFound if group is not found
    /// Returns Response::Unavail if there was an error
    fn get_entry_by_name(name: String) -> Response<Group> {
        let access_token = match crate::AUTH.lock().unwrap().get_access_token() {
            Ok(token) => token.clone(),
            Err(err) => {
                log::error!("Failed to get access token: {:?}", err);
                return Response::TryAgain;
            }
        };
        let group = get_group_by_name(
            &crate::CONFIG.keycloak,
            &crate::CONFIG.mapping,
            &access_token,
            &name,
        );
        match group {
            Err(err) => {
                log::error!("Failed to get group by name: {:?}", err);
                Response::TryAgain
            }
            Ok(None) => Response::NotFound,
            Ok(Some(group)) => Response::Success(Group::from(group)),
        }
    }
}
