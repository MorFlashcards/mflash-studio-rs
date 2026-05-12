use serde::{Deserialize, Serialize};

// --- V3 STRUCT DEFINITIONS ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MFlashDeck {
    #[serde(default = "default_format")]
    pub format: String,

    #[serde(default = "default_version_v3")]
    pub version: u32,

    pub id: String,
    pub title: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_term_lang: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_def_lang: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deck_tags: Vec<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<MediaInfo>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cards: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: String,

    #[serde(default = "default_card_kind")]
    pub kind: CardKind,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_lang: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub def_lang: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Example>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<MediaInfo>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occlusion: Option<Occlusion>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lexical: Option<LexicalInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum CardKind {
    Basic,
    ImageOcclusion,
    Listening,
    MediaPrompt,
    Cloze,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    pub src: String,

    #[serde(rename = "type")]
    pub media_type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Occlusion {
    pub image: MediaInfo,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub masks: Vec<OcclusionMask>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OcclusionMask {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LexicalInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub part_of_speech: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub forms: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub antonyms: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Example {
    Text(String),
    Detailed(ExampleInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExampleInfo {
    pub text: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translation_lang: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<MediaInfo>,
}

fn default_format() -> String {
    "mflash".to_string()
}

fn default_version_v3() -> u32 {
    3
}

fn default_card_kind() -> CardKind {
    CardKind::Basic
}
