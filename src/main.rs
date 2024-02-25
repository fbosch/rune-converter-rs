#[non_exhaustive]
struct ElderFuthark;

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
    pub const BLANK: char = '᛫';
    pub fn lookup(rune: char, convert_spaces: bool) -> Option<char> {
        match rune {
            'f' => Some(Self::FEHU),
            'u' => Some(Self::URUZ),
            'þ' => Some(Self::THURISAZ),
            'a' => Some(Self::ANSUZ),
            'r' => Some(Self::RAIDHO),
            'k' => Some(Self::KAUNAN),
            'g' => Some(Self::GEBO),
            'w' => Some(Self::WUNJO),
            'x' => Some(Self::THURISAZ),
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
            'e' => Some(Self::EHWAZ),
            'm' => Some(Self::MANNAZ),
            'l' => Some(Self::LAGUZ),
            'ŋ' => Some(Self::INGWAZ),
            'd' => Some(Self::DAGAZ),
            'o' => Some(Self::OTHALA),
            'q' => Some(Self::KAUNAN),
            ' ' => {
                if convert_spaces {
                    Some(Self::BLANK)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub fn latin_to_elder_futhark(input: &str) -> String {
    input
        .to_ascii_lowercase()
        .chars()
        .map(|c| ElderFuthark::lookup(c, false).unwrap_or(c))
        .collect()
}

fn main() {
    let hello_world = String::from("HELLO WORLD!");
    let converted = latin_to_elder_futhark(hello_world.as_str());

    println!("converted: {}", converted);
}
