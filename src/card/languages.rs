use serde::{Deserialize, Serialize};


/// Enum defining the languages a card can be printed in.
#[derive(Default, Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[cfg_attr(test, serde(deny_unknown_fields))]
pub enum Languages {
    #[default]
    #[serde(rename = "en")]
    English,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "zhs")]
    SimplifiedChinese,
    #[serde(rename = "zht")]
    TraditionalChinese,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "la")]
    Latin,
    #[serde(rename = "grc")]
    AncientGreek,
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "sa")]
    Sanskrit,
    #[serde(rename = "ph")]
    Phyrexian,
    #[serde(rename = "qya")]
    Quenya,
}

impl std::fmt::Display for Languages {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Languages::*;
        write!(
            f,
            "{}",
            match self {
                English => "en",
                Spanish => "es",
                French => "fr",
                German => "de",
                Italian => "it",
                Portuguese => "pt",
                Japanese => "ja",
                Korean => "ko",
                Russian => "ru",
                SimplifiedChinese => "zhs",
                TraditionalChinese => "zht",
                Hebrew => "he",
                Latin => "la",
                AncientGreek => "grc",
                Arabic => "ar",
                Sanskrit => "sa",
                Phyrexian => "ph",
                Quenya => "qya",
            }
        )
    }
}


// impl Languages {
//     /// Returns the language code as a string slice.
//     pub fn as_str(&self) -> &str {
//         use Languages::*;
//         match self {
//             English => "en",
//             Spanish => "es",
//             French => "fr",
//             German => "de",
//             Italian => "it",
//             Portuguese => "pt",
//             Japanese => "ja",
//             Korean => "ko",
//             Russian => "ru",
//             SimplifiedChinese => "zhs",
//             TraditionalChinese => "zht",
//             Hebrew => "he",
//             Latin => "la",
//             AncientGreek => "grc",
//             Arabic => "ar",
//             Sanskrit => "sa",
//             Phyrexian => "ph",
//             Quenya => "qya",
//         }
//     }

//     /// Returns the language name as a string slice.
//     pub fn as_long_str(&self) -> &str {
//         use Languages::*;
//         match self {
//             English => "English",
//             Spanish => "Spanish",
//             French => "French",
//             German => "German",
//             Italian => "Italian",
//             Portuguese => "Portuguese",
//             Japanese => "Japanese",
//             Korean => "Korean",
//             Russian => "Russian",
//             SimplifiedChinese => "Simplified Chinese",
//             TraditionalChinese => "Traditional Chinese",
//             Hebrew => "Hebrew",
//             Latin => "Latin",
//             AncientGreek => "Ancient Greek",
//             Arabic => "Arabic",
//             Sanskrit => "Sanskrit",
//             Phyrexian => "Phyrexian",
//             Quenya => "Quenya",
//         }
//     }
// }