use std::ops::Add;
use std::time::Duration;

use anyhow::Result;
use mock_instant::MockClock;

use nss_keycloak::{AUTH, CONFIG};
use nss_keycloak::keycloak::auth::TokenProvider;
use nss_keycloak::config::KeycloakConfig;

/// validate token using the Keycloak token introspection endpoint
fn validate_token(
    config: &KeycloakConfig, 
    access_token: &str,
) -> Result<()> {

    #[derive(serde::Deserialize)]
    struct KeycloakIntrospectionResponse {
        active: bool,
    }

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(
            &format!("{}/realms/{}/protocol/openid-connect/token/introspect", 
            config.url, 
            config.realm,
        ))
        .form(&[
            ("token", access_token),
            ("client_id", &config.client_id),
            ("client_secret", &config.client_secret),
        ])
        .send()?;
    let introspection = serde_json::from_str::<KeycloakIntrospectionResponse>(&response.text()?)?;
    if introspection.active {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Token introspection failed"))
    }
}

#[test]
fn test_keycloak_auth_get_token() {
    // warn if the 'mock' feature is not enabled
    // required for modifying the system time
    #[cfg(not(feature = "mock"))]
    panic!("This test requires the 'mock' feature to be enabled");

    #[cfg(feature = "mock")]
    temp_env::with_var("NSSKEYCLOAK_CONFIG_FILE", Some("tests/files/config.toml"), || {
        let access_token01 = AUTH.lock().unwrap().get_access_token().unwrap().to_owned();
        validate_token(&CONFIG.keycloak, &access_token01)
            .expect("Token validation failed");

        // expiration time of the access token (plus 1 second to be above that)
        let access_token_exires_in = AUTH.lock().unwrap().access_token_expires_in().unwrap().add(Duration::from_secs(1));
        MockClock::advance_system_time(access_token_exires_in);
        let access_token02 = AUTH.lock().unwrap().get_access_token().unwrap().to_owned();
        validate_token(&CONFIG.keycloak, &access_token02)
            .expect("Token validation failed");

        // the access tokens should be different because it has been updated 
        // using the refresh operation
        assert_ne!(access_token01, access_token02);

        // expiration time of the refresh token (plus 1 second to be above that)
        let refresh_token_expires_in = AUTH.lock().unwrap().refresh_token_expires_in().unwrap().add(Duration::from_secs(1));
        MockClock::advance_system_time(refresh_token_expires_in);
        let access_token03 = AUTH.lock().unwrap().get_access_token().unwrap().to_owned();
        validate_token(&CONFIG.keycloak, &access_token03)
            .expect("Token validation failed");

        // the access tokens should be different because it has been updated 
        // with a full re-authentication
        assert_ne!(access_token02, access_token03);
    });
}
