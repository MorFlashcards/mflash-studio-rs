// src/audio/bcp47.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TtsLanguage {
    pub display_name: &'static str,
    pub bcp_47: &'static str,
}

// A zero-dependency, static lookup table of BCP-47 codes.
pub const SUPPORTED_LANGUAGES: &[TtsLanguage] = &[
    // English
    TtsLanguage {
        display_name: "English (US)",
        bcp_47: "en-US",
    },
    TtsLanguage {
        display_name: "English (UK)",
        bcp_47: "en-GB",
    },
    TtsLanguage {
        display_name: "English (Australia)",
        bcp_47: "en-AU",
    },
    // Spanish
    TtsLanguage {
        display_name: "Spanish (Spain)",
        bcp_47: "es-ES",
    },
    TtsLanguage {
        display_name: "Spanish (Mexico)",
        bcp_47: "es-MX",
    },
    // French
    TtsLanguage {
        display_name: "French (France)",
        bcp_47: "fr-FR",
    },
    TtsLanguage {
        display_name: "French (Canada)",
        bcp_47: "fr-CA",
    },
    // German
    TtsLanguage {
        display_name: "German (Germany)",
        bcp_47: "de-DE",
    },
    // Asian Languages
    TtsLanguage {
        display_name: "Japanese",
        bcp_47: "ja-JP",
    },
    TtsLanguage {
        display_name: "Korean",
        bcp_47: "ko-KR",
    },
    TtsLanguage {
        display_name: "Chinese (Mandarin, Simplified)",
        bcp_47: "zh-CN",
    },
    TtsLanguage {
        display_name: "Chinese (Mandarin, Traditional)",
        bcp_47: "zh-TW",
    },
    // Other Common European
    TtsLanguage {
        display_name: "Italian",
        bcp_47: "it-IT",
    },
    TtsLanguage {
        display_name: "Portuguese (Portugal)",
        bcp_47: "pt-PT",
    },
    TtsLanguage {
        display_name: "Portuguese (Brazil)",
        bcp_47: "pt-BR",
    },
    TtsLanguage {
        display_name: "Russian",
        bcp_47: "ru-RU",
    },
    TtsLanguage {
        display_name: "Dutch",
        bcp_47: "nl-NL",
    },
    TtsLanguage {
        display_name: "Polish",
        bcp_47: "pl-PL",
    },
    // Classical / Niche
    TtsLanguage {
        display_name: "Latin",
        bcp_47: "la",
    }, // Note: Many system TTS lack Latin, but some neural models support it
];

/// Helper function to aggressively match loose input strings to a strict BCP-47 code.
pub fn normalize_to_bcp47(input: &str) -> String {
    // Clean the input: remove whitespace and make it lowercase
    // e.g., " Estonian " -> "estonian", "en-US" -> "en-us"
    let clean_input = input.trim().to_lowercase();

    match clean_input.as_str() {
        // --- English ---
        "english" | "eng" | "en" | "en-us" | "en_us" => "en-US".to_string(),
        "english (uk)" | "en-gb" | "en_gb" => "en-GB".to_string(),

        // --- Baltic & Nordic ---
        "estonian" | "est" | "et" | "et-ee" => "et-EE".to_string(),
        "latvian" | "lav" | "lv" | "lv-lv" => "lv-LV".to_string(),
        "lithuanian" | "lit" | "lt" | "lt-lt" => "lt-LT".to_string(),
        "finnish" | "fin" | "fi" | "fi-fi" => "fi-FI".to_string(),
        "swedish" | "swe" | "sv" | "sv-se" => "sv-SE".to_string(),
        "icelandic" | "isl" | "is" | "is-is" => "is-IS".to_string(),

        // --- Western European ---
        "spanish" | "spa" | "es" | "es-es" => "es-ES".to_string(),
        "french" | "fra" | "fr" | "fr-fr" => "fr-FR".to_string(),
        "german" | "deu" | "de" | "de-de" => "de-DE".to_string(),
        "italian" | "ita" | "it" | "it-it" => "it-IT".to_string(),
        "dutch" | "nld" | "nl" | "nl-nl" => "nl-NL".to_string(),
        "portuguese" | "por" | "pt" | "pt-br" | "pt-pt" => "pt-PT".to_string(),

        // --- Eastern European ---
        "russian" | "rus" | "ru" | "ru-ru" => "ru-RU".to_string(),
        "polish" | "pol" | "pl" | "pl-pl" => "pl-PL".to_string(),
        "ukrainian" | "ukr" | "uk" | "uk-ua" => "uk-UA".to_string(),

        // --- Asian ---
        "japanese" | "jpn" | "ja" | "ja-jp" => "ja-JP".to_string(),
        "korean" | "kor" | "ko" | "ko-kr" => "ko-KR".to_string(),
        "chinese" | "mandarin" | "zho" | "zh" | "zh-cn" => "zh-CN".to_string(),

        // --- Classical ---
        "latin" | "lat" | "la" => "la".to_string(),
        "greek" | "ell" | "el" | "el-gr" => "el-GR".to_string(),
        "hebrew" | "heb" | "he" | "he-il" => "he-IL".to_string(),

        // --- Fallback ---
        // If empty, return English. Otherwise, pass their custom tag through!
        "" => "en-US".to_string(),
        _ => input.trim().to_string(),
    }
}
