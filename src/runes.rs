use crate::transcription::Transcriber;
use crate::transcription::TranscriptionConfig;

#[non_exhaustive]
pub struct ElderFuthark {
    config: TranscriptionConfig,
}

impl ElderFuthark {
    pub fn new(config: TranscriptionConfig) -> Self {
        Self { config }
    }
}

impl Default for ElderFuthark {
    fn default() -> Self {
        Self {
            config: TranscriptionConfig::default(),
        }
    }
}

impl ElderFuthark {
    pub const FEHU: char = 'ᚠ';
    pub const URUZ: char = 'ᚢ';
    pub const THURISAZ: char = 'ᚦ';
    pub const ANSUZ: char = 'ᚨ';
    pub const RAIDHO: char = 'ᚱ';
    pub const KAUNAN: char = 'ᚲ';
    pub const GEBO: char = 'ᚷ';
    pub const WUNJO: char = 'ᚹ';
    pub const HAGALAZ: char = 'ᚺ';
    pub const NAUDIZ: char = 'ᚾ';
    pub const ISA: char = 'ᛁ';
    pub const JERA: char = 'ᛃ';
    pub const EIHWAZ: char = 'ᛇ';
    pub const PERTHO: char = 'ᛈ';
    pub const ALGIZ: char = 'ᛉ';
    pub const SOWILO: char = 'ᛊ';
    pub const TIWAZ: char = 'ᛏ';
    pub const BERKANAN: char = 'ᛒ';
    pub const EHWAZ: char = 'ᛖ';
    pub const MANNAZ: char = 'ᛗ';
    pub const LAGUZ: char = 'ᛚ';
    pub const INGWAZ: char = 'ᛜ';
    pub const DAGAZ: char = 'ᛞ';
    pub const OTHALA: char = 'ᛟ';
    pub const SPACE: char = '᛫';
    pub const CROSS: char = '᛭';
}

impl Transcriber for ElderFuthark {
    fn lookup(&self, character: char) -> Option<char> {
        match character {
            'f' => Some(Self::FEHU),
            'u' => Some(Self::URUZ),
            'þ' => Some(Self::THURISAZ),
            'a' | 'æ' => Some(Self::ANSUZ),
            'r' => Some(Self::RAIDHO),
            'k' | 'c' => Some(Self::KAUNAN),
            'g' => Some(Self::GEBO),
            'w' | 'v' => Some(Self::WUNJO),
            'h' => Some(Self::HAGALAZ),
            'n' => Some(Self::NAUDIZ),
            'i' => Some(Self::ISA),
            'j' => Some(Self::JERA),
            'y' => Some(Self::EIHWAZ),
            'p' => Some(Self::PERTHO),
            'z' => Some(Self::ALGIZ),
            's' => Some(Self::SOWILO),
            't' => Some(Self::TIWAZ),
            'b' => Some(Self::BERKANAN),
            'e' | 'ø' => Some(Self::EHWAZ),
            'm' => Some(Self::MANNAZ),
            'l' => Some(Self::LAGUZ),
            'd' => Some(Self::DAGAZ),
            'o' | 'å' => Some(Self::OTHALA),
            'q' => Some(Self::KAUNAN),
            '\'' | ',' if self.config.transcribe_punctuation => Some('\0'),
            '.' if self.config.transcribe_punctuation => Some(Self::CROSS),
            ' ' if self.config.transcribe_spaces => Some(Self::SPACE),
            _ => None,
        }
    }
    fn transcribe(&self, text: &str) -> String {
        let mut result = String::new();
        let lowercase_text = text.to_lowercase();
        let mut chars = lowercase_text.chars().peekable();

        while let Some(current_char) = chars.next() {
            let next_char = chars.peek().copied(); // Store the peeked value
            match (current_char, next_char) {
                ('x', _) => {
                    result.push(Self::KAUNAN);
                    result.push(Self::SOWILO);
                }
                ('t', Some('h')) => {
                    result.push(Self::THURISAZ);
                    chars.next(); // Consume 'h'
                }
                ('n', Some('g')) => {
                    result.push(Self::INGWAZ);
                    chars.next(); // Consume 'g'
                }
                ('e', Some('a')) => {
                    result.push(Self::EHWAZ);
                    chars.next(); // Consume 'a'
                }
                (c, _) => {
                    if let Some(rune) = Self::lookup(self, c) {
                        result.push(rune);
                    } else {
                        // If the character is not found, just push it as is
                        result.push(c);
                    }
                }
            }
        }

        result
    }
}
