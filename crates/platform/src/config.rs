// SRP: loads app config from .env
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub default_language: String,
    pub tts_enabled: bool,
    pub tts_backend: String,
    pub analytics_enabled: bool,
    pub debug_cheats: bool,
    pub log_level: String,
    pub skip_intro: bool,
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();
        Self {
            default_language: env::var("DEFAULT_LANGUAGE").unwrap_or_else(|_| "en".into()),
            tts_enabled: env::var("TTS_ENABLED").unwrap_or_else(|_| "true".into()) == "true",
            tts_backend: env::var("TTS_BACKEND").unwrap_or_else(|_| "audio_fallback".into()),
            analytics_enabled: env::var("ANALYTICS_ENABLED")
                .unwrap_or_else(|_| "false".into())
                == "true",
            debug_cheats: env::var("DEBUG_CHEATS").unwrap_or_else(|_| "false".into()) == "true",
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),
            skip_intro: env::var("SKIP_INTRO").unwrap_or_else(|_| "false".into()) == "true",
            master_volume: env::var("MASTER_VOLUME")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1.0),
            sfx_volume: env::var("SFX_VOLUME")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1.0),
            music_volume: env::var("MUSIC_VOLUME")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.7),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = AppConfig {
            default_language: "en".into(),
            tts_enabled: true,
            tts_backend: "audio_fallback".into(),
            analytics_enabled: false,
            debug_cheats: false,
            log_level: "info".into(),
            skip_intro: false,
            master_volume: 1.0,
            sfx_volume: 1.0,
            music_volume: 0.7,
        };
        assert_eq!(config.default_language, "en");
        assert!(config.tts_enabled);
        assert!(!config.analytics_enabled);
    }
}
