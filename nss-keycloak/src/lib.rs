pub mod config;
pub mod keycloak;
mod passwd;
mod group;

use std::sync::Mutex;

use lazy_static::lazy_static;

pub use passwd::KeycloakNssPasswd;
pub use group::KeycloakNssGroup;

lazy_static! {
    // TODO: Remove pub visibility once the plugin is implemented
    pub static ref CONFIG: config::Config = config::load_config()
        .expect("Failed to load plugin configuration");

    pub static ref AUTH: Mutex<keycloak::auth::KeycloakAuth<'static>> = 
        Mutex::new(
            keycloak::auth::KeycloakAuth::new(&CONFIG.keycloak)
                .expect("Failed to initialize Keycloak authentication")
            );
}
