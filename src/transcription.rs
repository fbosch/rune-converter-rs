#[derive(Clone, Copy)]
pub struct TranscriptionConfig {
    pub convert_spaces: bool,
    pub convert_punctuation: bool,
}

impl Default for TranscriptionConfig {
    fn default() -> Self {
        TranscriptionConfig {
            convert_spaces: true,
            convert_punctuation: true,
        }
    }
}
