mod model;

use anyhow::Result;

pub use model::{Config, KeycloakConfig, MappingConfig};

pub const CONFIG_ENV: &str = "NSSKEYCLOAK_CONFIG_FILE";
const CONFIG_DEFAULT_FILE: &str = "/etc/nss-keycloak/config.toml";

/// Load the configuration file defined by the environment variable `NSSKEYCLOAK_CONFIG_FILE` or
/// use the default configuration file path `/etc/nss-keycloak/config.toml`.
pub fn load_config() -> Result<Config> {
    let path = get_config_path();
    read_config_file(&path)
}

/// Get the path to the configuration file from the environment variable or use the default
/// configuration file path. ENV is `NSSKEYCLOAK_CONFIG_FILE` and the default is
/// `/etc/nss-keycloak/config.toml`.
pub fn get_config_path() -> String {
    std::env::var(CONFIG_ENV).unwrap_or(CONFIG_DEFAULT_FILE.to_string())
}

/// Read the configuration file from the given path and parse it into a `Config` struct.
pub fn read_config_file(path: &str) -> Result<Config> {
    let file = std::path::Path::new(path);
    let buf = std::fs::read_to_string(file)?;
    let config = toml::from_str::<Config>(&buf)?;
    Ok(config)
}

// -----------------------------------------------------------------------------------------------
// --- Unit tests -------------------------------------------------------------------------------
// -----------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;
    use tempfile;

    /// Test that the configuration file path is read from the environment variable
    #[test]
    fn test_get_config_path_from_env() {
        const CONFIG_FILE: &str = "/tmp/config.toml";
        temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some(CONFIG_FILE), || {
            assert_eq!(get_config_path(), CONFIG_FILE);
        });
    }

    /// Test that the default configuration file path is used when the environment variable 
    /// is not set
    #[test]
    fn test_get_config_path_from_default() {
        temp_env::with_var_unset("NSSKEYCLOAK_CONFIG_FILE", || {
            assert_eq!(get_config_path(), "/etc/nss-keycloak/config.toml");
        });
    }

    /// Test that a configuration file in TOML format is parsed corretly
    #[test]
    fn test_read_config_file() {
        // prepare temp file with the configuration in TOML format
        let config_content = r#"
            [keycloak]
            realm = "myrealm"
            url = "http://localhost:8080/auth"
            client_id = "myclient"
            client_secret = "mysecret"
            username = "myuser"
            password = "mypassword"

            [mapping]
            user_home = "homedirectory"
            user_shell = "defaultshell"
            user_gecos = "gecos"
            user_uid = "uidnumber"
            user_gid = "gidnumber"
            group_gid = "gidnumber"
        "#;
        let expected = Config {
            keycloak: KeycloakConfig {
                realm: "myrealm".to_string(),
                client_id: "myclient".to_string(),
                client_secret: "mysecret".to_string(),
                url: "http://localhost:8080/auth".to_string(),
                username: "myuser".to_string(),
                password: "mypassword".to_string(),
            },
            mapping: MappingConfig {
                user_home: "homedirectory".to_string(),
                user_shell: "defaultshell".to_string(),
                user_gecos: "gecos".to_string(),
                user_uid: "uidnumber".to_string(),
                user_gid: "gidnumber".to_string(),
                group_gid: "gidnumber".to_string(),
            },
        };
        let mut tmp = tempfile::NamedTempFile::new().unwrap();
        writeln!(tmp, "{}", config_content).unwrap();
        let config = read_config_file(tmp.path().to_str().unwrap()).unwrap();
        assert_eq!(config, expected);
    }
}
