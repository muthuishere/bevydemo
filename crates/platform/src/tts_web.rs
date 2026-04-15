// OCP: Web TTS backend - browser speech synthesis
use core_domain::tts::TtsService;

pub struct WebTts {
    pub enabled: bool,
}

impl TtsService for WebTts {
    fn speak(&self, text: &str, language: &str) {
        if !self.enabled {
            return;
        }
        // In a real wasm build, call web_sys::SpeechSynthesisUtterance
        log::info!("[WebTTS] speak: lang={} text={}", language, text);
    }

    fn stop(&self) {
        log::info!("[WebTTS] stop");
    }

    fn is_available(&self) -> bool {
        self.enabled
    }
}
