use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MFlashDeck {
    #[serde(default = "default_format")]
    pub format: String,

    #[serde(default = "default_version")]
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
    pub cover_media: Option<String>,

    pub cards: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub id: String,
    pub term: String,
    pub definition: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_lang: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub def_lang: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phonetic: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub part_of_speech: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyperlink: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub media: Vec<MediaInfo>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Example>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaInfo {
    pub src: String,

    #[serde(rename = "type")]
    pub media_type: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
}

fn default_format() -> String {
    "mflash".to_string()
}

fn default_version() -> u32 {
    2
}

pub fn load_mflash(path: &str) -> Option<(MFlashDeck, String)> {
    load_mflash_result(path).ok()
}

fn load_mflash_result(path: &str) -> Result<(MFlashDeck, String), Box<dyn std::error::Error>> {
    let contents = read_deck_json(path)?;
    let deck: MFlashDeck = serde_json::from_str(&contents)?;
    let normalized_json = serde_json::to_string_pretty(&deck)?;
    Ok((deck, normalized_json))
}

fn read_deck_json(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if is_raw_json_path(path) {
        return Ok(std::fs::read_to_string(path)?);
    }

    let file = std::fs::File::open(path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let mut json_file = archive.by_name("deck.json")?;
    let mut contents = String::new();
    json_file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn save_mflash(
    source_path: &str,
    dest_path: &str,
    new_json: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut deck: MFlashDeck = serde_json::from_str(new_json)?;

    let mut ingest_queue: Vec<(String, String)> = Vec::new();
    collect_external_media(&mut deck, &mut ingest_queue);

    let cleaned_json_str = serde_json::to_string_pretty(&deck)?;

    if is_raw_json_path(dest_path) || is_raw_json_path(source_path) {
        std::fs::write(dest_path, cleaned_json_str)?;
        return Ok(());
    }

    save_packaged_mflash(source_path, dest_path, &cleaned_json_str, ingest_queue)
}

fn save_packaged_mflash(
    source_path: &str,
    dest_path: &str,
    cleaned_json_str: &str,
    ingest_queue: Vec<(String, String)>,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = format!("{}.tmp", dest_path);

    let temp_file = std::fs::File::create(&temp_path)?;
    let mut zip_writer = zip::ZipWriter::new(temp_file);

    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut added_files = HashSet::new();

    for (source, internal_name) in ingest_queue {
        if added_files.contains(&internal_name) {
            continue;
        }

        if let Ok(mut ext_file) = std::fs::File::open(&source) {
            zip_writer.start_file(internal_name.as_str(), options)?;
            std::io::copy(&mut ext_file, &mut zip_writer)?;
            added_files.insert(internal_name);
        }
    }

    if Path::new(source_path).exists() {
        if let Ok(orig_file) = std::fs::File::open(source_path) {
            if let Ok(mut archive) = zip::ZipArchive::new(orig_file) {
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let name = file.name().to_string();

                    if name == "deck.json" || added_files.contains(&name) {
                        continue;
                    }

                    zip_writer.start_file(name.as_str(), options)?;
                    std::io::copy(&mut file, &mut zip_writer)?;
                }
            }
        }
    }

    zip_writer.start_file("deck.json", options)?;
    zip_writer.write_all(cleaned_json_str.as_bytes())?;
    zip_writer.finish()?;

    std::fs::rename(&temp_path, dest_path)?;

    Ok(())
}

fn collect_external_media(deck: &mut MFlashDeck, ingest_queue: &mut Vec<(String, String)>) {
    if let Some(cover_media) = &mut deck.cover_media {
        ingest_path_value(cover_media, ingest_queue, true);
    }

    for card in &mut deck.cards {
        for media in &mut card.media {
            ingest_path_value(&mut media.src, ingest_queue, false);
        }
    }
}

fn ingest_path_value(
    path_value: &mut String,
    ingest_queue: &mut Vec<(String, String)>,
    prefer_root: bool,
) {
    let path = Path::new(path_value.as_str());

    if !path.is_absolute() || !path.exists() {
        return;
    }

    let Some(file_name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else {
        return;
    };

    let internal_name = if prefer_root {
        file_name
    } else {
        format!("media/{}", file_name)
    };

    ingest_queue.push((path_value.clone(), internal_name.clone()));
    *path_value = internal_name;
}

fn is_raw_json_path(path: &str) -> bool {
    path.ends_with(".mflash.json") || path.ends_with(".json")
}
