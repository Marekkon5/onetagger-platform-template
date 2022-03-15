use onetagger_tagger::{
    create_plugin, AudioFileInfo, AutotaggerSource, AutotaggerSourceBuilder, PlatformInfo,
    TaggerConfig, Track,
};
use std::error::Error;

/// Is used to get metadata about plugin and to create instances of `AutotaggerSource`
struct PlatformBuilder {}

impl AutotaggerSourceBuilder for PlatformBuilder {
    fn new() -> Self {
        Self {}
    }

    /// Create new instance of `AutotaggerSource`, will be called n-threads times
    fn get_source(
        &mut self,
        _config: &TaggerConfig,
    ) -> Result<Box<dyn AutotaggerSource>, Box<dyn Error>> {
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
        }
    }
}

struct Platform {}

impl AutotaggerSource for Platform {
    /// Returns Err if an error happened, `Ok(None)` if no matches, Ok(Some(accuracy from 0.0 to 1.0, track)) for matched track
    fn match_track(
        &mut self,
        _info: &AudioFileInfo,
        _config: &TaggerConfig,
    ) -> Result<Option<(f64, Track)>, Box<dyn Error>> {
        // Check onetagger_tagger::MatchingUtils for track matching functions
        Ok(None)
    }
}

// Required to make this work as a plugin
create_plugin!(PlatformBuilder, Platform);
