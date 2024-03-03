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
    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        let response = nss_keycloak::KeycloakNssGroup::get_all_entries();
        match response {
            Response::Success(groups) => {
                assert_eq!(groups.len(), 2);
                assert_eq!(groups[0].name, "group01");
                assert_eq!(groups[0].gid, 500);
                assert_eq!(groups[0].members, vec!["user01", ]);
                assert_eq!(groups[1].name, "group02");
                assert_eq!(groups[1].gid, 501);
                assert_eq!(groups[1].members, vec!["user02", ]);
            },
            _ => panic!(
                "Failed to get all groups. Expected Respose::Success, got {}", 
                response_type_as_str(&response),
            ),
        };
    });
}
