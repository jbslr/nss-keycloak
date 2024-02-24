mod config;

use lazy_static::lazy_static;

lazy_static! {
    // TODO: Remove pub visibility once the plugin is implemented
    pub static ref CONFIG: config::Config = config::load_config()
        .expect("Failed to load plugin configuration");
}
