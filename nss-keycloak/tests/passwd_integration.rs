use libnss::passwd::PasswdHooks;
use libnss::interop::Response;


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
            _ => panic!("Failed to get all entries"),
        }
    });
}