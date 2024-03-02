use libnss::passwd::PasswdHooks;
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
fn test_password_get_all_entries() {
    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        let response = nss_keycloak::KeycloakNssPasswd::get_all_entries();
        match response {
            Response::Success(passwds) => {
                assert_eq!(passwds.len(), 2);
                // test expected values for user 01
                assert_eq!(passwds[0].name, "user01");
                assert_eq!(passwds[0].uid, 1000);
                assert_eq!(passwds[0].gid, 500);
                assert_eq!(passwds[0].dir, "/home/user01");
                assert_eq!(passwds[0].gecos, ",,,");
                assert_eq!(passwds[0].shell, "/bin/bash");
                // test expected values for user 02
                assert_eq!(passwds[1].name, "user02");
                assert_eq!(passwds[1].uid, 1001);
                assert_eq!(passwds[1].gid, 501);
                assert_eq!(passwds[1].dir, "/home/user02");
                assert_eq!(passwds[1].gecos, ",,,");
                assert_eq!(passwds[1].shell, "/bin/bash");
            },
            _ => panic!(
                "Failed to get all passwds. Expected Respose::Success, got {}", 
                response_type_as_str(&response),
            ),
        }
    });
}

#[test]
fn test_get_user_by_name_found() {
    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        let username = "user01".to_string();
        let response = nss_keycloak::KeycloakNssPasswd::get_entry_by_name(username);
        match response {
            Response::Success(passwd) => {
                assert_eq!(passwd.name, "user01");
                assert_eq!(passwd.uid, 1000);
                assert_eq!(passwd.gid, 500);
                assert_eq!(passwd.dir, "/home/user01");
                assert_eq!(passwd.gecos, ",,,");
                assert_eq!(passwd.shell, "/bin/bash");
            },
            _ => panic!(
                "Failed to get user by name. Expected Respose::Success, got {}", 
                response_type_as_str(&response),
            ),
        }
    });
}

#[test]
fn test_get_user_by_name_not_found() {
    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        let username = "does_not_exist".to_string(); // username does not exist
        let response = nss_keycloak::KeycloakNssPasswd::get_entry_by_name(username);
        match response {
            Response::NotFound => (), // expected
            _ => panic!(
                "Failed to get user by name. Expected Respose::NotFound, got {}", 
                response_type_as_str(&response),
            ),
        }
    });
}

#[test]
fn test_get_user_by_uid_found() {
    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        let uid = 1000;
        let response = nss_keycloak::KeycloakNssPasswd::get_entry_by_uid(uid);
        match response {
            Response::Success(passwd) => {
                assert_eq!(passwd.name, "user01");
                assert_eq!(passwd.uid, 1000);
                assert_eq!(passwd.gid, 500);
                assert_eq!(passwd.dir, "/home/user01");
                assert_eq!(passwd.gecos, ",,,");
                assert_eq!(passwd.shell, "/bin/bash");
            },
            _ => panic!(
                "Failed to get user by uid. Expected Respose::Success, got {}", 
                response_type_as_str(&response),
            ),
        }
    });
}

#[test]
fn test_get_user_by_uid_not_found() {
    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        let uid = 9999; // uid does not exist
        let response = nss_keycloak::KeycloakNssPasswd::get_entry_by_uid(uid);
        match response {
            Response::NotFound => (), // expected
            _ => panic!("Failed to get user by uid. Expected Respose::NotFound, got {}", response_type_as_str(&response)),
        }
    });
}
