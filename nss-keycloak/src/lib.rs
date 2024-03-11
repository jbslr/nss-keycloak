pub mod config;
mod group;
pub mod keycloak;
mod passwd;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate libnss;

pub use group::KeycloakNssGroup;
pub use passwd::KeycloakNssPasswd;

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

libnss_group_hooks!(keycloak, KeycloakNssGroup);
libnss_passwd_hooks!(keycloak, KeycloakNssPasswd);
