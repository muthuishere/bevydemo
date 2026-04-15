// OCP: Fallback TTS - plays pre-recorded audio files
use core_domain::tts::TtsService;

pub struct FallbackTts {
    pub enabled: bool,
}

impl TtsService for FallbackTts {
    fn speak(&self, text: &str, language: &str) {
        if !self.enabled {
            return;
        }
        // Would trigger bevy_kira_audio asset playback
        log::info!(
            "[FallbackTTS] would play audio for lang={} text={}",
            language,
            text
        );
    }

    fn stop(&self) {}

    fn is_available(&self) -> bool {
        self.enabled
    }
}
