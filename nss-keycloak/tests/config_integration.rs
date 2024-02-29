use temp_env;

use nss_keycloak::CONFIG;

#[test]
fn test_get_config_path_from_env() {

    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        // config.keycloak
        assert_eq!(CONFIG.keycloak.realm, "test");
        assert_eq!(CONFIG.keycloak.client_id, "nss-client");
        assert_eq!(CONFIG.keycloak.client_secret, "UO2i2h5Vku3oQBvFIlsMG23pIMZyRJOi");
        assert_eq!(CONFIG.keycloak.url, "http://localhost:8080");
        assert_eq!(CONFIG.keycloak.username, "nss-user");
        assert_eq!(CONFIG.keycloak.password, "nss-user");
        // config.mapping
        assert_eq!(CONFIG.mapping.user_home, "homedirectory");
        assert_eq!(CONFIG.mapping.user_shell, "loginshell");
        assert_eq!(CONFIG.mapping.user_gecos, "gecos");
        assert_eq!(CONFIG.mapping.user_uid, "uidnumber");
        assert_eq!(CONFIG.mapping.user_gid, "gidnumber");
        assert_eq!(CONFIG.mapping.group_gid, "gidnumber");
    });
}
