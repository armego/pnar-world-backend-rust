use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Pnar alphabet character mapping
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PnarCharacter {
    #[schema(example = "æ")]
    pub small: &'static str,
    #[schema(example = "Æ")]
    pub capital: &'static str,
    #[schema(example = "se")]
    pub kbf_small: &'static str,
    #[schema(example = "Ae")]
    pub kbf_capital: &'static str,
    #[schema(example = 6)]
    pub sort_order: u8,
}

/// Fixed Pnar alphabet character mappings
pub const PNAR_ALPHABET: [PnarCharacter; 25] = [
    PnarCharacter { small: "a", capital: "A", kbf_small: "a", kbf_capital: "A", sort_order: 1 },
    PnarCharacter { small: "b", capital: "B", kbf_small: "b", kbf_capital: "B", sort_order: 2 },
    PnarCharacter { small: "c", capital: "C", kbf_small: "c", kbf_capital: "C", sort_order: 3 },
    PnarCharacter { small: "d", capital: "D", kbf_small: "d", kbf_capital: "D", sort_order: 4 },
    PnarCharacter { small: "e", capital: "E", kbf_small: "e", kbf_capital: "E", sort_order: 5 },
    PnarCharacter { small: "æ", capital: "Æ", kbf_small: "se", kbf_capital: "Ae", sort_order: 6 },
    PnarCharacter { small: "f", capital: "F", kbf_small: "f", kbf_capital: "F", sort_order: 7 },
    PnarCharacter { small: "g", capital: "G", kbf_small: "g", kbf_capital: "G", sort_order: 8 },
    PnarCharacter { small: "h", capital: "H", kbf_small: "h", kbf_capital: "H", sort_order: 9 },
    PnarCharacter { small: "i", capital: "I", kbf_small: "i", kbf_capital: "I", sort_order: 10 },
    PnarCharacter { small: "ï", capital: "Ï", kbf_small: "ii", kbf_capital: "Ii", sort_order: 11 },
    PnarCharacter { small: "j", capital: "J", kbf_small: "j", kbf_capital: "J", sort_order: 12 },
    PnarCharacter { small: "k", capital: "K", kbf_small: "k", kbf_capital: "K", sort_order: 13 },
    PnarCharacter { small: "l", capital: "L", kbf_small: "l", kbf_capital: "L", sort_order: 14 },
    PnarCharacter { small: "m", capital: "M", kbf_small: "m", kbf_capital: "M", sort_order: 15 },
    PnarCharacter { small: "n", capital: "N", kbf_small: "n", kbf_capital: "N", sort_order: 16 },
    PnarCharacter { small: "ñ", capital: "Ñ", kbf_small: "gn", kbf_capital: "Ng", sort_order: 17 },
    PnarCharacter { small: "o", capital: "O", kbf_small: "o", kbf_capital: "O", sort_order: 18 },
    PnarCharacter { small: "p", capital: "P", kbf_small: "p", kbf_capital: "P", sort_order: 19 },
    PnarCharacter { small: "r", capital: "R", kbf_small: "r", kbf_capital: "R", sort_order: 20 },
    PnarCharacter { small: "s", capital: "S", kbf_small: "s", kbf_capital: "S", sort_order: 21 },
    PnarCharacter { small: "t", capital: "T", kbf_small: "t", kbf_capital: "T", sort_order: 22 },
    PnarCharacter { small: "u", capital: "U", kbf_small: "u", kbf_capital: "U", sort_order: 23 },
    PnarCharacter { small: "w", capital: "W", kbf_small: "w", kbf_capital: "W", sort_order: 24 },
    PnarCharacter { small: "y", capital: "Y", kbf_small: "y", kbf_capital: "Y", sort_order: 25 },
];

/// Convert keyboard-friendly text to Pnar characters
pub fn convert_kbf_to_pnar(text: &str) -> String {
    let mut result = text.to_string();
    
    // Sort by length descending to match longer patterns first
    let mut patterns: Vec<_> = PNAR_ALPHABET.iter()
        .flat_map(|ch| vec![
            (ch.kbf_small, ch.small),
            (ch.kbf_capital, ch.capital),
        ])
        .collect();
    patterns.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    
    for (kbf, pnar) in patterns {
        result = result.replace(kbf, pnar);
    }
    
    result
}

/// Convert Pnar characters to keyboard-friendly text
pub fn convert_pnar_to_kbf(text: &str) -> String {
    let mut result = text.to_string();
    
    for ch in &PNAR_ALPHABET {
        result = result.replace(ch.small, ch.kbf_small);
        result = result.replace(ch.capital, ch.kbf_capital);
    }
    
    result
}
