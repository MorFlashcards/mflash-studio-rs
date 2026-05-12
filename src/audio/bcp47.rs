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
pub fn normalize_to_bcp47(input: &str) -> &'static str {
    // Clean the input: remove whitespace and make it lowercase
    // e.g., " Estonian " -> "estonian", "en-US" -> "en-us"
    let clean_input = input.trim().to_lowercase();

    match clean_input.as_str() {
        // --- English ---
        "english" | "eng" | "en" | "en-us" | "en_us" => "en-US",
        "english (uk)" | "en-gb" | "en_gb" => "en-GB",

        // --- Baltic & Nordic ---
        "estonian" | "est" | "et" | "et-ee" => "et-EE",
        "latvian" | "lav" | "lv" | "lv-lv" => "lv-LV",
        "lithuanian" | "lit" | "lt" | "lt-lt" => "lt-LT",
        "finnish" | "fin" | "fi" | "fi-fi" => "fi-FI",
        "swedish" | "swe" | "sv" | "sv-se" => "sv-SE",
        "icelandic" | "isl" | "is" | "is-is" => "is-IS",

        // --- Western European ---
        "spanish" | "spa" | "es" | "es-es" => "es-ES",
        "french" | "fra" | "fr" | "fr-fr" => "fr-FR",
        "german" | "deu" | "de" | "de-de" => "de-DE",
        "italian" | "ita" | "it" | "it-it" => "it-IT",
        "dutch" | "nld" | "nl" | "nl-nl" => "nl-NL",
        "portuguese" | "por" | "pt" | "pt-br" | "pt-pt" => "pt-PT",

        // --- Eastern European ---
        "russian" | "rus" | "ru" | "ru-ru" => "ru-RU",
        "polish" | "pol" | "pl" | "pl-pl" => "pl-PL",
        "ukrainian" | "ukr" | "uk" | "uk-ua" => "uk-UA",

        // --- Asian ---
        "japanese" | "jpn" | "ja" | "ja-jp" => "ja-JP",
        "korean" | "kor" | "ko" | "ko-kr" => "ko-KR",
        "chinese" | "mandarin" | "zho" | "zh" | "zh-cn" => "zh-CN",

        // --- Classical ---
        "latin" | "lat" | "la" => "la",
        "greek" | "ell" | "el" | "el-gr" => "el-GR",
        "hebrew" | "heb" | "he" | "he-il" => "he-IL",

        // --- Fallback ---
        // If they typed something we don't recognize, or left it blank,
        // default to en-US so the audio engine doesn't panic.
        _ => "en-US",
    }
}
