#[derive(Clone, Copy)]
pub struct TranscriptionConfig {
    /// Indicates if spaces should be transcribed as runes (true) or left as spaces (false)
    pub transcribe_spaces: bool,
    ///  Determines if punctuation should be transcribed as runes (true) or remain unchanged (false)
    pub transcribe_punctuation: bool,
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        TranscriptionConfig {
            transcribe_spaces: true,
            transcribe_punctuation: true,
        }
    }
}

pub trait Transcriber {
    fn lookup(rune: char, config: TranscriptionConfig) -> Option<char>;
    fn transcribe(&self, text: &str) -> String;
}
