use rune_converter::runes::ElderFuthark;
use rune_converter::transcription::{Transcriber, TranscriptionConfig};

fn main() {
    let futhark = ElderFuthark::new(TranscriptionConfig {
        transcribe_spaces: true,
        transcribe_punctuation: true,
    });

    let lorem = futhark.transcribe("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    println!("{}", lorem);
}
