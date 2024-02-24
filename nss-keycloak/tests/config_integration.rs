use temp_env;

use nss_keycloak::CONFIG;

#[test]
fn test_get_config_path_from_env() {

    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        // config.keycloak
        assert_eq!(CONFIG.keycloak.realm, "myrealm");
        assert_eq!(CONFIG.keycloak.client_id, "myclient");
        assert_eq!(CONFIG.keycloak.client_secret, "mysecret");
        assert_eq!(CONFIG.keycloak.url, "http://localhost:8080/auth");
        assert_eq!(CONFIG.keycloak.username, "myuser");
        assert_eq!(CONFIG.keycloak.password, "mypassword");
        // config.mapping
        assert_eq!(CONFIG.mapping.user_home, "homedirectory");
        assert_eq!(CONFIG.mapping.user_shell, "defaultshell");
        assert_eq!(CONFIG.mapping.user_gecos, "gecos");
        assert_eq!(CONFIG.mapping.user_uid, "uidnumber");
        assert_eq!(CONFIG.mapping.user_gid, "gidnumber");
        assert_eq!(CONFIG.mapping.group_gid, "gidnumber");
    });
}
