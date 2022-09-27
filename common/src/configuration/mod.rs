use config::{Config, File};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(
        Config::builder()
        .add_source(File::with_name("common/src/configuration/config.ron"))
        .build()
        .unwrap()
    );
}
