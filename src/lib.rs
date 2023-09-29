#[macro_use]
extern crate log;

use anyhow::Error;
use onetagger_tagger::{
    create_plugin, supported_tags, AudioFileInfo, AutotaggerSource, AutotaggerSourceBuilder,
    ConfigCallbackResponse, PlatformInfo, TaggerConfig, Track, TrackMatch,
};
use serde_json::Value;

/// Is used to get metadata about plugin and to create instances of `AutotaggerSource`
struct PlatformBuilder {}

impl AutotaggerSourceBuilder for PlatformBuilder {
    fn new() -> Self {
        Self {}
    }

    /// Create new instance of `AutotaggerSource`, will be called n-threads times
    fn get_source(&mut self, _config: &TaggerConfig) -> Result<Box<dyn AutotaggerSource>, Error> {
        Ok(Box::new(Platform {}))
    }

    /// Info shown in UI
    fn info(&self) -> PlatformInfo {
        PlatformInfo {
            // Should be unique
            id: "example".to_string(),
            name: "Example".to_string(),
            description: "Example plugin".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            // Use png with transparent background
            icon: include_bytes!("../icon.png"),
            // 0 - for user defined number threads
            max_threads: 1,
            custom_options: Default::default(),
            requires_auth: false,
            supported_tags: supported_tags!(Title, Artist),
        }
    }

    /// (Optional)
    fn config_callback(&mut self, _name: &str, _config: Value) -> ConfigCallbackResponse {
        ConfigCallbackResponse::Empty
    }
}

struct Platform {}

impl AutotaggerSource for Platform {
    /// This function should return matched tracks with least info possible
    fn match_track(
        &mut self,
        _info: &AudioFileInfo,
        _config: &TaggerConfig,
    ) -> Result<Vec<TrackMatch>, Error> {
        // Check docs for onetagger_tagger::MatchingUtils for useful funct

        info!("You can also use log here!");

        Ok(vec![])
    }

    /// This function gets called on the matched track to fill in any remaining info
    fn extend_track(&mut self, _track: &mut Track, _config: &TaggerConfig) -> Result<(), Error> {
        Ok(())
    }
}

// Required to make this work as a plugin
create_plugin!(PlatformBuilder, Platform);
