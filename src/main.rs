use rune_converter::runes::ElderFuthark;
use rune_converter::transcription::{Transcriber, TranscriptionConfig};

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    text: String,
    #[arg(short, long, value_parser = clap::value_parser!(bool), default_value = "true")]
    spaces: bool,
    #[arg(short, long, value_parser = clap::value_parser!(bool), default_value = "true")]
    punctuation: bool,
}

fn main() {
    let args = Cli::parse();

    let config = TranscriptionConfig {
        transcribe_spaces: args.spaces,
        transcribe_punctuation: args.punctuation,
    };

    let elder_futhark = ElderFuthark::new(config);

    let transcribed = elder_futhark.transcribe(&args.text);

    println!("{}", transcribed);
}
