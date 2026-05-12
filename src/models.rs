use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MFlashDeck {
    pub deck: DeckMetadata,
    pub cards: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckMetadata {
    pub title: String,
    pub term_language: Option<String>, // NEW: Deck-level fallback
    pub definition_language: Option<String>, // NEW: Deck-level fallback
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub term: String,
    pub definition: String,
    pub media: Option<MediaInfo>,
    pub term_language: Option<String>,
    pub definition_language: Option<String>,
    pub hyperlink: Option<String>,
    pub example_sentences: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaInfo {
    pub path: String,
}

pub fn load_mflash(path: &str) -> Option<(MFlashDeck, String)> {
    let Ok(file) = std::fs::File::open(path) else {
        return None;
    };
    let mut archive = zip::ZipArchive::new(file).ok()?;

    let mut json_file = archive.by_name("deck.json").ok()?;
    let mut contents = String::new();
    let _ = json_file.read_to_string(&mut contents);

    let deck: MFlashDeck = serde_json::from_str(&contents).ok()?;
    Some((deck, contents))
}

pub fn save_mflash(
    source_path: &str,
    dest_path: &str,
    new_json: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = format!("{}.tmp", dest_path);

    {
        // 1. Parse JSON into a dynamic Value so we don't lose unmapped fields (like tags/description)
        let mut parsed_json: serde_json::Value = serde_json::from_str(new_json)?;
        let mut ingest_queue: Vec<(String, String)> = Vec::new();

        // 2. Scan the Deck's cover image
        if let Some(deck) = parsed_json.get_mut("deck") {
            if let Some(media) = deck.get_mut("media") {
                if let Some(path_val) = media.get_mut("path") {
                    if let Some(path_str) = path_val.as_str() {
                        let path = std::path::Path::new(path_str);
                        // If it's an absolute path on the user's hard drive...
                        if path.is_absolute() && path.exists() {
                            if let Some(file_name) =
                                path.file_name().map(|n| n.to_string_lossy().to_string())
                            {
                                ingest_queue.push((path_str.to_string(), file_name.clone()));
                                // Mutate the JSON to just hold the relative filename!
                                *path_val = serde_json::Value::String(file_name);
                            }
                        }
                    }
                }
            }
        }

        // 3. Scan every Card's media image
        if let Some(cards) = parsed_json.get_mut("cards") {
            if let Some(card_array) = cards.as_array_mut() {
                for card in card_array {
                    if let Some(media) = card.get_mut("media") {
                        if let Some(path_val) = media.get_mut("path") {
                            if let Some(path_str) = path_val.as_str() {
                                let path = std::path::Path::new(path_str);
                                if path.is_absolute() && path.exists() {
                                    if let Some(file_name) =
                                        path.file_name().map(|n| n.to_string_lossy().to_string())
                                    {
                                        ingest_queue
                                            .push((path_str.to_string(), file_name.clone()));
                                        *path_val = serde_json::Value::String(file_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 4. Open archives for atomic writing
        let orig_file = std::fs::File::open(source_path)?;
        let mut archive = zip::ZipArchive::new(orig_file)?;

        let temp_file = std::fs::File::create(&temp_path)?;
        let mut zip_writer = zip::ZipWriter::new(temp_file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        let mut added_files = std::collections::HashSet::new();

        // 5. Ingest external files off the hard drive and pack them into the ZIP
        for (source, internal_name) in ingest_queue {
            if added_files.contains(&internal_name) {
                continue;
            } // Avoid duplicates
            if let Ok(mut ext_file) = std::fs::File::open(&source) {
                zip_writer.start_file(internal_name.as_str(), options)?;
                std::io::copy(&mut ext_file, &mut zip_writer)?;
                added_files.insert(internal_name);
            }
        }

        // 6. Copy over original files (skipping ones we replaced, and skipping old deck.json)
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name().to_string();

            if name == "deck.json" || added_files.contains(&name) {
                continue;
            }

            zip_writer.start_file(name.as_str(), options)?;
            std::io::copy(&mut file, &mut zip_writer)?;
        }

        // 7. Write our newly cleaned JSON string!
        zip_writer.start_file("deck.json", options)?;
        let cleaned_json_str = serde_json::to_string_pretty(&parsed_json)?;
        zip_writer.write_all(cleaned_json_str.as_bytes())?;
        zip_writer.finish()?;
    }

    std::fs::rename(&temp_path, dest_path)?;
    Ok(())
}
