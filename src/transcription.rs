#[derive(Clone, Copy, Default)]
pub struct TranscriptionConfig {
    /// Indicates if spaces should be transcribed as runes (true) or left as spaces (false)
    pub transcribe_spaces: bool,
    /// Determines if punctuation should be transcribed as runes (true) or remain unchanged (false)
    pub transcribe_punctuation: bool,
}

pub trait Transcriber {
    fn lookup(&self, letter: char) -> Option<char>;
    fn transcribe(&self, text: &str) -> String;
}
