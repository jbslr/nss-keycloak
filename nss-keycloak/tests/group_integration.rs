use libnss::group::GroupHooks;
use libnss::interop::Response;

fn response_type_as_str<T>(response: &Response<T>) -> &str {
    match response {
        Response::Success(_) => "Success",
        Response::NotFound => "NotFound",
        Response::Unavail => "Unavail",
        Response::TryAgain => "TryAgain",
        Response::Return => "Return",
    }
}

#[test]
fn test_get_all_entries() {
    temp_env::with_var(
        "NSSKEYCLOAK_CONFIG_FILE",
        Some("tests/files/config.toml"),
        || {
            let response = nss_keycloak::KeycloakNssGroup::get_all_entries();
            match response {
                Response::Success(groups) => {
                    assert_eq!(groups.len(), 2);
                    assert_eq!(groups[0].name, "group01");
                    assert_eq!(groups[0].gid, 500);
                    assert_eq!(groups[0].members, vec!["user01",]);
                    assert_eq!(groups[1].name, "group02");
                    assert_eq!(groups[1].gid, 501);
                    assert_eq!(groups[1].members, vec!["user02",]);
                }
                _ => panic!(
                    "Failed to get all groups. Expected Respose::Success, got {}",
                    response_type_as_str(&response),
                ),
            };
        },
    );
}

#[test]
fn test_get_entry_by_name() {
    temp_env::with_var(
        "NSSKEYCLOAK_CONFIG_FILE",
        Some("tests/files/config.toml"),
        || {
            let response = nss_keycloak::KeycloakNssGroup::get_entry_by_name("group02".to_string());
            match response {
                Response::Success(group) => {
                    assert_eq!(group.name, "group02");
                    assert_eq!(group.gid, 501);
                    assert_eq!(group.members, vec!["user02",]);
                }
                _ => panic!(
                    "Failed to get group by name. Expected Respose::Success, got {}",
                    response_type_as_str(&response),
                ),
            };
        },
    );
}

#[test]
fn test_get_entry_by_name_not_found() {
    temp_env::with_var(
        "NSSKEYCLOAK_CONFIG_FILE",
        Some("tests/files/config.toml"),
        || {
            let response = nss_keycloak::KeycloakNssGroup::get_entry_by_name("group03".to_string());
            match response {
                Response::NotFound => (),
                _ => panic!(
                    "Failed to get group by name. Expected Respose::NotFound, got {}",
                    response_type_as_str(&response),
                ),
            };
        },
    );
}

#[test]
fn test_get_entry_by_gid() {
    temp_env::with_var(
        "NSSKEYCLOAK_CONFIG_FILE",
        Some("tests/files/config.toml"),
        || {
            let response = nss_keycloak::KeycloakNssGroup::get_entry_by_gid(501);
            match response {
                Response::Success(group) => {
                    assert_eq!(group.name, "group02");
                    assert_eq!(group.gid, 501);
                    assert_eq!(group.members, vec!["user02",]);
                }
                _ => panic!(
                    "Failed to get group by gid. Expected Respose::Success, got {}",
                    response_type_as_str(&response),
                ),
            };
        },
    );
}

#[test]
fn test_get_entry_by_gid_not_found() {
    temp_env::with_var(
        "NSSKEYCLOAK_CONFIG_FILE",
        Some("tests/files/config.toml"),
        || {
            let response = nss_keycloak::KeycloakNssGroup::get_entry_by_gid(502);
            match response {
                Response::NotFound => (),
                _ => panic!(
                    "Failed to get group by gid. Expected Respose::NotFound, got {}",
                    response_type_as_str(&response),
                ),
            };
        },
    );
}
