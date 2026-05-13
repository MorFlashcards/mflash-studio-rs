use crate::{media, MFlashStudioApp, Workspace};
use eframe::egui;
use std::collections::BTreeMap;
use std::path::Path;

const TILE_WIDTH: f32 = 180.0;
const TILE_HEIGHT: f32 = 178.0;
const TILE_GAP_X: f32 = 8.0;
const TILE_GAP_Y: f32 = 8.0;
const THUMB_SIZE: f32 = 82.0;

const ASSET_FILTER_MEMORY_KEY: &str = "assets_workspace_kind_filter";

#[derive(Clone)]
struct AssetRow {
    card_index: usize,
    media_index: usize,
    src: String,
    kind: AssetKind,
    card_term: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum AssetKind {
    Image,
    Gif,
    Svg,
    Audio,
    Video,
    Font,
    Data,
    File,
    Other(String),
}

impl AssetKind {
    fn label(&self) -> String {
        match self {
            AssetKind::Image => "Image".to_string(),
            AssetKind::Gif => "GIF".to_string(),
            AssetKind::Svg => "SVG".to_string(),
            AssetKind::Audio => "Audio".to_string(),
            AssetKind::Video => "Video".to_string(),
            AssetKind::Font => "Font".to_string(),
            AssetKind::Data => "Data".to_string(),
            AssetKind::File => "File".to_string(),
            AssetKind::Other(ext) => ext.to_uppercase(),
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            AssetKind::Image => "🖼",
            AssetKind::Gif => "🎞",
            AssetKind::Svg => "◇",
            AssetKind::Audio => "♪",
            AssetKind::Video => "▶",
            AssetKind::Font => "Aa",
            AssetKind::Data => "{}",
            AssetKind::File => "□",
            AssetKind::Other(_) => "?",
        }
    }

    fn can_thumbnail(&self) -> bool {
        matches!(self, AssetKind::Image | AssetKind::Gif)
    }

    fn filter_key(&self) -> &'static str {
        match self {
            AssetKind::Image => "images",
            AssetKind::Gif => "gifs",
            AssetKind::Svg => "svgs",
            AssetKind::Audio => "audio",
            AssetKind::Video => "video",
            AssetKind::Font => "fonts",
            AssetKind::Data => "data",
            AssetKind::File => "files",
            AssetKind::Other(_) => "other",
        }
    }
}

pub fn render(app: &mut MFlashStudioApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    ui.heading("Assets");
    ui.label("Browse attached images, audio, video, GIFs, SVGs, and other deck files.");

    ui.add_space(8.0);

    if app.deck.is_none() {
        ui.centered_and_justified(|ui| {
            ui.label("No deck loaded.");
        });
        return;
    }

    let mut assets = collect_assets(app);
    let total_asset_count = assets.len();

    render_viewbar(ui);

    ui.add_space(6.0);

    ui.horizontal(|ui| {
        ui.label("🔍 Search:");
        ui.add_sized(
            [320.0, 22.0],
            egui::TextEdit::singleline(&mut app.search_query),
        );

        ui.separator();

        ui.label(
            egui::RichText::new(format!(
                "{} asset reference{}",
                total_asset_count,
                if total_asset_count == 1 { "" } else { "s" }
            ))
            .weak(),
        );
    });

    ui.separator();

    if assets.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label("No media assets are attached to this deck yet.");
        });
        return;
    }

    let active_filter = current_filter(ui);
    assets.retain(|asset| asset_matches_filter(asset, &active_filter));

    let search = app.search_query.trim().to_lowercase();

    if !search.is_empty() {
        assets.retain(|asset| {
            asset.src.to_lowercase().contains(&search)
                || asset.kind.label().to_lowercase().contains(&search)
                || asset.card_term.to_lowercase().contains(&search)
        });
    }

    if assets.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label("No matching assets.");
        });
        return;
    }

    render_summary(ui, &assets);

    ui.add_space(6.0);

    let mut jump_to_card: Option<usize> = None;
    let mut delete_asset: Option<(usize, usize)> = None;

    let available_width = ui.available_width().max(TILE_WIDTH);
    let columns = ((available_width + TILE_GAP_X) / (TILE_WIDTH + TILE_GAP_X))
        .floor()
        .max(1.0) as usize;

    let row_count = assets.len().div_ceil(columns);
    let row_height = TILE_HEIGHT + TILE_GAP_Y;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show_rows(ui, row_height, row_count, |ui, row_range| {
            for row in row_range {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = TILE_GAP_X;

                    for col in 0..columns {
                        let asset_index = row * columns + col;

                        let Some(asset) = assets.get(asset_index) else {
                            break;
                        };

                        render_asset_tile(
                            app,
                            ui,
                            ctx,
                            asset,
                            &mut jump_to_card,
                            &mut delete_asset,
                        );
                    }
                });

                ui.add_space(TILE_GAP_Y);
            }
        });

    if let Some((card_index, media_index)) = delete_asset {
        remove_asset_reference(app, card_index, media_index, ctx);
    }

    if let Some(card_index) = jump_to_card {
        app.set_index(card_index, ctx);
        app.workspace = Workspace::VisualEditor;
    }
}

fn render_viewbar(ui: &mut egui::Ui) {
    let mut filter = current_filter(ui);

    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;

        ui.label(egui::RichText::new("👁 View:").strong());

        filter_button(ui, &mut filter, "all", "All");
        filter_button(ui, &mut filter, "images", "Images");
        filter_button(ui, &mut filter, "gifs", "GIFs");
        filter_button(ui, &mut filter, "svgs", "SVGs");
        filter_button(ui, &mut filter, "audio", "Audio");
        filter_button(ui, &mut filter, "video", "Video");
        filter_button(ui, &mut filter, "fonts", "Fonts");
        filter_button(ui, &mut filter, "data", "Data");
        filter_button(ui, &mut filter, "files", "Files");
        filter_button(ui, &mut filter, "other", "Other");
    });

    set_filter(ui, filter);
}

fn filter_button(ui: &mut egui::Ui, filter: &mut String, key: &str, label: &str) {
    if ui.selectable_label(filter == key, label).clicked() {
        *filter = key.to_string();
    }
}

fn current_filter(ui: &egui::Ui) -> String {
    let id = egui::Id::new(ASSET_FILTER_MEMORY_KEY);

    ui.ctx().data_mut(|data| {
        data.get_persisted::<String>(id)
            .unwrap_or_else(|| "all".to_string())
    })
}

fn set_filter(ui: &egui::Ui, filter: String) {
    let id = egui::Id::new(ASSET_FILTER_MEMORY_KEY);

    ui.ctx().data_mut(|data| {
        data.insert_persisted(id, filter);
    });
}

fn asset_matches_filter(asset: &AssetRow, filter: &str) -> bool {
    filter == "all" || asset.kind.filter_key() == filter
}

fn render_summary(ui: &mut egui::Ui, assets: &[AssetRow]) {
    let summary = summarize_assets(assets);

    ui.horizontal_wrapped(|ui| {
        for (kind, count) in summary {
            ui.label(
                egui::RichText::new(format!("{}: {}", kind, count))
                    .small()
                    .weak(),
            );
        }
    });
}

fn render_asset_tile(
    app: &MFlashStudioApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    asset: &AssetRow,
    jump_to_card: &mut Option<usize>,
    delete_asset: &mut Option<(usize, usize)>,
) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(6.0))
        .show(ui, |ui| {
            ui.set_min_size(egui::vec2(TILE_WIDTH, TILE_HEIGHT));
            ui.set_max_size(egui::vec2(TILE_WIDTH, TILE_HEIGHT));

            ui.vertical_centered(|ui| {
                if render_asset_preview(app, ui, ctx, asset) {
                    *jump_to_card = Some(asset.card_index);
                }

                ui.add_space(4.0);

                let file_name = asset_file_name(&asset.src);
                let display_name = truncate_middle(&file_name, 22);

                if ui
                    .selectable_label(false, egui::RichText::new(display_name).strong())
                    .clicked()
                {
                    *jump_to_card = Some(asset.card_index);
                }

                ui.label(
                    egui::RichText::new(format!(
                        "{} · {}",
                        asset.kind.label(),
                        format_card_label(asset.card_index, &asset.card_term)
                    ))
                    .small()
                    .weak(),
                );

                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if ui.small_button("Open").clicked() {
                        *jump_to_card = Some(asset.card_index);
                    }

                    if ui.small_button("Delete Ref").clicked() {
                        *delete_asset = Some((asset.card_index, asset.media_index));
                    }
                });
            });
        });
}

fn render_asset_preview(
    app: &MFlashStudioApp,
    ui: &mut egui::Ui,
    ctx: &egui::Context,
    asset: &AssetRow,
) -> bool {
    let thumb_size = egui::vec2(THUMB_SIZE, THUMB_SIZE);

    if asset.kind.can_thumbnail() {
        if let Some(texture) = media::load_texture(ctx, &app.path, &asset.src) {
            let response = ui.add(
                egui::ImageButton::new((texture.id(), thumb_size))
                    .rounding(egui::Rounding::same(2.0)),
            );

            response
                .on_hover_text("Open this card in Visual Editor")
                .clicked()
        } else {
            render_asset_icon_button(ui, asset, "Preview unavailable. Open this card anyway.")
        }
    } else {
        render_asset_icon_button(ui, asset, "Open this card in Visual Editor")
    }
}

fn render_asset_icon_button(ui: &mut egui::Ui, asset: &AssetRow, tooltip: &str) -> bool {
    let response = ui.add_sized(
        [THUMB_SIZE, THUMB_SIZE],
        egui::Button::new(egui::RichText::new(asset.kind.icon()).size(28.0).strong()),
    );

    response.on_hover_text(tooltip).clicked()
}

fn collect_assets(app: &MFlashStudioApp) -> Vec<AssetRow> {
    let Some(deck) = &app.deck else {
        return Vec::new();
    };

    let mut assets = Vec::new();

    for (card_index, card) in deck.cards.iter().enumerate() {
        let card_term = card.term.as_deref().unwrap_or("(Untitled)").to_string();

        for (media_index, media) in card.media.iter().enumerate() {
            let src = media.src.clone();
            let kind = classify_asset(&src);

            assets.push(AssetRow {
                card_index,
                media_index,
                src,
                kind,
                card_term: card_term.clone(),
            });
        }
    }

    assets
}

fn summarize_assets(assets: &[AssetRow]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();

    for asset in assets {
        *counts.entry(asset.kind.label()).or_insert(0) += 1;
    }

    counts
}

fn remove_asset_reference(
    app: &mut MFlashStudioApp,
    card_index: usize,
    media_index: usize,
    ctx: &egui::Context,
) {
    let Some(deck) = &app.deck else {
        return;
    };

    if card_index >= deck.cards.len() {
        return;
    }

    if media_index >= deck.cards[card_index].media.len() {
        return;
    }

    app.push_snapshot();

    let Some(deck) = &mut app.deck else {
        return;
    };

    if card_index >= deck.cards.len() {
        return;
    }

    if media_index >= deck.cards[card_index].media.len() {
        return;
    }

    deck.cards[card_index].media.remove(media_index);

    if app.selected_index == card_index {
        app.load_image(ctx);
    }
}

fn asset_file_name(src: &str) -> String {
    Path::new(src)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(src)
        .to_string()
}

fn classify_asset(src: &str) -> AssetKind {
    let extension = Path::new(src)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "png" | "jpg" | "jpeg" | "webp" | "bmp" | "tiff" | "avif" => AssetKind::Image,
        "gif" => AssetKind::Gif,
        "svg" => AssetKind::Svg,
        "mp3" | "wav" | "ogg" | "flac" | "m4a" | "aac" => AssetKind::Audio,
        "mp4" | "webm" | "mov" | "mkv" | "avi" => AssetKind::Video,
        "ttf" | "otf" | "woff" | "woff2" => AssetKind::Font,
        "json" | "toml" | "yaml" | "yml" | "xml" => AssetKind::Data,
        "" => AssetKind::File,
        other => AssetKind::Other(other.to_string()),
    }
}

fn format_card_label(card_index: usize, card_term: &str) -> String {
    format!("{}: {}", card_index + 1, truncate_middle(card_term, 18))
}

fn truncate_middle(input: &str, max_chars: usize) -> String {
    let char_count = input.chars().count();

    if char_count <= max_chars {
        return input.to_string();
    }

    if max_chars <= 3 {
        return "...".to_string();
    }

    let left_count = (max_chars - 3) / 2;
    let right_count = max_chars - 3 - left_count;

    let left: String = input.chars().take(left_count).collect();
    let right: String = input
        .chars()
        .rev()
        .take(right_count)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    format!("{}...{}", left, right)
}
