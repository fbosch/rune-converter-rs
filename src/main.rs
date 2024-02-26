use rune_converter::runes::ElderFuthark;
use rune_converter::transcription::TranscriptionConfig;

fn main() {
    let futhark = ElderFuthark::new(TranscriptionConfig::default());

    let converted = futhark.transcribe("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");

    println!("converted: {}", converted);
}
