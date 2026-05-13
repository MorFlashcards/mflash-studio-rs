use crate::models::{Example, MFlashDeck};
use std::collections::HashSet;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Clone)]
struct IngestItem {
    source_path: String,
    internal_path: String,
}

// --- FILE I/O ---

pub fn load_mflash(path: &str) -> Option<(MFlashDeck, String)> {
    load_mflash_result(path).ok()
}

fn load_mflash_result(path: &str) -> Result<(MFlashDeck, String), Box<dyn std::error::Error>> {
    let contents = read_deck_json(path)?;

    // Deserialize directly into v3 MFlashDeck.
    let deck: MFlashDeck = serde_json::from_str(&contents)?;

    // Strictly enforce version 3.
    if deck.version != 3 {
        return Err("Unsupported deck version. Please use a v3 deck.".into());
    }

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

    let mut ingest_queue: Vec<IngestItem> = Vec::new();
    collect_external_assets(&mut deck, &mut ingest_queue);

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
    ingest_queue: Vec<IngestItem>,
) -> Result<(), Box<dyn std::error::Error>> {
    let temp_path = format!("{}.tmp", dest_path);

    let temp_file = std::fs::File::create(&temp_path)?;
    let mut zip_writer = zip::ZipWriter::new(temp_file);

    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    let mut added_files = HashSet::new();

    // 1. Add newly ingested external files.
    for item in ingest_queue {
        if added_files.contains(&item.internal_path) {
            continue;
        }

        if let Ok(mut ext_file) = std::fs::File::open(&item.source_path) {
            zip_writer.start_file(item.internal_path.as_str(), options)?;
            std::io::copy(&mut ext_file, &mut zip_writer)?;
            added_files.insert(item.internal_path);
        }
    }

    // 2. Preserve existing package files where practical.
    if Path::new(source_path).exists() {
        if let Ok(orig_file) = std::fs::File::open(source_path) {
            if let Ok(mut archive) = zip::ZipArchive::new(orig_file) {
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    let name = file.name().to_string();

                    if name == "deck.json" {
                        continue;
                    }

                    if is_unsafe_package_path(&name) {
                        continue;
                    }

                    if added_files.contains(&name) {
                        continue;
                    }

                    zip_writer.start_file(name.as_str(), options)?;
                    std::io::copy(&mut file, &mut zip_writer)?;
                    added_files.insert(name);
                }
            }
        }
    }

    // 3. Write root deck.json.
    zip_writer.start_file("deck.json", options)?;
    zip_writer.write_all(cleaned_json_str.as_bytes())?;
    zip_writer.finish()?;

    std::fs::rename(&temp_path, dest_path)?;

    Ok(())
}

// --- MEDIA COLLECTION & INGESTION ---

fn collect_external_assets(deck: &mut MFlashDeck, ingest_queue: &mut Vec<IngestItem>) {
    let mut used_internal_paths: HashSet<String> = HashSet::new();

    if let Some(cover) = &mut deck.cover {
        ingest_media_src(
            &mut cover.src,
            ingest_queue,
            "assets/deck",
            &mut used_internal_paths,
        );
    }

    for card in &mut deck.cards {
        // Sanitize the ID to ensure it's safe for a directory name
        let safe_id = sanitize_file_name(&card.id);
        let card_asset_dir = format!("assets/cards/{}", safe_id);

        for media in &mut card.media {
            ingest_media_src(
                &mut media.src,
                ingest_queue,
                &card_asset_dir,
                &mut used_internal_paths,
            );
        }

        if let Some(occlusion) = &mut card.occlusion {
            ingest_media_src(
                &mut occlusion.image.src,
                ingest_queue,
                &card_asset_dir,
                &mut used_internal_paths,
            );
        }

        for example in &mut card.examples {
            if let Example::Detailed(info) = example {
                for media in &mut info.media {
                    ingest_media_src(
                        &mut media.src,
                        ingest_queue,
                        &card_asset_dir,
                        &mut used_internal_paths,
                    );
                }
            }
        }
    }
}

fn ingest_media_src(
    src: &mut String,
    ingest_queue: &mut Vec<IngestItem>,
    target_dir: &str,
    used_paths: &mut HashSet<String>,
) {
    let path = Path::new(src.as_str());

    // Already package-relative, remote, missing, or unsupported for ingest.
    if !path.is_absolute() || !path.exists() {
        return;
    }

    let Some(file_name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else {
        return;
    };

    let safe_name = sanitize_file_name(&file_name);
    let mut internal_path = format!("{}/{}", target_dir, safe_name);

    // Deduplicate the filename if there's a collision in this specific directory
    let mut counter = 1;
    while used_paths.contains(&internal_path) {
        let stem = Path::new(&safe_name)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let ext = Path::new(&safe_name)
            .extension()
            .unwrap_or_default()
            .to_string_lossy();
        let ext_str = if ext.is_empty() {
            String::new()
        } else {
            format!(".{}", ext)
        };

        internal_path = format!("{}/{}_{}{}", target_dir, stem, counter, ext_str);
        counter += 1;
    }

    used_paths.insert(internal_path.clone());

    ingest_queue.push(IngestItem {
        source_path: src.clone(),
        internal_path: internal_path.clone(),
    });

    *src = internal_path;
}

// --- UTILITIES & SAFETY CHECKS ---

fn sanitize_file_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn is_unsafe_package_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");

    if normalized.starts_with('/') {
        return true;
    }

    if normalized.contains("../") || normalized == ".." || normalized.starts_with("../") {
        return true;
    }

    if normalized.chars().nth(1).is_some_and(|c| c == ':') {
        return true;
    }

    false
}

fn is_raw_json_path(path: &str) -> bool {
    path.ends_with(".mflash.json") || path.ends_with(".json")
}
