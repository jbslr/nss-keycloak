use libnss::interop::Response;
use libnss::group::{Group, GroupHooks};

use crate::keycloak::auth::TokenProvider;
use crate::keycloak::groups::{list_groups, KeycloakGroup};

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
        match list_groups(
            &crate::CONFIG.keycloak,
            &crate::CONFIG.mapping,
            crate::AUTH.lock().unwrap().get_access_token().unwrap(),
        ) {
            Ok(groups) => Response::Success(
                groups.into_iter()
                .map(|group| Group::from(group))
                .collect()
            ),
            Err(err) => {
                log::error!("Failed to get all groups: {:?}", err);
                Response::Unavail
            },
        }
    }

    fn get_entry_by_gid(gid: libc::gid_t) -> Response<Group> {
        todo!()
    }

    fn get_entry_by_name(name: String) -> Response<Group> {
        todo!()
    }
}
