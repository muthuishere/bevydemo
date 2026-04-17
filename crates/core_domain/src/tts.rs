// OCP: TTS abstraction - add new backends without touching existing code
pub trait TtsService: Send + Sync {
    fn speak(&self, text: &str, language: &str);
    fn stop(&self);
    fn is_available(&self) -> bool;
}

pub struct NoopTts;

impl TtsService for NoopTts {
    fn speak(&self, _text: &str, _language: &str) {}
    fn stop(&self) {}
    fn is_available(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_tts_not_available() {
        let tts = NoopTts;
        assert!(!tts.is_available());
    }
}
